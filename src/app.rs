use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use ratatui::style::Color;

use crate::animation::fireworks::FireworkManager;
use crate::animation::screen_flash::FlashManager;
use crate::data::chests::loot_table_for;
use crate::data::items::get_item;
use crate::data::relics::{self, relic_stat_totals};
use crate::data::skills::all_skills;
use crate::data::upgrades::all_upgrades;
use crate::game::chest::{ChestState, ChestType};
use crate::game::item::{ItemInstance, Rarity};
use crate::game::progression::xp_for_level;
use crate::game::save;
use crate::game::state::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveTab {
    Skills,
    Upgrades,
    Relics,
    Inventory,
    Stats,
}

impl ActiveTab {
    pub const ALL: [ActiveTab; 5] = [
        ActiveTab::Skills,
        ActiveTab::Upgrades,
        ActiveTab::Relics,
        ActiveTab::Inventory,
        ActiveTab::Stats,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ActiveTab::Skills => "Skills",
            ActiveTab::Upgrades => "Upgrades",
            ActiveTab::Relics => "Relics",
            ActiveTab::Inventory => "Inventory",
            ActiveTab::Stats => "Stats",
        }
    }
}

pub struct App {
    pub state: GameState,
    pub active_tab: ActiveTab,
    pub tab_scroll: usize,
    pub show_help: bool,
    pub rng: SmallRng,
    pub auto_save_counter: u32,
    pub message_log: Vec<(String, u32)>, // (message, ticks_remaining)
    pub float_texts: Vec<FloatText>,
    pub fireworks: FireworkManager,
    pub flashes: FlashManager,
    pub screen_w: u16,
    pub screen_h: u16,
    // Skill trigger counters
    pub non_rare_streak: u32,
    pub chests_since_jackpot: u32,
    pub chests_since_xp_surge: u32,
    pub idle_income_ticks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatDir {
    Up,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct FloatText {
    pub text: String,
    pub color: Color,
    pub ticks_remaining: u32,
    pub total_ticks: u32,
    pub x_offset: i16,
    pub dir: FloatDir,
}

impl App {
    pub fn new() -> Self {
        let mut state = save::load_game().unwrap_or_default();

        // Migrate old saves: convert old key/auto_opener upgrades to skills
        let old_iron = state.upgrades.get_level("iron_key") >= 1;
        let old_silver = state.upgrades.get_level("silver_key") >= 1;
        let old_gold = state.upgrades.get_level("gold_key") >= 1;
        let old_auto = state.upgrades.get_level("auto_opener") >= 1;

        // Only migrate if skill_tree is empty (fresh load of old save)
        if state.skill_tree.learned.is_empty() && state.player.level > 1 {
            state
                .skill_tree
                .migrate_from_old_save(state.player.level, old_iron, old_silver, old_gold, old_auto);
        }

        Self {
            state,
            active_tab: ActiveTab::Skills,
            tab_scroll: 0,
            show_help: false,
            rng: SmallRng::from_os_rng(),
            auto_save_counter: 0,
            message_log: Vec::new(),
            float_texts: Vec::new(),
            fireworks: FireworkManager::default(),
            flashes: FlashManager::default(),
            screen_w: 80,
            screen_h: 24,
            non_rare_streak: 0,
            chests_since_jackpot: 0,
            chests_since_xp_surge: 0,
            idle_income_ticks: 0,
        }
    }

    pub fn on_tick(&mut self) {
        // Track terminal size for firework positioning
        if let Ok((w, h)) = crossterm::terminal::size() {
            self.screen_w = w;
            self.screen_h = h;
        }

        // Auto-save every ~30 seconds (900 ticks at 30/sec)
        self.auto_save_counter += 1;
        if self.auto_save_counter >= 900 {
            self.auto_save_counter = 0;
            self.save_game();
        }

        // Tick chest progress
        self.state.chest_progress.tick();

        // Auto-opener: skill tree version or legacy upgrade
        let has_auto_skill = self.state.skill_tree.has_skill("auto_opener");
        let has_auto_upgrade = self.state.player.auto_speed > 0.0;
        if (has_auto_skill || has_auto_upgrade)
            && self.state.chest_progress.state == ChestState::Idle
        {
            self.start_opening();
        }

        // Auto-collect after reveal
        let quick_collect = self.state.skill_tree.has_skill("quick_collect");
        if self.state.chest_progress.state == ChestState::Revealing {
            // Quick Collect: auto-collect after 30 ticks (~1 second)
            if quick_collect && self.state.chest_progress.reveal_ticks > 30 {
                self.collect_and_reset();
            }
            // Legacy auto-collect or auto opener skill: after 60 ticks
            else if (has_auto_upgrade || has_auto_skill)
                && self.state.chest_progress.reveal_ticks > 60
            {
                self.collect_and_reset();
            }
        }

        // Idle Income: earn 1 GP per second (30 ticks) while chest is idle
        if self.state.skill_tree.has_skill("idle_income")
            && self.state.chest_progress.state == ChestState::Idle
        {
            self.idle_income_ticks += 1;
            if self.idle_income_ticks >= 30 {
                self.idle_income_ticks = 0;
                self.state.player.gp += 1;
                self.state.stats.total_gp_earned += 1;
            }
        } else {
            self.idle_income_ticks = 0;
        }

        // Tick messages
        self.message_log.retain_mut(|m| {
            m.1 = m.1.saturating_sub(1);
            m.1 > 0
        });

        // Tick float texts
        self.float_texts.retain_mut(|f| {
            f.ticks_remaining = f.ticks_remaining.saturating_sub(1);
            f.ticks_remaining > 0
        });

        // Tick animations
        self.fireworks.tick();
        self.flashes.tick();

        // Check if chest just finished opening -> roll loot
        if self.state.chest_progress.state == ChestState::Revealing
            && self.state.chest_progress.reveal_ticks == 1
        {
            self.roll_loot();
        }
    }

    /// Returns true if the app should quit
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        // Global keys
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
                return false;
            }
            _ => {}
        }

