use log::debug;

use crate::models::dice::d6;

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub movement: i8,
    pub toughness: i8,
    pub save: i8,
    pub invunlerable: i8,
    pub wounds: i8,
    pub leadership: i8,
    pub objective_control: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weapons {
    pub name: String,
    pub weapon_range: i8,
    pub attacks: i8,
    pub hit: i8,
    pub strength: i8,
    pub armour_pen: i8,
    pub damage: i8,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: String,
    pub points: i16,
    pub quantity: i8,
    pub stats: Stats,
    pub ranged_weapons: Option<Vec<Weapons>>,
    pub melee_weapons: Option<Vec<Weapons>>,
    pub tags: Option<Vec<String>>,
}

impl Unit {
    pub fn new(
        name: &str,
        points: i16,
        stats: Stats,
        quantity: i8,
        ranged_weapons: Option<Vec<Weapons>>,
        melee_weapons: Option<Vec<Weapons>>,
        tags: Option<Vec<String>>,
    ) -> Self {
        Unit {
            name: name.to_string(),
            points,
            stats,
            quantity,
            tags,
            ranged_weapons,
            melee_weapons,
        }
    }

    pub fn movement(&self, distance: i8) -> i8 {
        let movement = distance - self.stats.movement;
        if movement <= 0 {
            0
        } else {
            movement
        }
    }

    pub fn shooting(
        attacker: &Unit,
        defender: &Unit,
        target_models: i8,
        target_wounds: i8,
        distance: i8,
    ) -> (i8, i8) {
        let mut defender_models = target_models;
        let mut defender_wounds = target_wounds;
        if let Some(ranged_weapons) = &attacker.ranged_weapons {
            for weapon in ranged_weapons {
                if distance <= weapon.weapon_range {
                    debug!(
                        "Target is {} inches away, which is within range of {}. Shooting now!",
                        distance, weapon.weapon_range
                    );
                    debug!("Successful Hit Threshold is {}", weapon.hit);
                    let hits = Self::hit(&weapon);
                    let wounds = Self::wound(&weapon, &defender, hits);
                    let saves = Self::save(&weapon, &defender, wounds);
                    let (models, wounds) = Self::damage(
                        &defender,
                        defender_models,
                        defender_wounds,
                        saves,
                        weapon.damage,
                    );
                    defender_models = models;
                    defender_wounds = wounds;
                    debug!(
                        "Made {} successful shots at the enemy, each dealing {} damage",
                        saves, weapon.damage
                    );
                } else {
                    debug!("Target is {} inches away, which is out of range of weapon {} which has a range of {}", distance, weapon.name, weapon.weapon_range);
                }
            }
        }
        (defender_models, defender_wounds)
    }

    pub fn melee(source: &Unit, target: &Unit, target_models: i8, target_wounds: i8) -> (i8, i8) {
        let mut defender_models = target_models;
        let mut defender_wounds = target_wounds;
        if let Some(melee_weapons) = &source.melee_weapons {
            for weapon in melee_weapons {
                debug!("Target is engaged in melee!");
                debug!("Successful Hit Threshold is {}", weapon.hit);
                let hits = Self::hit(&weapon);
                let wounds = Self::wound(&weapon, &target, hits);
                let saves = Self::save(&weapon, &target, wounds);
                let (models, wounds) = Self::damage(
                    &target,
                    defender_models,
                    defender_wounds,
                    saves,
                    weapon.damage,
                );
                defender_models = models;
                defender_wounds = wounds;
                debug!(
                    "Made {} successful attacks at the enemy, each dealing {} damage",
                    saves, weapon.damage
                );
            }
        }
        (defender_models, defender_wounds)
    }

    pub fn hit(weapon: &Weapons) -> i8 {
        let mut success = 0;
        for _ in 0..weapon.attacks {
            let d6 = d6();
            if d6 >= weapon.hit {
                success += 1;
                debug!("Successful Hit!, {}", d6);
            } else {
                debug!("Failed Hit!, {}", d6);
            }
        }

        debug!("Total Successful Hits: {} \n---", &success);
        success
    }

