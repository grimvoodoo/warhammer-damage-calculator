mod data;
mod models;

use data::tyranids::{genestealers, swarmlord};
use models::combat::{Stats, Unit};

// Main function
fn main() {
    env_logger::init();
    // Start 32 inches appart, which seems a realistic distance
    println!(
        "Attacker is {} x {}. \nDefender is {} x {}",
        swarmlord().quantity,
        swarmlord().name,
        genestealers().quantity,
        genestealers().name
    );
    let distance = 15;
    let combat = Unit::simulate_combat(swarmlord().clone(), genestealers().clone(), distance);
}