        if self.show_help {
            // Any key closes help
            self.show_help = false;
            return false;
        }

        match key.code {
            // Chest interaction
            KeyCode::Char(' ') | KeyCode::Enter => match self.state.chest_progress.state {
                ChestState::Idle => self.start_opening(),
                ChestState::Opening => {} // can't skip
                ChestState::Revealing | ChestState::Complete => self.collect_and_reset(),
            },

            // Tab switching
            KeyCode::Tab => {
                let idx = ActiveTab::ALL
                    .iter()
                    .position(|&t| t == self.active_tab)
                    .unwrap_or(0);
                self.active_tab = ActiveTab::ALL[(idx + 1) % ActiveTab::ALL.len()];
                self.tab_scroll = 0;
            }
            KeyCode::BackTab => {
                let idx = ActiveTab::ALL
                    .iter()
                    .position(|&t| t == self.active_tab)
                    .unwrap_or(0);
                self.active_tab = ActiveTab::ALL
                    [(idx + ActiveTab::ALL.len() - 1) % ActiveTab::ALL.len()];
                self.tab_scroll = 0;
            }

            // Chest type selection
            KeyCode::Char(c @ '1'..='7') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < ChestType::ALL.len() {
                    let ct = ChestType::ALL[idx];
                    if self.state.unlocked_chests.contains(&ct) {
                        self.state.current_chest_type = ct;
                        self.add_message(format!("Selected {} chest", ct.name()));
                    }
                }
            }

            // Tab-specific controls
            KeyCode::Up => {
                self.tab_scroll = self.tab_scroll.saturating_sub(1);
            }
            KeyCode::Down => {
                self.tab_scroll += 1;
            }