    pub fn wound(weapon: &Weapons, defender: &Unit, hits: i8) -> i8 {
        let mut success = 0;
        // calculate what the wound threshold is
        let success_threshold = match weapon.strength.cmp(&defender.stats.toughness) {
            std::cmp::Ordering::Greater => {
                if weapon.strength >= defender.stats.toughness * 2 {
                    2
                } else {
                    3
                }
            }
            std::cmp::Ordering::Equal => 4,
            std::cmp::Ordering::Less => {
                if weapon.strength * 2 <= defender.stats.toughness {
                    6
                } else {
                    5
                }
            }
        };
        debug!("Successful Wound Threshold is : {}", &success_threshold);

        // roll to wound
        for _ in 0..hits {
            let d6 = d6();
            if d6 >= success_threshold {
                success += 1;
                debug!("Successful Wound!, {}", d6);
            } else {
                debug!("Failed Wound!, {}", d6);
            }
        }
        debug!("Total Successfull Wounds: {} \n---", &success);
        success
    }

    pub fn save(weapon: &Weapons, defender: &Unit, wounds: i8) -> i8 {
        let mut success: i8 = 0;

        // check if normal or invulnerable save should be used
        let save: i8;
        if defender.stats.save + weapon.armour_pen < defender.stats.invunlerable {
            save = defender.stats.save + weapon.armour_pen;
            debug!(
                "using normal save of {}+ reduced to {}+ by attackers ap of -{}",
                &defender.stats.save, &save, &weapon.armour_pen
            )
        } else {
            save = defender.stats.invunlerable;
            debug!("using invuln save of {}, because the normal save of {} was reduced to {} by the enemy ap of {}",
            &defender.stats.invunlerable, &defender.stats.save, &defender.stats.save + &weapon.armour_pen, &weapon.armour_pen);
        }

        // roll saves
        for _ in 0..wounds {
            let d6 = d6();
            if d6 < save {
                success += 1;
                debug!("failed save!, {}", &d6)
            } else {
                debug!("successful save!, {}", &d6)
            }
        }

        debug!("Total Failed Saves: {} \n---", &success);
        success
    }

    pub fn damage(
        defender: &Unit,
        current_models: i8,
        current_wounds: i8,
        hits: i8,
        damage: i8,
    ) -> (i8, i8) {
        let mut wounds = current_wounds;
        let mut models = current_models;
        for _ in 0..hits {
            match damage {
                d if d >= defender.stats.wounds => {
                    models -= 1;
                    wounds = defender.stats.wounds;
                }
                d if d >= wounds => {
                    models -= 1;
                    wounds = defender.stats.wounds;
                }
                d if d < wounds => wounds -= damage,
                _ => {
                    debug!("Failed to add damage to models");
                    return (0, 0);
                }
            }
        }
        return (models, wounds);
    }

    pub fn charge(self, distance: i8) -> (bool, bool) {
        let mut engaged = false;
        let mut charged = false;
        match distance {
            d if distance <= 0 => {
                engaged = true;
                charged = false;
                debug!("{} is in melee, didn't charge this turn", self.name)
            }
            d if distance <= 12 => {
                let charge_roll = (d6(), d6());
                let charge = charge_roll.0 + charge_roll.1;
                debug!("Attempting to charge, target is {} inches away", distance);
                if charge >= distance {
                    debug!("Charge Successful! with a roll of {}", charge);
                    charged = true;
                    engaged = true;
                } else {
                    debug!("Charge Failed with a roll of {}", charge)
                }
            }
            _ => (),
        }
        return (engaged, charged);
    }

    pub fn check_victory(
        target_models: i8,
        target_wounds: i8,
        target: &Unit,
        source: &Unit,
        source_models: i8,
        source_wounds: i8,
        round: i8,
        ranged: bool,
    ) -> bool {
        let mut combat: String;
        if ranged == true {
            combat = "shooting".to_string()
        } else {
            combat = "melee".to_string()
        }
        if target_models <= 0 {
            println!(
                "{} killed after {} rounds, {} still has {} models and {} wounds",
                target.name, round, source.name, source_models, source_wounds
            );
            true
        } else {
            println!(
                "After {} {} {} remain for the target, with {} wounds on the weakest one.",
                combat, target_models, target.name, target_wounds
            );
            false
        }
    }

