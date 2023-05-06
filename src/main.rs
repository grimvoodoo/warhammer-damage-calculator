mod models;

use models::combat::{Stats, Unit};

// Main function
fn main() {
    env_logger::init();
    // Create a hive_tyrant unit
    let hive_tyrant_stats = Stats {
        m: 9,
        ws: 2,
        bs: 2,
        s: 7,
        t: 8,
        w: 12,
        a: 5,
        ld: 10,
        sv: 2,
        inv: 4,
        ap: 2,
        dam: 3,
    };
    let hive_tyrant = Unit::new("hive_tyrant", 160, hive_tyrant_stats, 1);

    // Create another unit, for example, an enemy unit
    // Replace this with the stats and name for the enemy unit
    let assult_intercessors = Stats {
        m: 6,
        ws: 3,
        bs: 3,
        s: 5,
        t: 4,
        w: 2,
        a: 2,
        ld: 7,
        sv: 3,
        inv: 7,
        ap: 1,
        dam: 1,
    };
    let enemy_unit = Unit::new("enemy_unit", 120, assult_intercessors, 5);

    // Simulate combat
    Unit::simulate_combat(hive_tyrant.clone(), enemy_unit.clone());
}