            // Buy upgrade / Learn skill
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.active_tab == ActiveTab::Upgrades {
                    self.try_buy_upgrade();
                } else if self.active_tab == ActiveTab::Skills {
                    self.try_learn_skill();
                }
            }

            // Equip/unequip relic
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if self.active_tab == ActiveTab::Relics {
                    self.toggle_relic();
                }
            }

            _ => {}
        }

        false
    }

    fn start_opening(&mut self) {
        if self.state.chest_progress.state != ChestState::Idle {
            return;
        }

        // Determine effective speed for auto opener skill
        let speed = if self.state.skill_tree.has_skill("auto_opener")
            && self.state.player.auto_speed == 0.0
        {
            // Auto opener skill: 50% speed unless Perpetual Motion learned
            let base = self.state.player.speed;
            if self.state.skill_tree.has_skill("perpetual_motion") {
                base * 1.0
            } else {
                base * 0.5
            }
        } else {
            self.state.player.speed
        };

        self.state
            .chest_progress
            .start_opening(self.state.current_chest_type, speed);
    }

    fn roll_loot(&mut self) {
        let table = loot_table_for(self.state.current_chest_type);
        let weighted = table.weighted_entries(self.state.player.luck);
        let total_weight: f64 = weighted.iter().map(|(_, w)| w).sum();
        let mut roll: f64 = self.rng.random::<f64>() * total_weight;

        let mut chosen_idx = 0;
        for (idx, weight) in &weighted {
            roll -= weight;
            if roll <= 0.0 {
                chosen_idx = *idx;
                break;
            }
        }

        let entry = &table.entries[chosen_idx];
        let Some(item_def) = get_item(entry.item_id) else {
            return;
        };

        let mut item_rarity = item_def.rarity;

        // Skill: Pity Timer - force Rare+ after 10 non-rare streak
        if self.state.skill_tree.has_skill("pity_timer") && self.non_rare_streak >= 10 {
            if matches!(item_rarity, Rarity::Common | Rarity::Uncommon) {
                item_rarity = Rarity::Rare;
            }
        }

        // Skill: Jackpot - every 50th chest is Epic+
        self.chests_since_jackpot += 1;
        if self.state.skill_tree.has_skill("jackpot") && self.chests_since_jackpot >= 50 {
            self.chests_since_jackpot = 0;
            if matches!(
                item_rarity,
                Rarity::Common | Rarity::Uncommon | Rarity::Rare
            ) {
                item_rarity = Rarity::Epic;
            }
        }

        // Track pity counter
        if matches!(item_rarity, Rarity::Common | Rarity::Uncommon) {
            self.non_rare_streak += 1;
        } else {
            self.non_rare_streak = 0;
        }

        // XP Surge counter
        self.chests_since_xp_surge += 1;

        // Crit calculation
        let is_crit = self.rng.random::<f64>() < self.state.player.crit_chance;

        // Skill: Overcharge - crit multiplier 3x instead of 2.5x
        let base_crit_mult = if self.state.skill_tree.has_skill("overcharge") {
            3.0
        } else {
            2.5
        };

        let mut crit_mult = if is_crit { base_crit_mult } else { 1.0 };

        // Skill: Crit Cascade - 25% chance to trigger another crit (compound)
        if is_crit && self.state.skill_tree.has_skill("crit_cascade") {
            while self.rng.random::<f64>() < 0.25 {
                crit_mult *= base_crit_mult;
            }
        }

        if is_crit {
            self.state.stats.crits_rolled += 1;
        }

        // Skill: Midas Touch - +100% base GP on items
        let gp_base = if self.state.skill_tree.has_skill("midas_touch") {
            item_def.base_gp * 2
        } else {
            item_def.base_gp
        };

        let mut gp_value = (gp_base as f64
            * item_rarity.gp_multiplier()
            * self.state.player.gp_multiplier
            * crit_mult) as u64;

        // Skill: Legendary Aura - legendary items give 3x XP
        let xp_rarity_mult = if item_rarity == Rarity::Legendary
            && self.state.skill_tree.has_skill("legendary_aura")
        {
            item_rarity.xp_multiplier() * 3.0
        } else {
            item_rarity.xp_multiplier()
        };

        let mut xp_value = (item_def.base_xp as f64
            * xp_rarity_mult
            * self.state.player.xp_multiplier
            * crit_mult) as u64;

        // Skill: XP Surge - every 5th chest gives 5x XP
        if self.state.skill_tree.has_skill("xp_surge") && self.chests_since_xp_surge >= 5 {
            self.chests_since_xp_surge = 0;
            xp_value *= 5;
        }

        // Skill: Double or Nothing - 20% double, 10% nothing
        if self.state.skill_tree.has_skill("double_or_nothing") {
            let don_roll = self.rng.random::<f64>();
            if don_roll < 0.2 {
                gp_value *= 2;
                xp_value *= 2;
            } else if don_roll < 0.3 {
                gp_value = 0;
                xp_value = 0;
            }
        }

        // Skill: Golden Touch - 15% chance to double final GP
        if self.state.skill_tree.has_skill("golden_touch") && self.rng.random::<f64>() < 0.15 {
            gp_value *= 2;
        }

        let instance = ItemInstance {
            id: item_def.id.to_string(),
            name: item_def.name.to_string(),
            rarity: item_rarity,
            gp_value,
            xp_value,
            is_crit,
        };

        // Update stats
        self.state.stats.chests_opened += 1;
        self.state.stats.items_found += 1;
        self.state.stats.total_gp_earned += gp_value;
        self.state.stats.total_xp_earned += xp_value;
        if gp_value > self.state.stats.highest_single_gp {
            self.state.stats.highest_single_gp = gp_value;
        }
        match item_rarity {
            Rarity::Rare => self.state.stats.rares_found += 1,
            Rarity::Epic => self.state.stats.epics_found += 1,
            Rarity::Legendary => self.state.stats.legendaries_found += 1,
            _ => {}
        }

        // Award GP and XP
        self.state.player.gp += gp_value;
        self.award_xp(xp_value);

        // Check relic drop
        self.try_relic_drop(item_rarity);

        // Skill: Recycler - auto-sell Common items for GP instead of adding to inventory
        let recycled = self.state.skill_tree.has_skill("recycler")
            && item_rarity == Rarity::Common;

        // Float texts: item name floats up, GP flies left, XP flies right
        // Item name — floats up
        self.float_texts.push(FloatText {
            text: item_def.name.to_string(),
            color: item_rarity.color(),
            ticks_remaining: 55,
            total_ticks: 55,
            x_offset: 0,
            dir: FloatDir::Up,
        });
        // GP value — flies to the left
        let gp_text = if is_crit {
            format!("+{} GP CRIT!", gp_value)
        } else {
            format!("+{} GP", gp_value)
        };
        self.float_texts.push(FloatText {
            text: gp_text,
            color: Color::Yellow,
            ticks_remaining: 50,
            total_ticks: 50,
            x_offset: 0,
            dir: FloatDir::Left,
        });
        // XP value — flies to the right
        self.float_texts.push(FloatText {
            text: format!("+{} XP", xp_value),
            color: Color::Cyan,
            ticks_remaining: 45,
            total_ticks: 45,
            x_offset: 0,
            dir: FloatDir::Right,
        });

        // Fireworks scaled by rarity — centered near the chest art
        self.spawn_rarity_fireworks(item_rarity);

        // Store in chest progress for display
        self.state.chest_progress.last_item = Some(instance.clone());

        // Add to inventory (unless recycled)
        if !recycled {
            self.state.inventory.add(instance);
        }

        // Skill: Multi-Drop - 10% chance for a second item
        if self.state.skill_tree.has_skill("multi_drop") && self.rng.random::<f64>() < 0.10 {
            self.roll_bonus_item();
        }

        // Skill: Scavenger - 5% chance for a bonus Common item
        if self.state.skill_tree.has_skill("scavenger") && self.rng.random::<f64>() < 0.05 {
            self.roll_scavenger_item();
        }
    }

    fn roll_bonus_item(&mut self) {
        // Roll a second item from the same loot table
        let table = loot_table_for(self.state.current_chest_type);
        let weighted = table.weighted_entries(self.state.player.luck);
        let total_weight: f64 = weighted.iter().map(|(_, w)| w).sum();
        let mut roll: f64 = self.rng.random::<f64>() * total_weight;

        let mut chosen_idx = 0;
        for (idx, weight) in &weighted {
            roll -= weight;
            if roll <= 0.0 {
                chosen_idx = *idx;
                break;
            }
        }

        let entry = &table.entries[chosen_idx];
        if let Some(item_def) = get_item(entry.item_id) {
            let gp_value = (item_def.base_gp as f64
                * item_def.rarity.gp_multiplier()
                * self.state.player.gp_multiplier) as u64;
            let xp_value = (item_def.base_xp as f64
                * item_def.rarity.xp_multiplier()
                * self.state.player.xp_multiplier) as u64;

            self.state.player.gp += gp_value;
            self.state.stats.total_gp_earned += gp_value;
            self.state.stats.items_found += 1;
            self.award_xp(xp_value);

            let instance = ItemInstance {
                id: item_def.id.to_string(),
                name: item_def.name.to_string(),
                rarity: item_def.rarity,
                gp_value,
                xp_value,
                is_crit: false,
            };

            self.add_message(format!("Multi-Drop: bonus {}!", item_def.name));
            self.state.inventory.add(instance);
        }
    }

    fn roll_scavenger_item(&mut self) {
        // Roll a random Common item from current chest's loot table
        let table = loot_table_for(self.state.current_chest_type);
        // Find first Common entry
        for entry in &table.entries {
            if let Some(item_def) = get_item(entry.item_id) {
                if item_def.rarity == Rarity::Common {
                    let gp_value = (item_def.base_gp as f64
                        * item_def.rarity.gp_multiplier()
                        * self.state.player.gp_multiplier) as u64;
                    let xp_value = (item_def.base_xp as f64
                        * item_def.rarity.xp_multiplier()
                        * self.state.player.xp_multiplier) as u64;

                    self.state.player.gp += gp_value;
                    self.state.stats.total_gp_earned += gp_value;
                    self.state.stats.items_found += 1;
                    self.award_xp(xp_value);

                    let instance = ItemInstance {
                        id: item_def.id.to_string(),
                        name: item_def.name.to_string(),
                        rarity: item_def.rarity,
                        gp_value,
                        xp_value,
                        is_crit: false,
                    };

                    // If recycler is active, don't add common to inventory
                    if !self.state.skill_tree.has_skill("recycler") {
                        self.state.inventory.add(instance);
                    }
                    self.add_message(format!("Scavenger: found {}!", item_def.name));
                    return;
                }
            }
        }
    }

    fn spawn_rarity_fireworks(&mut self, rarity: Rarity) {
        let game_w = (self.screen_w as f64 * 0.45).max(20.0);
        let cx = game_w / 2.0;
        let cy = self.screen_h as f64 * 0.3;
        let spread_x = game_w * 0.7;
        let spread_y = self.screen_h as f64 * 0.6;
        match rarity {
            Rarity::Common => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x, spread_y,
                    &[Color::Gray, Color::DarkGray, Color::White],
                    8, 18, 2,
                );
            }
            Rarity::Uncommon => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x, spread_y,
                    &[Color::Green, Color::LightGreen, Color::Rgb(100, 255, 100)],
                    12, 25, 3,
                );
            }
            Rarity::Rare => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x * 1.2, spread_y * 1.1,
                    &[Color::Blue, Color::Cyan, Color::LightBlue, Color::Rgb(80, 150, 255)],
                    18, 35, 5,
                );
            }
            Rarity::Epic => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x * 1.4, spread_y * 1.3,
                    &[Color::Magenta, Color::LightMagenta, Color::Rgb(200, 100, 255), Color::White, Color::Rgb(255, 100, 200)],
                    25, 45, 7,
                );
            }
            Rarity::Legendary => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x * 1.6, spread_y * 1.5,
                    &[Color::Yellow, Color::LightYellow, Color::Rgb(255, 200, 50), Color::White, Color::Rgb(255, 150, 0), Color::Rgb(255, 100, 0)],
                    35, 60, 10,
                );
                self.flashes.spawn(Color::Yellow, 10);
            }
        }
    }

    fn award_xp(&mut self, xp: u64) {
        self.state.player.xp += xp;
        while self.state.player.xp >= self.state.player.xp_to_next {
            self.state.player.xp -= self.state.player.xp_to_next;
            self.state.player.level += 1;
            self.state.player.xp_to_next = xp_for_level(self.state.player.level);
            // Grant 1 skill point per level
            self.state.skill_tree.skill_points += 1;
            self.add_message(format!(
                "LEVEL UP! Level {} (+1 Skill Point)",
                self.state.player.level
            ));
            self.check_chest_unlocks();
        }
    }

    fn check_chest_unlocks(&mut self) {
        for ct in ChestType::ALL {
            if !self.state.unlocked_chests.contains(&ct) {
                let unlocked = match ct {
                    ChestType::Wooden => true,
                    // Skill tree keys
                    ChestType::Iron => self.state.skill_tree.has_skill("iron_key"),
                    ChestType::Silver => self.state.skill_tree.has_skill("silver_key"),
                    ChestType::Gold => self.state.skill_tree.has_skill("gold_key"),
                    // Crystal/Shadow/Void require Void Attune skill
                    ChestType::Crystal | ChestType::Shadow | ChestType::Void => {
                        self.state.skill_tree.has_skill("void_attune")
                    }
                };
                if unlocked {
                    self.state.unlocked_chests.push(ct);
                    self.add_message(format!("{} chests unlocked!", ct.name()));
                }
            }
        }
    }

    fn try_relic_drop(&mut self, item_rarity: Rarity) {
        // Relics only drop from rarer items in higher chests
        let chest_tier = self.state.current_chest_type.index();
        if chest_tier < 3 {
            return; // Gold+ only
        }

        let base_drop_chance = match item_rarity {
            Rarity::Epic => 0.08,
            Rarity::Legendary => 0.20,
            _ => return,
        };

        // Skill: Relic Hunter - double relic drop chance
        let drop_chance = if self.state.skill_tree.has_skill("relic_hunter") {
            base_drop_chance * 2.0
        } else {
            base_drop_chance
        };

        if self.rng.random::<f64>() < drop_chance {
            // Pick a relic we don't own yet
            let available: Vec<_> = relics::all_relics()
                .into_iter()
                .filter(|r| r.min_chest_tier <= chest_tier && !self.state.relics.owns(r.id))
                .collect();

            if let Some(relic) = available.first() {
                self.state.relics.add_relic(relic.id.to_string());
                self.add_message(format!("RELIC FOUND: {}!", relic.name));
                self.float_texts.push(FloatText {
                    text: format!("NEW RELIC: {}", relic.name),
                    color: relic.rarity.color(),
                    ticks_remaining: 90,
                    total_ticks: 90,
                    x_offset: 0,
                    dir: FloatDir::Up,
                });
            }
        }
    }

    fn collect_and_reset(&mut self) {
        self.state.chest_progress.collect();

        // Skill: Chain Opener - immediately start the next chest
        if self.state.skill_tree.has_skill("chain_opener") {
            self.start_opening();
        }
    }

    fn try_learn_skill(&mut self) {
        let skills = all_skills();
        if self.tab_scroll >= skills.len() {
            return;
        }
        let skill = &skills[self.tab_scroll];
        if self.state.skill_tree.learn(skill.id) {
            self.add_message(format!("Learned: {}!", skill.name));
            self.recalculate_player_stats();
            self.check_chest_unlocks();
        } else if self.state.skill_tree.has_skill(skill.id) {
            self.add_message("Already learned!".to_string());
        } else if self.state.skill_tree.skill_points == 0 {
            self.add_message("No skill points!".to_string());
        } else {
            self.add_message("Prerequisites not met!".to_string());
        }
    }

    fn try_buy_upgrade(&mut self) {
        let upgrades = all_upgrades();
        if self.tab_scroll >= upgrades.len() {
            return;
        }
        let upg = &upgrades[self.tab_scroll];
        let current_level = self.state.upgrades.get_level(upg.id);
        if current_level >= upg.max_level {
            self.add_message("Already maxed!".to_string());
            return;
        }
        let cost = upg.cost_at_level(current_level);
        if self.state.player.gp < cost {
            self.add_message(format!("Need {} GP!", cost));
            return;
        }
        self.state.player.gp -= cost;
        self.state.upgrades.increment(upg.id);
        self.add_message(format!(
            "Upgraded {} to level {}",
            upg.name,
            current_level + 1
        ));
        self.recalculate_player_stats();
        self.check_chest_unlocks();
    }

    fn toggle_relic(&mut self) {
        let owned = &self.state.relics.owned;
        if self.tab_scroll >= owned.len() {
            return;
        }
        let id = owned[self.tab_scroll].clone();
        self.state.relics.toggle_equip(&id);
        self.recalculate_player_stats();
    }

    pub fn recalculate_player_stats(&mut self) {
        let upgrades = all_upgrades();
        let mut up_luck = 0.0f64;
        let mut up_speed = 0.0f64;
        let mut up_gp = 0.0f64;
        let mut up_xp = 0.0f64;
        let mut up_crit = 0.0f64;
        let mut up_auto = 0.0f64;

        for upg in &upgrades {
            let lvl = self.state.upgrades.get_level(upg.id) as f64;
            match upg.id {
                "swift_hands" => up_speed += lvl * 0.10,
                "nimble_fingers" => up_speed += lvl * 0.05,
                "auto_opener" => up_auto += lvl * 0.5,
                "lucky_charm" => up_luck += lvl * 1.0,
                "four_leaf" => up_luck += lvl * 2.0,
                "critical_eye" => up_crit += lvl * 0.02,
                "gold_touch" => up_gp += lvl * 0.10,
                "xp_boost" => up_xp += lvl * 0.10,
                "treasure_sense" => up_gp += lvl * 0.20,
                _ => {}
            }
        }

        // Skill tree stat bonuses
        let tree = &self.state.skill_tree;
        if tree.has_skill("lucky_charm") {
            up_luck += 3.0;
        }
        if tree.has_skill("four_leaf") {
            up_luck += 5.0;
        }
        if tree.has_skill("treasure_sense") {
            up_gp += 0.50;
        }
        if tree.has_skill("swift_hands") {
            up_speed += 0.30;
        }
        if tree.has_skill("nimble_fingers") {
            up_speed += 0.20;
        }
        if tree.has_skill("critical_eye") {
            up_crit += 0.05;
        }

        let (r_luck, r_speed, r_gp, r_xp, r_crit) =
            relic_stat_totals(&self.state.relics.equipped);

        self.state.player.recalculate_stats(
            up_luck, up_speed, up_gp, up_xp, up_crit, up_auto, r_luck, r_speed, r_gp, r_xp,
            r_crit,
        );
    }

    fn add_message(&mut self, msg: String) {
        self.message_log.push((msg, 90)); // 3 seconds
    }

    pub fn save_game(&self) {
        save::save_game(&self.state);
    }
}
