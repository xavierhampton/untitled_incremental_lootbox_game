use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use ratatui::style::Color;

use crate::animation::fireworks::FireworkManager;
use crate::animation::screen_flash::FlashManager;
use crate::data::chests::loot_table_for;
use crate::data::items::get_item;
use crate::data::rebirth_skills::all_rebirth_skills;
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
    Rebirth,
}

impl ActiveTab {
    pub const ALL: [ActiveTab; 6] = [
        ActiveTab::Skills,
        ActiveTab::Upgrades,
        ActiveTab::Relics,
        ActiveTab::Inventory,
        ActiveTab::Stats,
        ActiveTab::Rebirth,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ActiveTab::Skills => "Skills",
            ActiveTab::Upgrades => "Upgrades",
            ActiveTab::Relics => "Relics",
            ActiveTab::Inventory => "Inventory",
            ActiveTab::Stats => "Stats",
            ActiveTab::Rebirth => "Rebirth",
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
    // New skill counters
    pub consecutive_chests: u32,      // momentum skill
    pub empty_streak: u32,            // gambler_spirit skill
    pub chaos_buff_ticks: u32,        // chaos_surge skill
    pub chaos_buff_type: Option<u8>,  // 0=GP, 1=XP, 2=Speed
    pub items_sold_count: u64,        // alchemy tracking
    pub catalyst_stacks: f64,         // catalyst_brew (resets on rebirth)
    pub rebirth_confirm: bool,        // R key double-press confirmation
    pub rare_streak_count: u32,       // lucky_streak tracking
    pub auto_opener_paused: bool,     // pause auto opener with 'P'
    pub show_chest_menu: bool,        // show chest selection popup
    pub show_settings: bool,          // show settings menu
    pub settings_selected: usize,     // selected setting option
    pub show_dev_options: bool,       // show dev options submenu
    pub dev_option_selected: usize,   // selected dev option
    // Settings
    pub setting_show_animations: bool,   // show fireworks/flashes
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

        let mut app = Self {
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
            consecutive_chests: 0,
            empty_streak: 0,
            chaos_buff_ticks: 0,
            chaos_buff_type: None,
            items_sold_count: 0,
            catalyst_stacks: 0.0,
            rebirth_confirm: false,
            rare_streak_count: 0,
            auto_opener_paused: false,
            show_chest_menu: false,
            show_settings: false,
            settings_selected: 0,
            show_dev_options: false,
            dev_option_selected: 0,
            setting_show_animations: true,
        };

        // Apply rebirth bonuses on load
        app.apply_rebirth_bonuses();
        app.recalculate_player_stats();

        app
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

        // Auto-opener: skill tree version or legacy upgrade (only if not paused)
        let has_auto_skill = self.state.skill_tree.has_skill("auto_opener");
        let has_auto_upgrade = self.state.upgrades.get_level("auto_opener") > 0;
        if !self.auto_opener_paused
            && (has_auto_skill || has_auto_upgrade)
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

        // Idle Income: earn 2 GP per second while chest is idle
        if self.state.skill_tree.has_skill("idle_income")
            && self.state.chest_progress.state == ChestState::Idle
        {
            self.idle_income_ticks += 1;
            let idle_rate = if self.state.skill_tree.has_skill("temporal_mastery") {
                4 // 30 / 8 = earn 8x per second (every 4 ticks, gp = 2)
            } else {
                15 // 2 GP per second (every 15 ticks)
            };
            if self.idle_income_ticks >= idle_rate {
                self.idle_income_ticks = 0;
                let gp = 2;
                self.state.player.gp += gp;
                self.state.stats.total_gp_earned += gp;
                self.state.rebirth.gp_earned_this_run += gp;
            }
        } else {
            self.idle_income_ticks = 0;
        }

        // Tick chaos buff
        if self.chaos_buff_ticks > 0 {
            self.chaos_buff_ticks -= 1;
            if self.chaos_buff_ticks == 0 {
                self.chaos_buff_type = None;
            }
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
            KeyCode::Esc => {
                // Toggle settings menu
                self.show_settings = !self.show_settings;
                // Reset to main settings when closing
                if !self.show_settings {
                    self.show_dev_options = false;
                    self.settings_selected = 0;
                }
                return false;
            }
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

        if self.show_settings {
            // Handle settings menu input
            return self.handle_settings_input(key);
        }

        // Block tab panel input when chest menu is open
        if self.show_chest_menu {
            return self.handle_chest_menu_input(key);
        }

        match key.code {
            // Chest interaction / Open menu / Pause/Resume (when auto opener active)
            KeyCode::Char(' ') => {
                // Check if auto opener is active
                let has_auto = self.state.skill_tree.has_skill("auto_opener")
                    || self.state.upgrades.get_level("auto_opener") > 0;

                if has_auto {
                    // With auto opener: Space always toggles pause/resume
                    self.auto_opener_paused = !self.auto_opener_paused;
                    let msg = if self.auto_opener_paused {
                        "Auto opener PAUSED"
                    } else {
                        "Auto opener RESUMED"
                    };
                    self.add_message(msg.to_string());
                } else {
                    // Without auto opener: Space opens/collects chest
                    match self.state.chest_progress.state {
                        ChestState::Idle => {
                            self.start_opening();
                        }
                        ChestState::Opening => {} // can't interact while opening
                        ChestState::Revealing | ChestState::Complete => {
                            self.collect_and_reset();
                        }
                    }
                }
            }

            // Tab switching
            KeyCode::Tab | KeyCode::Right => {
                let idx = ActiveTab::ALL
                    .iter()
                    .position(|&t| t == self.active_tab)
                    .unwrap_or(0);
                self.active_tab = ActiveTab::ALL[(idx + 1) % ActiveTab::ALL.len()];
                self.tab_scroll = 0;
                self.rebirth_confirm = false;
            }
            KeyCode::BackTab | KeyCode::Left => {
                let idx = ActiveTab::ALL
                    .iter()
                    .position(|&t| t == self.active_tab)
                    .unwrap_or(0);
                self.active_tab = ActiveTab::ALL
                    [(idx + ActiveTab::ALL.len() - 1) % ActiveTab::ALL.len()];
                self.tab_scroll = 0;
                self.rebirth_confirm = false;
            }

            // Toggle chest menu with 'C'
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.show_chest_menu = !self.show_chest_menu;
            }

            // Tab-specific controls
            KeyCode::Up => {
                self.tab_scroll = self.tab_scroll.saturating_sub(1);
            }
            KeyCode::Down => {
                let max = match self.active_tab {
                    ActiveTab::Skills => all_skills().len().saturating_sub(1),
                    ActiveTab::Upgrades => all_upgrades().len().saturating_sub(1),
                    ActiveTab::Relics => self.state.relics.owned.len().saturating_sub(1),
                    ActiveTab::Inventory => self.state.inventory.items.len().saturating_sub(1), // Display count matches items vec
                    ActiveTab::Rebirth => all_rebirth_skills().len().saturating_sub(1),
                    ActiveTab::Stats => 100, // stats just scrolls freely
                };
                if self.tab_scroll < max {
                    self.tab_scroll += 1;
                }
            }

            // Buy upgrade / Learn skill / Equip relic (E key)
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if self.active_tab == ActiveTab::Upgrades {
                    self.try_buy_upgrade();
                } else if self.active_tab == ActiveTab::Skills {
                    self.try_learn_skill();
                } else if self.active_tab == ActiveTab::Rebirth {
                    self.try_learn_rebirth_skill();
                } else if self.active_tab == ActiveTab::Relics {
                    self.toggle_relic();
                }
            }

            // Unequip all relics
            KeyCode::Char('u') | KeyCode::Char('U') => {
                if self.active_tab == ActiveTab::Relics {
                    self.unequip_all_relics();
                }
            }

            // Rebirth
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if self.active_tab == ActiveTab::Rebirth {
                    self.try_rebirth();
                }
            }

