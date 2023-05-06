use log::debug;

use crate::models::dice::roll_dice;

#[derive(Debug, Clone)]
pub struct Stats {
    pub m: i8,
    pub ws: i8,
    pub bs: i8,
    pub s: i8,
    pub t: i8,
    pub w: i8,
    pub a: i8,
    pub ld: i8,
    pub sv: i8,
    pub inv: i8,
    pub ap: i8,
    pub dam: i8,
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub name: String,
    pub points: i16,
    pub quantity: i8,
    pub stats: Stats,
}

// Implement methods for creating new units and simulating combat
impl Unit {
    pub fn new(name: &str, points: i16, stats: Stats, quantity: i8) -> Self {
        Unit {
            name: name.to_string(),
            points,
            stats,
            quantity,
        }
    }

    pub fn hit(&self) -> i8 {
        let mut successful_attacks = 0;
        for _ in 0..self.stats.a {
            let d6 = roll_dice();
            if d6 >= self.stats.ws {
                successful_attacks += 1;
                debug!("Successful Hit!, {}", d6);
            } else {
                debug!("Failed Hit!, {}", d6);
            }
        }

        debug!("Total Successful Hits: {} \n---", &successful_attacks);
        successful_attacks
    }

    pub fn wound(&self, defender: &mut Unit, successful_attacks: i8) -> i8 {
        let mut successful_wounds = 0;
        // calculate what the wound threshold is
        let success_threshold = match self.stats.s.cmp(&defender.stats.t) {
            std::cmp::Ordering::Greater => {
                if self.stats.s >= defender.stats.t * 2 {
                    2
                } else {
                    3
                }
            }
            std::cmp::Ordering::Equal => 4,
            std::cmp::Ordering::Less => {
                if self.stats.s * 2 <= defender.stats.t {
                    6
                } else {
                    5
                }
            }
        };
        debug!("Successful Wound Threshold is : {}", &success_threshold);

        // roll to wound
        for _ in 0..successful_attacks {
            let d6 = roll_dice();
            if d6 >= success_threshold {
                successful_wounds += 1;
                debug!("Successful Wound!, {}", d6);
            } else {
                debug!("Failed Wound!, {}", d6);
            }
        }
        debug!("Total Successfull Wounds: {} \n---", &successful_wounds);
        successful_wounds
    }

    pub fn save(&self, defender: &mut Unit, successful_wounds: i8) -> i8 {
        let mut wounds: i8 = 0;

        // check if normal or invulnerable save should be used
        let save: i8;
        if defender.stats.sv + self.stats.ap < defender.stats.inv {
            save = defender.stats.sv + self.stats.ap;
            debug!(
                "using normal save of {}+ reduced to {}+ by attackers ap of -{}",
                &defender.stats.sv, &save, &self.stats.ap
            )
        } else {
            save = defender.stats.inv;
            debug!("using invuln save of {}, because the normal save of {} was reduced to {} by the enemy ap of {}", 
            &defender.stats.inv, &defender.stats.sv, &defender.stats.sv + &self.stats.ap, &self.stats.ap);
        }

        // roll saves
        for _ in 0..successful_wounds {
            let d6 = roll_dice();
            if d6 < save {
                wounds += 1;
                debug!("failed save!, {}", &d6)
            } else {
                debug!("successful save!, {}", &d6)
            }
        }

        debug!("Total Failed Saves: {} \n---", &wounds);
        wounds
    }

    pub fn damage(&self, defender: &mut Unit, wounds: i8) -> i8 {
        let mut remaining_wounds: i8 = defender.stats.w;
        let mut losses: i8 = 0;
        for _ in 0..wounds {
            debug!(
                "{} models left and wound count is {}",
                &defender.quantity - &losses,
                &remaining_wounds
            );
            if self.stats.dam >= defender.stats.w {
                losses += 1;
                remaining_wounds = defender.stats.w;
                debug!(
                    "unit lost becuase the damage of {} was greater than the wounds of {}",
                    &self.stats.dam, &defender.stats.w
                );
            } else {
                if self.stats.dam >= remaining_wounds {
                    debug!(
                        "unit lost becuase the damage of {} was greater than the remaining wounds of {}",
                        &self.stats.dam, &remaining_wounds
                    );
                    losses += 1;
                    remaining_wounds = defender.stats.w;
                } else {
                    debug!(
                        "unit damaged, but survived becuase the damage of {} was less than the remaining wounds of {}",
                        &self.stats.dam, &remaining_wounds
                    );
                    remaining_wounds -= self.stats.dam
                }
            }
        }
        let remaining_units = {
            if losses >= defender.quantity {
                0
            } else {
                defender.quantity - losses
            }
        };
        remaining_units
    }

    pub fn attack(&self, defender: &mut Unit) {
        let successful_attacks = self.hit();

        let successful_wounds = self.wound(defender, successful_attacks);

        let wounds: i8 = self.save(defender, successful_wounds);

        let losses: i8 = self.damage(defender, wounds);

        println!(
            "{} failed saves, generating {} damage each, which leaves the enemy unit at {} models",
            &wounds, &self.stats.dam, &losses
        );
    }

    pub fn simulate_combat(attacker: Unit, mut defender: Unit) {
        println!("Before combat:");
        println!("Attacker: {:?}", attacker);
        println!("Defender: {:?}", defender);

        attacker.attack(&mut defender);

        //     println!("After combat:");
        //     println!("Attacker: {:?}", attacker);
        //     println!("Defender: {:?}", defender);
    }
}
