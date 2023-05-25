use crate::models::combat::Model;
use crate::models::combat::WeaponSet;
use crate::models::combat::Weapons;
use crate::models::dice::d6;
use crate::Stats;

pub fn trajann_valoris_weapons() -> WeaponSet {
    let ranged_weapons = vec![Weapons {
        name: "Watcher's Axe".to_string(),
        weapon_range: 24,
        attacks: 2,
        hit: 2,
        strength: 5,
        armour_pen: 2,
        damage: 3,
        tags: { Some(vec!["Assault".to_string()]) },
    }];

    let melee_weapons = vec![Weapons {
        name: "Watcher's Axe".to_string(),
        weapon_range: 0,
        attacks: 6,
        hit: 2,
        strength: 10,
        armour_pen: 2,
        damage: 3,
        tags: Some(vec![]),
    }];
    WeaponSet {
        ranged_weapons,
        melee_weapons,
    }
}

pub fn trajann_valoris() -> Model {
    let name = "Trajann Valoris".to_string();
    let points = 160;
    let quantity = 1;
    let stats = Stats {
        movement: 6,
        toughness: 6,
        save: 2,
        invunlerable: 4,
        wounds: 7,
        leadership: 5,
        objective_control: 2,
    };

    let ranged_weapon = None;

    let melee_weapon = None;

    let tags: Vec<String> = {
        vec![
            "Infantry".to_string(),
            "Character".to_string(),
            "Epic Hero".to_string(),
            "Imperium".to_string(),
            "Trajann Valoris".to_string(),
        ]
    };
    Model::new(&name, stats, ranged_weapon, melee_weapon, tags)
}

pub fn allarus_custodians_weapons() -> WeaponSet {
    let ranged_weapons = vec![
        Weapons {
            name: "Balistus Grenade Launcher".to_string(),
            weapon_range: 18,
            attacks: d6(),
            hit: 2,
            strength: 4,
            armour_pen: 1,
            damage: 1,
            tags: { Some(vec!["Blast".to_string()]) },
        },
        Weapons {
            name: "Castellan axe".to_string(),
            weapon_range: 24,
            attacks: 2,
            hit: 2,
            strength: 5,
            armour_pen: 2,
            damage: 3,
            tags: { Some(vec!["Assault".to_string()]) },
        },
        Weapons {
            name: "Guardian Spear".to_string(),
            weapon_range: 24,
            attacks: 2,
            hit: 2,
            strength: 5,
            armour_pen: 2,
            damage: 3,
            tags: { Some(vec!["Assault".to_string()]) },
        },
    ];

    let melee_weapons = vec![
        Weapons {
            name: "Castellan axe".to_string(),
            weapon_range: 0,
            attacks: 4,
            hit: 3,
            strength: 9,
            armour_pen: 1,
            damage: 3,
            tags: { Some(vec![]) },
        },
        Weapons {
            name: "Guardian Spear".to_string(),
            weapon_range: 0,
            attacks: 5,
            hit: 2,
            strength: 7,
            armour_pen: 2,
            damage: 2,
            tags: { Some(vec![]) },
        },
        Weapons {
            name: "Misericordia".to_string(),
            weapon_range: 0,
            attacks: 5,
            hit: 2,
            strength: 5,
            armour_pen: 2,
            damage: 1,
            tags: { Some(vec![]) },
        },
    ];

    WeaponSet {
        ranged_weapons,
        melee_weapons,
    }
}

pub fn allarus_custodians() -> Model {
    let name = "Allarus Custodians".to_string();
    let points = 140;
    let quantity = 2;

    let stats = Stats {
        movement: 5,
        toughness: 7,
        save: 2,
        invunlerable: 4,
        wounds: 4,
        leadership: 6,
        objective_control: 2,
    };

    let ranged_weapon = None;

    let melee_weapon = None;

    let tags: Vec<String> = {
        vec![
            "Infantry".to_string(),
            "Genestealers".to_string(),
            "Great Devourer".to_string(),
        ]
    };
    Model::new(&name, stats, ranged_weapon, melee_weapon, tags)
}

pub fn prosecutors_weapons() -> WeaponSet {
    let ranged_weapons = vec![Weapons {
        name: "Boltgun".to_string(),
        weapon_range: 24,
        attacks: 1,
        hit: 3,
        strength: 4,
        armour_pen: 0,
        damage: 1,
        tags: { Some(vec!["Rapid Fire".to_string()]) },
    }];

    let melee_weapons = vec![Weapons {
        name: "Close Combat Weapon".to_string(),
        weapon_range: 0,
        attacks: 2,
        hit: 3,
        strength: 4,
        armour_pen: 0,
        damage: 1,
        tags: { Some(vec![]) },
    }];
    WeaponSet {
        ranged_weapons,
        melee_weapons,
    }
}

pub fn prosecutors() -> Model {
    let name = "Prosecutors".to_string();
    let points = 140;
    let quantity = 3;

    let stats = Stats {
        movement: 6,
        toughness: 3,
        save: 3,
        invunlerable: 7,
        wounds: 1,
        leadership: 6,
        objective_control: 2,
    };

    let ranged_weapon = None;

    let melee_weapon = None;

    let tags: Vec<String> = {
        vec![
            "Infantry".to_string(),
            "Battleline".to_string(),
            "Imperium".to_string(),
            "Anathma Psykana".to_string(),
            "Prosecutors".to_string(),
        ]
    };
    Model::new(&name, stats, ranged_weapon, melee_weapon, tags)
}