    // pub fn damage(&self, defender: &mut Unit, wounds: i8) -> i8 {
    //     let mut remaining_wounds: i8 = defender.stats.w;
    //     let mut losses: i8 = 0;
    //     for _ in 0..wounds {
    //         debug!(
    //             "{} models left and wound count is {}",
    //             &defender.quantity - &losses,
    //             &remaining_wounds
    //         );
    //         if self.stats.dam >= defender.stats.w {
    //             losses += 1;
    //             remaining_wounds = defender.stats.w;
    //             debug!(
    //                 "unit lost becuase the damage of {} was greater than the wounds of {}",
    //                 &self.stats.dam, &defender.stats.w
    //             );
    //         } else {
    //             if self.stats.dam >= remaining_wounds {
    //                 debug!(
    //                     "unit lost becuase the damage of {} was greater than the remaining wounds of {}",
    //                     &self.stats.dam, &remaining_wounds
    //                 );
    //                 losses += 1;
    //                 remaining_wounds = defender.stats.w;
    //             } else {
    //                 debug!(
    //                     "unit damaged, but survived becuase the damage of {} was less than the remaining wounds of {}",
    //                     &self.stats.dam, &remaining_wounds
    //                 );
    //                 remaining_wounds -= self.stats.dam
    //             }
    //         }
    //     }
    //     let remaining_units = {
    //         if losses >= defender.quantity {
    //             0
    //         } else {
    //             defender.quantity - losses
    //         }
    //     };
    //     remaining_units
    // }

    // pub fn attack(&self, defender: &mut Unit) {
    //     let successful_attacks = self.hit();
    //     let successful_wounds = self.wound(defender, successful_attacks);
    //     let wounds: i8 = self.save(defender, successful_wounds);
    //     let losses: i8 = self.damage(defender, wounds);

    //     println!(
    //         "{} failed saves, generating {} damage each, which leaves the enemy unit at {} models",
    //         &wounds, &self.stats.dam, &losses
    //     );
    // }

    pub fn simulate_combat(attacker: Unit, defender: Unit, starting_distance: i8) -> bool {
        let mut distance = starting_distance;
        let mut attacker_models = attacker.quantity;
        let mut attacker_wounds = attacker.stats.wounds;
        let mut defender_models = defender.quantity;
        let mut defender_wounds = defender.stats.wounds;
        let mut round: i8 = 1;
        let mut engaged = false;
        let mut charged = false;
        let mut result = None;

        debug!("Starting {} inches to enemy", distance);

        while result == None {
            println!(
                "Round {}! \n{} has {} models and {} wounds.\n{} has {} models and {} wounds.",
                round,
                attacker.name,
                attacker_models,
                attacker_wounds,
                defender.name,
                defender_models,
                defender_wounds
            );

            distance = attacker.movement(distance);
            debug!("After moving, {} inches to enemy", distance);

            (defender_models, defender_wounds) = Self::shooting(
                &attacker,
                &defender,
                defender_models,
                defender_wounds,
                distance,
            );

            if Self::check_victory(
                defender_models,
                defender_wounds,
                &defender,
                &attacker,
                attacker_models,
                attacker_wounds,
                round,
                true,
            ) {
                result = Some(true);
                break;
            }

            (engaged, charged) = Self::charge(attacker.clone(), distance);

            if engaged && !charged {
                Self::melee(&attacker, &defender, defender_models, defender_wounds);
            }

            if Self::check_victory(
                defender_models,
                defender_wounds,
                &defender,
                &attacker,
                attacker_models,
                attacker_wounds,
                round,
                false,
            ) {
                result = Some(true);
                break;
            }

            if engaged {
                Self::melee(&defender, &attacker, attacker_models, attacker_wounds);
            }

            if Self::check_victory(
                attacker_models,
                attacker_wounds,
                &attacker,
                &defender,
                defender_models,
                defender_wounds,
                round,
                false,
            ) {
                result = Some(false);
                break;
            }

            round += 1;
        }

        result.unwrap()
    }
}
