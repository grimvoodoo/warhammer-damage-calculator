use crate::models::combat::Weapons;
use crate::models::dice::d6;
use crate::Stats;
use crate::Unit;

pub fn swarmlord() -> Unit {
    let name = "Swarmlord".to_string();
    let points = 160;
    let quantity = 1;
    let stats = Stats {
        movement: 8,
        toughness: 8,
        save: 2,
        invunlerable: 4,
        wounds: 10,
        leadership: 7,
        objective_control: 3,
    };

    let ranged_weapons = Some(vec![Weapons {
        name: "Synaptic Pulse".to_string(),
        weapon_range: 18,
        attacks: d6() + 3,
        hit: 0,
        strength: 5,
        armour_pen: 1,
        damage: 2,
        tags: { Some(vec!["Psychic".to_string(), "Torrent".to_string()]) },
    }]);

    let melee_weapons = Some(vec![Weapons {
        name: "Bone Sabres".to_string(),
        weapon_range: 0,
        attacks: 4,
        hit: 2,
        strength: 9,
        armour_pen: 2,
        damage: 3,
        tags: Some(vec!["Twin-Linked".to_string()]),
    }]);

    let tags: Option<Vec<String>> = {
        Some(vec![
            "Monster".to_string(),
            "Character".to_string(),
            "Epic Hero".to_string(),
            "Psyker".to_string(),
            "Great Devourer".to_string(),
            "Synapse".to_string(),
            "The Swarmlord".to_string(),
        ])
    };
    Unit::new(
        &name,
        points,
        stats,
        quantity,
        ranged_weapons,
        melee_weapons,
        tags,
    )
}

pub fn genestealers() -> Unit {
    let name = "Genestealers".to_string();
    let points = 140;
    let quantity = 5;

    let stats = Stats {
        movement: 8,
        toughness: 4,
        save: 5,
        invunlerable: 5,
        wounds: 2,
        leadership: 7,
        objective_control: 1,
    };

    let ranged_weapons = None;

    let melee_weapons = Some(vec![Weapons {
        name: "Genestealer Claws and Talons".to_string(),
        weapon_range: 0,
        attacks: 4,
        hit: 2,
        strength: 4,
        armour_pen: 2,
        damage: 1,
        tags: None,
    }]);

    let tags: Option<Vec<String>> = {
        Some(vec![
            "Infantry".to_string(),
            "Genestealers".to_string(),
            "Great Devourer".to_string(),
        ])
    };
    Unit::new(
        &name,
        points,
        stats,
        quantity,
        ranged_weapons,
        melee_weapons,
        tags,
    )
}