            // Sell item (Alchemy)
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if self.active_tab == ActiveTab::Inventory {
                    self.try_sell_item();
                }
            }

            // Sell all items (Alchemy)
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.active_tab == ActiveTab::Inventory {
                    self.try_sell_all_items();
                }
            }

            _ => {}
        }

        false
    }

    fn handle_chest_menu_input(&mut self, key: KeyEvent) -> bool {
        match key.code {
            // Close chest menu with Space, C, or Esc
            KeyCode::Char(' ') | KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Esc => {
                self.show_chest_menu = false;
            }
            // Select chest with number keys 1-7
            KeyCode::Char(c @ '1'..='7') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < ChestType::ALL.len() {
                    let ct = ChestType::ALL[idx];
                    if self.state.unlocked_chests.contains(&ct) {
                        self.state.current_chest_type = ct;
                        self.show_chest_menu = false;
                        self.start_opening();
                        self.add_message(format!("Opening {} chest...", ct.name()));
                    } else {
                        self.add_message(format!("{} chest is locked!", ct.name()));
                    }
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

        // Use player speed for all opening (manual and auto)
        let mut speed = self.state.player.speed;

        // Momentum skill: each consecutive chest opens 3% faster (max 60%)
        if self.state.skill_tree.has_skill("momentum") {
            let bonus = (self.consecutive_chests as f64 * 0.03).min(0.60);
            speed *= 1.0 + bonus;
        }

        // Chaos buff: speed
        if self.chaos_buff_type == Some(2) && self.chaos_buff_ticks > 0 {
            speed *= 1.5;
        }

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

        // Determine if pandemonium doubles chaos chances
        let chaos_mult = if self.state.skill_tree.has_skill("pandemonium") {
            2.0
        } else {
            1.0
        };

        // Skill: Wild Magic - 5% (or 10% with pandemonium) chance to upgrade rarity
        if self.state.skill_tree.has_skill("wild_magic") {
            let chance = 0.05 * chaos_mult;
            if self.rng.random::<f64>() < chance {
                item_rarity = match item_rarity {
                    Rarity::Common => Rarity::Uncommon,
                    Rarity::Uncommon => Rarity::Rare,
                    Rarity::Rare => Rarity::Epic,
                    Rarity::Epic => Rarity::Legendary,
                    Rarity::Legendary => Rarity::Mythic,
                    Rarity::Mythic => Rarity::Mythic,
                };
            }
        }

        // Skill: Gambler's Spirit - after 3 empty/common streaks, force Rare+
        if self.state.skill_tree.has_skill("gambler_spirit") {
            if matches!(item_rarity, Rarity::Common) {
                self.empty_streak += 1;
            } else {
                self.empty_streak = 0;
            }
            let threshold = if self.state.skill_tree.has_skill("pandemonium") { 2 } else { 3 };
            if self.empty_streak >= threshold {
                if matches!(item_rarity, Rarity::Common | Rarity::Uncommon) {
                    item_rarity = Rarity::Rare;
                }
                self.empty_streak = 0;
            }
        }

        // Skill: Pity Timer - force Rare+ after streak
        let pity_threshold = if self.state.skill_tree.has_skill("golden_rain") {
            5
        } else {
            10
        };
        if self.state.skill_tree.has_skill("pity_timer") && self.non_rare_streak >= pity_threshold {
            if matches!(item_rarity, Rarity::Common | Rarity::Uncommon) {
                item_rarity = Rarity::Rare;
            }
        }

        // Skill: Jackpot - every 40th chest is Epic+
        self.chests_since_jackpot += 1;
        if self.state.skill_tree.has_skill("jackpot") && self.chests_since_jackpot >= 40 {
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

        // Track lucky_streak (consecutive rare+ finds)
        if matches!(item_rarity, Rarity::Rare | Rarity::Epic | Rarity::Legendary) {
            self.rare_streak_count += 1;
        } else {
            self.rare_streak_count = 0;
        }

        // XP Surge counter
        self.chests_since_xp_surge += 1;
        // Momentum counter
        self.consecutive_chests += 1;

        // Crit calculation
        let is_crit = self.rng.random::<f64>() < self.state.player.crit_chance;

        // Base crit multiplier from upgrades/skills
        let crit_power_bonus = self.state.upgrades.get_level("crit_power") as f64 * 0.2;
        let executioner_bonus = self.state.upgrades.get_level("executioners_edge") as f64 * 0.3;

        // Skill: Overcharge - crit multiplier 3.5x instead of 2.5x
        let base_crit_mult = if self.state.skill_tree.has_skill("overcharge") {
            3.5
        } else {
            2.5
        } + crit_power_bonus + executioner_bonus;

        // Skill: Grand Mastery capstone - crit mult +3x
        let base_crit_mult = if self.state.skill_tree.has_skill("grand_mastery") {
            base_crit_mult + 3.0
        } else {
            base_crit_mult
        };

        // Relic crit mult bonus
        let relic_totals = relic_stat_totals(&self.state.relics.equipped);
        let base_crit_mult = base_crit_mult + relic_totals.crit_mult;

        let mut crit_mult = if is_crit {
            // Skill: Chaos Crit - random 1x-5x instead of fixed
            if self.state.skill_tree.has_skill("chaos_crit") {
                let random_mult = 1.0 + self.rng.random::<f64>() * 4.0;
                random_mult + crit_power_bonus + executioner_bonus + relic_totals.crit_mult
            } else {
                base_crit_mult
            }
        } else {
            1.0
        };

        // Skill: Crit Cascade - 30% chance to trigger another crit (compound)
        if is_crit && self.state.skill_tree.has_skill("crit_cascade") {
            while self.rng.random::<f64>() < 0.30 {
                crit_mult *= base_crit_mult;
            }
        }

        if is_crit {
            self.state.stats.crits_rolled += 1;
        }

        // Skill: Midas Touch - +150% base GP on items
        let gp_base = if self.state.skill_tree.has_skill("midas_touch") {
            (item_def.base_gp as f64 * 2.5) as u64
        } else {
            item_def.base_gp
        };

        // Skill: Material Insight - +20% GP from Uncommon+ items
        let material_bonus = if self.state.skill_tree.has_skill("material_insight")
            && !matches!(item_rarity, Rarity::Common)
        {
            1.2
        } else {
            1.0
        };

        let mut gp_value = (gp_base as f64
            * item_rarity.gp_multiplier()
            * self.state.player.gp_multiplier
            * crit_mult
            * material_bonus) as u64;

        // Skill: Fortune Favors - +10% GP
        if self.state.skill_tree.has_skill("fortune_favors") {
            gp_value = (gp_value as f64 * 1.10) as u64;
        }

        // Skill: Lucky Streak - consecutive rare+ finds give +15% GP each
        if self.state.skill_tree.has_skill("lucky_streak") && self.rare_streak_count > 1 {
            let bonus = 1.0 + (self.rare_streak_count - 1) as f64 * 0.15;
            gp_value = (gp_value as f64 * bonus) as u64;
        }

        // Skill: Golden Rain capstone - +40% GP
        if self.state.skill_tree.has_skill("golden_rain") {
            gp_value = (gp_value as f64 * 1.40) as u64;
        }

        // Skill: Grand Mastery capstone - +40% all multipliers
        if self.state.skill_tree.has_skill("grand_mastery") {
            gp_value = (gp_value as f64 * 1.40) as u64;
        }

        // Upgrade: Legendary Focus - +10% legendary GP per level
        if item_rarity == Rarity::Legendary {
            let legendary_focus_lvl = self.state.upgrades.get_level("legendary_focus") as f64;
            if legendary_focus_lvl > 0.0 {
                gp_value = (gp_value as f64 * (1.0 + legendary_focus_lvl * 0.10)) as u64;
            }
        }

        // Skill: Legendary Aura - legendary items give 4x XP
        let xp_rarity_mult = if item_rarity == Rarity::Legendary
            && self.state.skill_tree.has_skill("legendary_aura")
        {
            item_rarity.xp_multiplier() * 4.0
        } else {
            item_rarity.xp_multiplier()
        };

        let mut xp_value = (item_def.base_xp as f64
            * xp_rarity_mult
            * self.state.player.xp_multiplier
            * crit_mult) as u64;

        // Skill: Deep Knowledge - +150% XP
        if self.state.skill_tree.has_skill("deep_knowledge") {
            xp_value = (xp_value as f64 * 2.5) as u64;
        }

        // Skill: Precision Strike - crits give +75% XP
        if is_crit && self.state.skill_tree.has_skill("precision_strike") {
            xp_value = (xp_value as f64 * 1.75) as u64;
        }

        // Skill: Grand Mastery capstone - +40% all multipliers (XP too)
        if self.state.skill_tree.has_skill("grand_mastery") {
            xp_value = (xp_value as f64 * 1.40) as u64;
        }

        // Skill: XP Surge - every 4th chest gives 6x XP
        if self.state.skill_tree.has_skill("xp_surge") && self.chests_since_xp_surge >= 4 {
            self.chests_since_xp_surge = 0;
            xp_value *= 6;
        }

        // Skill: Entropy - loot values vary ±30%
        if self.state.skill_tree.has_skill("entropy") {
            let variance = 0.7 + self.rng.random::<f64>() * 0.6; // 0.7 to 1.3
            gp_value = (gp_value as f64 * variance) as u64;
            xp_value = (xp_value as f64 * variance) as u64;
        }

        // Skill: Double or Nothing - 25% double, 8% nothing
        if self.state.skill_tree.has_skill("double_or_nothing") {
            let don_roll = self.rng.random::<f64>();
            if don_roll < 0.25 {
                gp_value *= 2;
                xp_value *= 2;
            } else if don_roll < 0.33 {
                gp_value = 0;
                xp_value = 0;
            }
        }

        // Skill: Golden Touch - 20% chance to double final GP
        if self.state.skill_tree.has_skill("golden_touch") && self.rng.random::<f64>() < 0.20 {
            gp_value *= 2;
        }

        // Skill: Reality Tear - 1% (2% with pandemonium) chance for 20x GP
        if self.state.skill_tree.has_skill("reality_tear") {
            let chance = 0.01 * chaos_mult;
            if self.rng.random::<f64>() < chance {
                gp_value *= 20;
                self.add_message("REALITY TEAR! 20x GP!".to_string());
                if self.setting_show_animations {
                    self.flashes.spawn(Color::Rgb(200, 50, 50), 15);
                }
            }
        }

        // Skill: Singularity capstone - 3% (6% pandemonium) chance to triple all
        if self.state.skill_tree.has_skill("singularity") {
            let chance = 0.03 * chaos_mult;
            if self.rng.random::<f64>() < chance {
                gp_value *= 3;
                xp_value *= 3;
                self.add_message("SINGULARITY! Triple loot!".to_string());
                if self.setting_show_animations {
                    self.flashes.spawn(Color::Magenta, 12);
                }
            }
        }

        // Chaos Surge - random buff
        if self.state.skill_tree.has_skill("chaos_surge") {
            let buff_type = (self.rng.random::<f64>() * 3.0) as u8;
            self.chaos_buff_type = Some(buff_type);
            self.chaos_buff_ticks = 300; // 10 seconds at 30fps

            match buff_type {
                0 => gp_value = (gp_value as f64 * 1.5) as u64,
                1 => xp_value = (xp_value as f64 * 1.5) as u64,
                2 => {} // speed buff applied in start_opening
                _ => {}
            }
        }

        // Catalyst brew bonus
        if self.state.skill_tree.has_skill("catalyst_brew") && self.catalyst_stacks > 0.0 {
            gp_value = (gp_value as f64 * (1.0 + self.catalyst_stacks / 100.0)) as u64;
        }

        // Magnum Opus capstone - +50% GP
        if self.state.skill_tree.has_skill("magnum_opus") {
            gp_value = (gp_value as f64 * 1.5) as u64;
        }

        // Ensure at least 1 GP
        gp_value = gp_value.max(1);

        let instance = ItemInstance {
            id: item_def.id.to_string(),
            name: item_def.name.to_string(),
            rarity: item_rarity,
            gp_value,
            xp_value,
            is_crit,
            count: 1,
        };

        // Update stats
        self.state.stats.chests_opened += 1;
        self.state.stats.items_found += 1;
        self.state.stats.total_gp_earned += gp_value;
        self.state.stats.total_xp_earned += xp_value;
        self.state.rebirth.gp_earned_this_run += gp_value;
        if gp_value > self.state.stats.highest_single_gp {
            self.state.stats.highest_single_gp = gp_value;
        }
        match item_rarity {
            Rarity::Rare => self.state.stats.rares_found += 1,
            Rarity::Epic => self.state.stats.epics_found += 1,
            Rarity::Legendary => self.state.stats.legendaries_found += 1,
            Rarity::Mythic => self.state.stats.mythics_found += 1,
            _ => {}
        }

        // Update highest level ever
        if self.state.player.level > self.state.rebirth.highest_level_ever {
            self.state.rebirth.highest_level_ever = self.state.player.level;
        }

        // Award GP and XP
        self.state.player.gp += gp_value;
        self.award_xp(xp_value);

        // Check relic drop
        self.try_relic_drop(item_rarity);

        // Skill: Recycler - auto-sell Common items for GP instead of adding to inventory
        let recycled = self.state.skill_tree.has_skill("recycler")
            && item_rarity == Rarity::Common;

        // Deep Salvage: recycled items give 3x GP
        if recycled && self.state.skill_tree.has_skill("deep_salvage") {
            let bonus = gp_value * 2; // already got gp_value, give 2x more
            self.state.player.gp += bonus;
            self.state.stats.total_gp_earned += bonus;
            self.state.rebirth.gp_earned_this_run += bonus;
        }

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

        // Relic: multi-drop bonus from relics
        let relic_multi_drop = relic_totals.multi_drop;

        // Upgrade: bonus_loot - +3% multi-drop per level
        let bonus_loot_chance = self.state.upgrades.get_level("bonus_loot") as f64 * 0.03;

        // Upgrade: lucky_find - +5% multi-drop per level
        let lucky_find_chance = self.state.upgrades.get_level("lucky_find") as f64 * 0.05;

        // Skill: Multi-Drop - 15% chance for a second item
        let base_multi = if self.state.skill_tree.has_skill("multi_drop") { 0.15 } else { 0.0 };
        let total_multi = base_multi + relic_multi_drop + bonus_loot_chance + lucky_find_chance;

        // World Explorer capstone: +50% all drop rates
        let total_multi = if self.state.skill_tree.has_skill("world_explorer") {
            total_multi * 1.5
        } else {
            total_multi
        };

        if total_multi > 0.0 && self.rng.random::<f64>() < total_multi {
            self.roll_bonus_item();
        }

        // Skill: Scavenger - 5% chance for a bonus Common item
        let scav_chance = if self.state.skill_tree.has_skill("scavenger") { 0.05 } else { 0.0 };
        let scav_chance = if self.state.skill_tree.has_skill("world_explorer") {
            scav_chance * 1.5
        } else {
            scav_chance
        };
        if scav_chance > 0.0 && self.rng.random::<f64>() < scav_chance {
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
            self.state.rebirth.gp_earned_this_run += gp_value;
            self.award_xp(xp_value);

            let instance = ItemInstance {
                id: item_def.id.to_string(),
                name: item_def.name.to_string(),
                rarity: item_def.rarity,
                gp_value,
                xp_value,
                is_crit: false,
                count: 1,
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
                    self.state.rebirth.gp_earned_this_run += gp_value;
                    self.award_xp(xp_value);

                    let instance = ItemInstance {
                        id: item_def.id.to_string(),
                        name: item_def.name.to_string(),
                        rarity: item_def.rarity,
                        gp_value,
                        xp_value,
                        is_crit: false,
                        count: 1,
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
        if !self.setting_show_animations {
            return;
        }

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
                if self.setting_show_animations {
                    self.flashes.spawn(Color::Yellow, 10);
                }
            }
            Rarity::Mythic => {
                self.fireworks.spawn_burst_wide(
                    cx, cy, spread_x * 2.0, spread_y * 2.0,
                    &[Color::Rgb(255, 50, 50), Color::Rgb(255, 100, 100), Color::White, Color::Rgb(255, 0, 0), Color::Rgb(200, 0, 0), Color::Rgb(255, 150, 150)],
                    50, 90, 15,
                );
                if self.setting_show_animations {
                    self.flashes.spawn(Color::Rgb(255, 50, 50), 15);
                }
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
            // Update highest level
            if self.state.player.level > self.state.rebirth.highest_level_ever {
                self.state.rebirth.highest_level_ever = self.state.player.level;
            }
            self.check_chest_unlocks();
        }
    }

    fn check_chest_unlocks(&mut self) {
        for ct in ChestType::ALL {
            if !self.state.unlocked_chests.contains(&ct) {
                let level_req = ct.required_level();
                let meets_level = self.state.player.level >= level_req;

                let has_key = match ct {
                    ChestType::Wooden => true,
                    ChestType::Iron => self.state.upgrades.get_level("iron_key") > 0,
                    ChestType::Silver => self.state.upgrades.get_level("silver_key") > 0,
                    ChestType::Gold => self.state.upgrades.get_level("gold_key") > 0,
                    ChestType::Crystal => self.state.upgrades.get_level("crystal_key") > 0,
                    ChestType::Shadow => self.state.upgrades.get_level("shadow_key") > 0,
                    ChestType::Void => self.state.upgrades.get_level("void_key") > 0,
                };

                if meets_level && has_key {
                    self.state.unlocked_chests.push(ct);
                    self.add_message(format!("{} chests unlocked!", ct.name()));
                }
            }
        }
    }

    fn try_relic_drop(&mut self, item_rarity: Rarity) {
        // Relics drop from higher chests; Uncommon relics from Silver+, Rare from Gold+, etc.
        let chest_tier = self.state.current_chest_type.index();

        // Uncommon relics can drop from Silver+ (tier 2+) on any item rarity
        // Rare+ relics from Gold+ (tier 3+) on Epic/Legendary items
        let base_drop_chance = match item_rarity {
            Rarity::Uncommon if chest_tier >= 2 => 0.05,
            Rarity::Rare if chest_tier >= 2 => 0.08,
            Rarity::Epic if chest_tier >= 3 => 0.08,
            Rarity::Legendary if chest_tier >= 3 => 0.20,
            _ => return,
        };

        // Skill: Relic Hunter - double relic drop chance
        let mut drop_chance = if self.state.skill_tree.has_skill("relic_hunter") {
            base_drop_chance * 2.0
        } else {
            base_drop_chance
        };

        // Deep Salvage: +10% relic drop
        if self.state.skill_tree.has_skill("deep_salvage") {
            drop_chance += 0.10;
        }

        // Relic: relic_drop_pct bonus
        let relic_totals = relic_stat_totals(&self.state.relics.equipped);
        drop_chance *= 1.0 + relic_totals.relic_drop_pct / 100.0;

        // Upgrade: relic_magnet - +5% per level
        let magnet_lvl = self.state.upgrades.get_level("relic_magnet") as f64;
        drop_chance += magnet_lvl * 0.05;

        // Upgrade: treasure_hunter - +8% per level
        let hunter_lvl = self.state.upgrades.get_level("treasure_hunter") as f64;
        drop_chance += hunter_lvl * 0.08;

        // Upgrade: artifact_sense - +10% per level
        let artifact_lvl = self.state.upgrades.get_level("artifact_sense") as f64;
        drop_chance += artifact_lvl * 0.10;

        // World Explorer capstone: +50% all drop rates
        if self.state.skill_tree.has_skill("world_explorer") {
            drop_chance *= 1.5;
        }

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

        // Check level requirement for key upgrades
        if upg.category == crate::data::upgrades::UpgradeCategory::Unlock {
            let req_level = match upg.id {
                "iron_key" => 5,
                "silver_key" => 10,
                "gold_key" => 20,
                "crystal_key" => 30,
                "shadow_key" => 40,
                "void_key" => 50,
                _ => 1,
            };
            if self.state.player.level < req_level {
                self.add_message(format!("Need level {}!", req_level));
                return;
            }
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
        use crate::game::item::Rarity;

        // Rebuild the same display order as the UI
        let owned = &self.state.relics.owned;
        let rarities_order = [
            Rarity::Mythic,
            Rarity::Legendary,
            Rarity::Epic,
            Rarity::Rare,
            Rarity::Uncommon,
        ];

        let mut organized_relics: Vec<(usize, String)> = Vec::new();
        for (original_idx, relic_id) in owned.iter().enumerate() {
            if let Some(_relic_def) = relics::get_relic(relic_id) {
                organized_relics.push((original_idx, relic_id.clone()));
            }
        }

        // Sort by rarity tier (same as UI)
        organized_relics.sort_by_key(|(_, relic_id)| {
            relics::get_relic(relic_id)
                .and_then(|def| rarities_order.iter().position(|r| *r == def.rarity))
                .unwrap_or(99)
        });

        if self.tab_scroll >= organized_relics.len() {
            return;
        }

        let id = organized_relics[self.tab_scroll].1.clone();

        // Dynamic max equipped based on upgrades and rebirth skills
        let extra_slots = self.state.upgrades.get_level("deep_pockets") as usize;
        let rebirth_slot = if self.state.rebirth.has_rebirth_skill("rb_relic_slot") { 1 } else { 0 };
        let world_explorer_slot = if self.state.skill_tree.has_skill("world_explorer") { 1 } else { 0 };
        let max_equipped = 3 + extra_slots + rebirth_slot + world_explorer_slot;

        if self.state.relics.is_equipped(&id) {
            self.state.relics.unequip(&id);
        } else if self.state.relics.equipped.len() < max_equipped {
            if self.state.relics.owns(&id) && !self.state.relics.is_equipped(&id) {
                self.state.relics.equipped.push(id);
            }
        } else {
            self.add_message("All relic slots full!".to_string());
            return;
        }
        self.recalculate_player_stats();
    }

    fn unequip_all_relics(&mut self) {
        if self.state.relics.equipped.is_empty() {
            self.add_message("No relics equipped!".to_string());
            return;
        }

        let count = self.state.relics.equipped.len();
        self.state.relics.equipped.clear();
        self.recalculate_player_stats();
        self.add_message(format!("Unequipped {} relics", count));
    }

    fn try_learn_rebirth_skill(&mut self) {
        let skills = all_rebirth_skills();
        if self.tab_scroll >= skills.len() {
            return;
        }
        let skill = &skills[self.tab_scroll];
        if self.state.rebirth.has_rebirth_skill(skill.id) {
            self.add_message("Already learned!".to_string());
        } else if self.state.rebirth.learn_rebirth_skill(skill.id) {
            self.add_message(format!("Learned rebirth skill: {}!", skill.name));
            self.apply_rebirth_bonuses();
            self.recalculate_player_stats();
        } else if self.state.rebirth.essence < skill.essence_cost {
            self.add_message(format!("Need {} Essence!", skill.essence_cost));
        } else {
            self.add_message("Prerequisites not met!".to_string());
        }
    }

    fn try_rebirth(&mut self) {
        let level = self.state.player.level;
        if !self.state.rebirth.can_rebirth(level) {
            self.add_message(format!(
                "Need level {} to rebirth! (currently {})",
                self.state.rebirth.min_level_for_rebirth(),
                level
            ));
            self.rebirth_confirm = false;
            return;
        }

        if !self.rebirth_confirm {
            self.rebirth_confirm = true;
            let essence = self.state.rebirth.calculate_essence_reward(
                level,
                self.state.rebirth.gp_earned_this_run,
            );
            self.add_message(format!(
                "Press [R] again to rebirth for {} Essence!",
                essence
            ));
            return;
        }

        // Perform rebirth
        self.perform_rebirth();
        self.rebirth_confirm = false;
    }

    fn perform_rebirth(&mut self) {
        let level = self.state.player.level;
        let gp_this_run = self.state.rebirth.gp_earned_this_run;

        // Calculate essence reward
        let essence = self.state.rebirth.calculate_essence_reward(level, gp_this_run);

        // Award essence
        self.state.rebirth.essence += essence;
        self.state.rebirth.total_essence_earned += essence;
        self.state.rebirth.rebirth_count += 1;

        // Update highest level
        if level > self.state.rebirth.highest_level_ever {
            self.state.rebirth.highest_level_ever = level;
        }

        // Reset run-specific state
        self.state.rebirth.gp_earned_this_run = 0;

        // Reset player to defaults
        self.state.player = crate::game::player::Player::default();

        // Reset inventory, upgrades, skills, chests, chest progress
        self.state.inventory = crate::game::inventory::Inventory::default();
        self.state.upgrades = crate::game::upgrade::UpgradeState::default();
        self.state.skill_tree = crate::game::skill_tree::SkillTreeState::default();
        self.state.chest_progress = crate::game::chest::ChestProgress::default();
        self.state.current_chest_type = ChestType::Wooden;
        self.state.unlocked_chests = vec![ChestType::Wooden];

        // Reset app counters
        self.non_rare_streak = 0;
        self.chests_since_jackpot = 0;
        self.chests_since_xp_surge = 0;
        self.idle_income_ticks = 0;
        self.consecutive_chests = 0;
        self.empty_streak = 0;
        self.chaos_buff_ticks = 0;
        self.chaos_buff_type = None;
        self.items_sold_count = 0;
        self.catalyst_stacks = 0.0;
        self.rare_streak_count = 0;
        self.tab_scroll = 0;

        // Apply rebirth bonuses
        self.apply_rebirth_bonuses();
        self.recalculate_player_stats();
        self.check_chest_unlocks();

        // Flash + message
        if self.setting_show_animations {
            self.flashes.spawn(Color::Rgb(150, 100, 255), 20);
        }
        self.add_message(format!(
            "REBIRTH #{} complete! +{} Essence",
            self.state.rebirth.rebirth_count, essence
        ));

        // Save immediately
        self.save_game();
    }

    fn apply_rebirth_bonuses(&mut self) {
        let rb = &self.state.rebirth;

        // Tier 1
        if rb.has_rebirth_skill("rb_lucky_start") {
            self.state.player.base_luck += 5.0;
        }
        if rb.has_rebirth_skill("rb_swift_start") {
            self.state.player.base_speed += 0.3;
        }
        if rb.has_rebirth_skill("rb_gp_boost") {
            self.state.player.base_gp_multiplier += 0.10;
        }
        if rb.has_rebirth_skill("rb_xp_boost") {
            self.state.player.base_xp_multiplier += 0.10;
        }
        if rb.has_rebirth_skill("rb_crit_boost") {
            self.state.player.base_crit_chance += 0.03;
        }
        if rb.has_rebirth_skill("rb_head_start") && self.state.player.level < 3 {
            self.state.player.level = 3;
            self.state.player.xp_to_next = xp_for_level(3);
            self.state.skill_tree.skill_points += 2; // levels 2 and 3
        }
        if rb.has_rebirth_skill("rb_starting_gp") {
            self.state.player.gp += 500;
        }

        // Tier 2
        if rb.has_rebirth_skill("rb_luck_mastery") {
            self.state.player.base_luck += 10.0;
        }
        if rb.has_rebirth_skill("rb_speed_mastery") {
            self.state.player.base_speed += 0.5;
        }
        if rb.has_rebirth_skill("rb_gp_mastery") {
            self.state.player.base_gp_multiplier += 0.25;
        }
        if rb.has_rebirth_skill("rb_xp_mastery") {
            self.state.player.base_xp_multiplier += 0.25;
        }
        if rb.has_rebirth_skill("rb_crit_mastery") {
            self.state.player.base_crit_chance += 0.05;
        }
        // rb_relic_slot - handled in toggle_relic max_equipped calculation
        if rb.has_rebirth_skill("rb_chest_unlock") {
            // Start with Iron and Silver unlocked
            if !self.state.unlocked_chests.contains(&ChestType::Iron) {
                self.state.unlocked_chests.push(ChestType::Iron);
            }
            if !self.state.unlocked_chests.contains(&ChestType::Silver) {
                self.state.unlocked_chests.push(ChestType::Silver);
            }
        }
        // rb_essence_boost - handled in calculate_essence_reward

        // Tier 3
        if rb.has_rebirth_skill("rb_all_luck") {
            self.state.player.base_luck += 20.0;
            self.state.player.base_gp_multiplier += 0.10;
        }
        if rb.has_rebirth_skill("rb_all_speed") {
            self.state.player.base_speed += 1.0;
            // auto opener at start is implicitly handled - player gets speed which makes auto_opener work
        }
        if rb.has_rebirth_skill("rb_all_wealth") {
            self.state.player.base_gp_multiplier += 0.50;
            self.state.player.base_xp_multiplier += 0.50;
        }
        if rb.has_rebirth_skill("rb_all_crit") {
            self.state.player.base_crit_chance += 0.10;
        }
        if rb.has_rebirth_skill("rb_ascension") {
            // All base stats +50%
            self.state.player.base_luck *= 1.5;
            self.state.player.base_speed *= 1.5;
            self.state.player.base_gp_multiplier *= 1.5;
            self.state.player.base_xp_multiplier *= 1.5;
            self.state.player.base_crit_chance = (self.state.player.base_crit_chance * 1.5).min(0.75);
            // Start with Gold chests
            if !self.state.unlocked_chests.contains(&ChestType::Gold) {
                self.state.unlocked_chests.push(ChestType::Gold);
            }
        }
    }

    fn try_sell_item(&mut self) {
        if !self.state.skill_tree.has_skill("transmute_basics") {
            self.add_message("Learn Transmute Basics to sell items!".to_string());
            return;
        }

        if self.tab_scroll >= self.state.inventory.items.len() {
            return;
        }

        // Clone item data before borrowing self mutably
        let item = self.state.inventory.items[self.tab_scroll].clone();

        // Calculate sell value for one item
        let mut sell_pct = if self.state.skill_tree.has_skill("philosophers_stone") {
            if matches!(item.rarity, Rarity::Rare | Rarity::Epic | Rarity::Legendary) {
                1.5
            } else {
                1.0
            }
        } else if self.state.skill_tree.has_skill("gold_synthesis") {
            0.75
        } else {
            0.50
        };

        // Magnum Opus doubles sell bonuses
        if self.state.skill_tree.has_skill("magnum_opus") {
            sell_pct *= 2.0;
        }

        let mut sell_gp = (item.gp_value as f64 * sell_pct) as u64;

        // Elixir of Fortune: 10% chance to double sell GP
        if self.state.skill_tree.has_skill("elixir_of_fortune") {
            if self.rng.random::<f64>() < 0.10 {
                sell_gp *= 2;
                self.add_message("Elixir of Fortune: double sell!".to_string());
            }
        }

        sell_gp = sell_gp.max(1);

        self.state.player.gp += sell_gp;
        self.state.stats.total_gp_earned += sell_gp;
        self.state.rebirth.gp_earned_this_run += sell_gp;
        self.items_sold_count += 1;

        // Catalyst Brew: +1% GP per sell stack
        if self.state.skill_tree.has_skill("catalyst_brew") {
            self.catalyst_stacks += 1.0;
        }

        // Essence Distill: selling grants +10% XP of GP value
        if self.state.skill_tree.has_skill("essence_distill") {
            let xp_bonus = (sell_gp as f64 * 0.10) as u64;
            self.award_xp(xp_bonus);
        }

        // Decrement count or remove item
        if item.count > 1 {
            self.state.inventory.items[self.tab_scroll].count -= 1;
            self.add_message(format!("Sold 1× {} for {} GP ({} left)", item.name, sell_gp, item.count - 1));
        } else {
            self.state.inventory.items.remove(self.tab_scroll);
            self.add_message(format!("Sold {} for {} GP", item.name, sell_gp));

            // Adjust scroll if needed
            if self.tab_scroll > 0 && self.tab_scroll >= self.state.inventory.items.len() {
                self.tab_scroll = self.state.inventory.items.len().saturating_sub(1);
            }
        }
    }

    fn try_sell_all_items(&mut self) {
        if !self.state.skill_tree.has_skill("transmute_basics") {
            self.add_message("Learn Transmute Basics to sell items!".to_string());
            return;
        }

        if self.state.inventory.items.is_empty() {
            self.add_message("No items to sell!".to_string());
            return;
        }

        let mut total_gp = 0u64;
        let mut total_sold = 0u32;

        // Calculate sell value for all items
        for item in &self.state.inventory.items {
            let mut sell_pct = if self.state.skill_tree.has_skill("philosophers_stone") {
                if matches!(item.rarity, Rarity::Rare | Rarity::Epic | Rarity::Legendary) {
                    1.5
                } else {
                    1.0
                }
            } else if self.state.skill_tree.has_skill("gold_synthesis") {
                0.75
            } else {
                0.50
            };

            if self.state.skill_tree.has_skill("magnum_opus") {
                sell_pct *= 2.0;
            }

            let sell_gp_per_item = ((item.gp_value as f64 * sell_pct) as u64).max(1);
            total_gp += sell_gp_per_item * item.count as u64;
            total_sold += item.count;
        }

        // Apply Elixir of Fortune once for the batch
        if self.state.skill_tree.has_skill("elixir_of_fortune") {
            if self.rng.random::<f64>() < 0.10 {
                total_gp *= 2;
                self.add_message("Elixir of Fortune: double sell!".to_string());
            }
        }

        self.state.player.gp += total_gp;
        self.state.stats.total_gp_earned += total_gp;
        self.state.rebirth.gp_earned_this_run += total_gp;
        self.items_sold_count += total_sold as u64;

        // Catalyst Brew: stacks based on count sold
        if self.state.skill_tree.has_skill("catalyst_brew") {
            self.catalyst_stacks += total_sold as f64;
        }

        // Essence Distill
        if self.state.skill_tree.has_skill("essence_distill") {
            let xp_bonus = (total_gp as f64 * 0.10) as u64;
            self.award_xp(xp_bonus);
        }

        self.add_message(format!("Sold {} items for {} GP", total_sold, total_gp));
        self.state.inventory.items.clear();
        self.tab_scroll = 0;
    }

    pub fn recalculate_player_stats(&mut self) {
        let upgrades = all_upgrades();
        let mut up_luck = 0.0f64;
        let mut up_speed = 0.0f64;
        let mut up_gp = 0.0f64;
        let mut up_xp = 0.0f64;
        let mut up_crit = 0.0f64;

        for upg in &upgrades {
            let lvl = self.state.upgrades.get_level(upg.id) as f64;
            match upg.id {
                "swift_hands" => up_speed += lvl * 0.10,
                "nimble_fingers" => up_speed += lvl * 0.05,
                "auto_opener" => {} // Just enables auto-opening, doesn't affect speed
                "lucky_charm" => up_luck += lvl * 1.0,
                "four_leaf" => up_luck += lvl * 2.0,
                "critical_eye" => up_crit += lvl * 0.02,
                "gold_touch" => up_gp += lvl * 0.10,
                "xp_boost" => up_xp += lvl * 0.10,
                "treasure_sense" => up_gp += lvl * 0.20,
                // Speed category
                "overdrive" => up_speed += lvl * 0.15,
                "perpetual_gear" => up_speed += lvl * 0.10,
                "quicksilver_touch" => up_speed += lvl * 0.08,
                "haste_rune" => up_speed += lvl * 0.20,
                "chrono_accelerator" => up_speed += lvl * 0.12,
                // Luck category
                "fortune_wheel" => up_luck += lvl * 3.0,
                "horseshoe" => {} // Handled in rarity upgrade logic
                "rabbits_paw" => up_luck += lvl * 4.0,
                "lucky_dice" => up_crit += lvl * 0.03,
                "stars_alignment" => up_luck += lvl * 5.0,
                // Wealth category
                "golden_magnet" => up_gp += lvl * 0.15,
                "wisdom_tome" => up_xp += lvl * 0.15,
                "alchemist_stone" => up_gp += lvl * 0.25,
                "scholars_cap" => up_xp += lvl * 0.25,
                "dragon_hoard_map" => up_gp += lvl * 0.30,
                // Mastery category
                "keen_edge" => up_crit += lvl * 0.03,
                "crit_power" => {} // Handled in roll_loot crit calculation
                "combo_counter" => {} // Handled in roll_loot
                "xp_amplifier" => up_xp += lvl * 0.20,
                "legendary_focus" => {} // Handled in roll_loot
                "executioners_edge" => {} // Handled in roll_loot crit calculation
                "precision_mastery" => up_crit += lvl * 0.04,
                "knowledge_nexus" => up_xp += lvl * 0.30,
                // Discovery category
                "relic_magnet" => {} // Handled in try_relic_drop
                "deep_pockets" => {} // Handled in toggle_relic
                "bonus_loot" => {} // Handled in roll_loot
                "chest_radar" => {} // Handled in roll_loot
                "void_sight" => {} // Handled in loot table weighting
                "treasure_hunter" => {} // Handled in try_relic_drop
                "lucky_find" => {} // Handled in roll_loot
                "artifact_sense" => {} // Handled in try_relic_drop
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
            up_speed += 0.25;
        }
        if tree.has_skill("critical_eye") {
            up_crit += 0.05;
        }

        // New skill stat bonuses
        if tree.has_skill("fortune_favors") {
            up_luck += 12.0;
        }
        if tree.has_skill("time_warp") {
            up_speed += 0.50;
        }
        if tree.has_skill("temporal_mastery") {
            up_speed += 0.75;
        }
        if tree.has_skill("precision_strike") {
            up_crit += 0.12;
        }
        if tree.has_skill("elixir_of_fortune") {
            up_luck += 5.0;
        }

        let relic_totals = relic_stat_totals(&self.state.relics.equipped);

        self.state.player.recalculate_stats(
            up_luck, up_speed, up_gp, up_xp, up_crit,
            relic_totals.luck, relic_totals.speed_pct, relic_totals.gp_pct,
            relic_totals.xp_pct, relic_totals.crit,
        );
    }

    pub fn max_equipped_relics(&self) -> usize {
        let extra_slots = self.state.upgrades.get_level("deep_pockets") as usize;
        let rebirth_slot = if self.state.rebirth.has_rebirth_skill("rb_relic_slot") { 1 } else { 0 };
        let world_explorer_slot = if self.state.skill_tree.has_skill("world_explorer") { 1 } else { 0 };
        3 + extra_slots + rebirth_slot + world_explorer_slot
    }

    fn add_message(&mut self, msg: String) {
        self.message_log.push((msg, 90)); // 3 seconds
    }

    pub fn save_game(&self) {
        save::save_game(&self.state);
    }

    fn handle_settings_input(&mut self, key: KeyEvent) -> bool {
        // If in dev options submenu, handle separately
        if self.show_dev_options {
            return self.handle_dev_options_input(key);
        }

        const NUM_SETTINGS: usize = 2; // Animations, Dev Options

        match key.code {
            KeyCode::Up => {
                self.settings_selected = self.settings_selected.saturating_sub(1);
                false
            }
            KeyCode::Down => {
                self.settings_selected = (self.settings_selected + 1).min(NUM_SETTINGS - 1);
                false
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                match self.settings_selected {
                    0 => {
                        // Toggle animations
                        self.setting_show_animations = !self.setting_show_animations;
                        let msg = if self.setting_show_animations {
                            "Animations enabled"
                        } else {
                            "Animations disabled"
                        };
                        self.add_message(msg.to_string());
                    }
                    1 => {
                        // Enter Dev Options
                        self.show_dev_options = true;
                        self.dev_option_selected = 0;
                    }
                    _ => {}
                }
                false
            }
            _ => false
        }
    }

    fn handle_dev_options_input(&mut self, key: KeyEvent) -> bool {
        const NUM_DEV_OPTIONS: usize = 4; // Reset, Unlock Chests, Max Money, Max Skills, Max Essence

        match key.code {
            KeyCode::Esc => {
                // Go back to main settings
                self.show_dev_options = false;
                self.settings_selected = 0;
                false
            }
            KeyCode::Up => {
                self.dev_option_selected = self.dev_option_selected.saturating_sub(1);
                false
            }
            KeyCode::Down => {
                self.dev_option_selected = (self.dev_option_selected + 1).min(NUM_DEV_OPTIONS);
                false
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                match self.dev_option_selected {
                    0 => {
                        // Reset game
                        self.reset_game();
                        self.add_message("Game reset!".to_string());
                    }
                    1 => {
                        // Unlock all chests
                        use crate::game::chest::ChestType;
                        for ct in ChestType::ALL {
                            if !self.state.unlocked_chests.contains(&ct) {
                                self.state.unlocked_chests.push(ct);
                            }
                        }
                        self.add_message("All chests unlocked!".to_string());
                    }
                    2 => {
                        // Max money
                        self.state.player.gp = 999_999_999;
                        self.add_message("Max GP granted!".to_string());
                    }
                    3 => {
                        // Max skills
                        self.state.skill_tree.skill_points = 9999;
                        self.add_message("Max skill points granted!".to_string());
                    }
                    4 => {
                        // Max essence
                        self.state.rebirth.essence = 999_999;
                        self.add_message("Max essence granted!".to_string());
                    }
                    _ => {}
                }
                false
            }
            _ => false
        }
    }

    fn reset_game(&mut self) {
        // Create a completely fresh game state
        self.state = GameState::default();

        // Reset app state
        self.tab_scroll = 0;
        self.active_tab = ActiveTab::Skills;
        self.non_rare_streak = 0;
        self.chests_since_jackpot = 0;
        self.chests_since_xp_surge = 0;
        self.idle_income_ticks = 0;
        self.consecutive_chests = 0;
        self.empty_streak = 0;
        self.chaos_buff_ticks = 0;
        self.chaos_buff_type = None;
        self.items_sold_count = 0;
        self.catalyst_stacks = 0.0;
        self.rebirth_confirm = false;
        self.rare_streak_count = 0;
        self.auto_opener_paused = false;
        self.show_chest_menu = false;
        self.show_settings = false;
        self.settings_selected = 0;
        self.float_texts.clear();
        self.message_log.clear();
        self.fireworks = FireworkManager::default();
        self.flashes = FlashManager::default();

        self.recalculate_player_stats();
        self.add_message("Game reset!".to_string());
        self.save_game();
    }
}
