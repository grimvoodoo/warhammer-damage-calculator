mod data;
mod models;

use data::custodes::{allarus_custodians, allarus_custodians_weapons};
use data::tyranids::{swarmlord, swarmlord_weapons};
use models::combat::{Model, Stats, Unit};

// Main function
fn main() {
    env_logger::init();

    let my_allarus_custodian_1 = Model::new(
        "Allarus Custodian with Grenade Launcher",
        allarus_custodians().stats.clone(),
        Some(allarus_custodians_weapons().ranged_weapons[0].clone()),
        Some(allarus_custodians_weapons().melee_weapons[0].clone()),
        allarus_custodians().tags.clone(),
    );
    let my_allarus_custodian_2 = Model::new(
        "Allarus Custodian with spear",
        allarus_custodians().stats.clone(),
        Some(allarus_custodians_weapons().ranged_weapons[2].clone()),
        Some(allarus_custodians_weapons().melee_weapons[2].clone()),
        allarus_custodians().tags.clone(),
    );

    let my_allarus_custodian_sqaud = Unit::new(
        "Allarus Custodians",
        200,
        vec![my_allarus_custodian_1, my_allarus_custodian_2],
        Some(allarus_custodians().tags.clone()),
    );

    let my_swarmlord_model = Model::new(
        "Swarmlord",
        swarmlord().stats.clone(),
        Some(swarmlord_weapons().ranged_weapons[0].clone()),
        Some(swarmlord_weapons().melee_weapons[0].clone()),
        swarmlord().tags.clone(),
    );

    let my_swarmlord_unit = Unit::new(
        "Swarmlord",
        180,
        vec![my_swarmlord_model],
        Some(swarmlord().tags.clone()),
    );

    // let combat = Unit::simulate_combat()
    // // Start 32 inches appart, which seems a realistic distance
    // println!(
    //     "Attacker is {} x {}. \nDefender is {} x {}",
    //     swarmlord().quantity,
    //     swarmlord().name,
    //     genestealers().quantity,
    //     genestealers().name
    // );
    // let distance = 15;
    // let combat = Unit::simulate_combat(swarmlord().clone(), genestealers().clone(), distance);
}
