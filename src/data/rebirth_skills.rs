#[derive(Debug, Clone)]
pub struct RebirthSkillDef {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub essence_cost: u64,
    pub tier: u32,
    pub prerequisites: &'static [&'static str],
}

pub fn all_rebirth_skills() -> &'static [RebirthSkillDef] {
    use std::sync::LazyLock;
    static SKILLS: LazyLock<Vec<RebirthSkillDef>> = LazyLock::new(|| {
        vec![
            // === Tier 1 — Foundations (7 skills, 50-100 Essence) ===
            RebirthSkillDef {
                id: "rb_lucky_start",
                name: "Lucky Start",
                description: "Start each run with +5 luck",
                essence_cost: 50,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_swift_start",
                name: "Swift Start",
                description: "Start each run with +0.3 speed",
                essence_cost: 50,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_gp_boost",
                name: "Golden Legacy",
                description: "+10% GP multiplier permanently",
                essence_cost: 75,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_xp_boost",
                name: "Wisdom Legacy",
                description: "+10% XP multiplier permanently",
                essence_cost: 75,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_crit_boost",
                name: "Sharp Instincts",
                description: "+3% crit chance permanently",
                essence_cost: 100,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_head_start",
                name: "Head Start",
                description: "Start each run at level 3",
                essence_cost: 100,
                tier: 1,
                prerequisites: &[],
            },
            RebirthSkillDef {
                id: "rb_starting_gp",
                name: "Seed Money",
                description: "Start each run with 500 GP",
                essence_cost: 60,
                tier: 1,
                prerequisites: &[],
            },
            // === Tier 2 — Mastery (8 skills, 200-400 Essence) ===
            RebirthSkillDef {
                id: "rb_luck_mastery",
                name: "Luck Mastery",
                description: "+10 luck permanently",
                essence_cost: 200,
                tier: 2,
                prerequisites: &["rb_lucky_start"],
            },
            RebirthSkillDef {
                id: "rb_speed_mastery",
                name: "Speed Mastery",
                description: "+0.5 speed permanently",
                essence_cost: 200,
                tier: 2,
                prerequisites: &["rb_swift_start"],
            },
            RebirthSkillDef {
                id: "rb_gp_mastery",
                name: "Wealth Mastery",
                description: "+25% GP multiplier permanently",
                essence_cost: 300,
                tier: 2,
                prerequisites: &["rb_gp_boost"],
            },
            RebirthSkillDef {
                id: "rb_xp_mastery",
                name: "Scholar's Mastery",
                description: "+25% XP multiplier permanently",
                essence_cost: 300,
                tier: 2,
                prerequisites: &["rb_xp_boost"],
            },
            RebirthSkillDef {
                id: "rb_crit_mastery",
                name: "Deadly Precision",
                description: "+5% crit, +0.5x crit multiplier",
                essence_cost: 350,
                tier: 2,
                prerequisites: &["rb_crit_boost"],
            },
            RebirthSkillDef {
                id: "rb_relic_slot",
                name: "Relic Affinity",
                description: "+1 relic slot permanently",
                essence_cost: 400,
                tier: 2,
                prerequisites: &["rb_lucky_start"],
            },
            RebirthSkillDef {
                id: "rb_chest_unlock",
                name: "Chest Familiarity",
                description: "Start with Iron and Silver chests unlocked",
                essence_cost: 250,
                tier: 2,
                prerequisites: &["rb_head_start"],
            },
            RebirthSkillDef {
                id: "rb_essence_boost",
                name: "Essence Siphon",
                description: "+20% essence gain from rebirth",
                essence_cost: 350,
                tier: 2,
                prerequisites: &["rb_gp_boost", "rb_xp_boost"],
            },
            // === Tier 3 — Transcendence (5 skills, 500-1500 Essence) ===
            RebirthSkillDef {
                id: "rb_all_luck",
                name: "Fortune's Blessing",
                description: "+20 luck, +10% GP permanently",
                essence_cost: 500,
                tier: 3,
                prerequisites: &["rb_luck_mastery"],
            },
            RebirthSkillDef {
                id: "rb_all_speed",
                name: "Temporal Blessing",
                description: "+1.0 speed, auto opener at start",
                essence_cost: 500,
                tier: 3,
                prerequisites: &["rb_speed_mastery"],
            },
            RebirthSkillDef {
                id: "rb_all_wealth",
                name: "Midas Blessing",
                description: "+50% GP, +50% XP permanently",
                essence_cost: 750,
                tier: 3,
                prerequisites: &["rb_gp_mastery", "rb_xp_mastery"],
            },
            RebirthSkillDef {
                id: "rb_all_crit",
                name: "Critical Blessing",
                description: "+10% crit, +1.0x crit multiplier",
                essence_cost: 750,
                tier: 3,
                prerequisites: &["rb_crit_mastery"],
            },
            RebirthSkillDef {
                id: "rb_ascension",
                name: "Ascension",
                description: "Capstone: all base stats +50%, essence +50%, start at Gold chests",
                essence_cost: 1500,
                tier: 3,
                prerequisites: &["rb_all_luck", "rb_all_speed", "rb_all_wealth", "rb_all_crit"],
            },
        ]
    });
    SKILLS.as_slice()
}

pub fn get_rebirth_skill(id: &str) -> Option<&'static RebirthSkillDef> {
    all_rebirth_skills().iter().find(|s| s.id == id)
}
