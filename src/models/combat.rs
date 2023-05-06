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

    pub fn shoot(weapon: &Weapons, defender: &Unit, distance: i8) -> (i8, i8) {
        let mut success = 0;
        if distance <= weapon.weapon_range {
            debug!(
                "Target is {} inches away, which is within range of {}. Shooting now!",
                distance, weapon.weapon_range
            );
            debug!("Successful Hit Threshold is {}", weapon.hit);
            let hits = Self::hit(&weapon);
            let wounds = Self::wound(&weapon, &defender, hits);
            let saves = Self::save(&weapon, &defender, wounds);
            success += saves
        } else {
            debug!("Target is {} inches away, which is out of range of weapon {} which has a range of {}", distance, weapon.name, weapon.weapon_range);
            return (0, 0);
        }
        return (success, weapon.damage);
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

    pub fn simulate_combat(attacker: Unit, defender: Unit, starting_distance: i8) {
        let distance = starting_distance;
        let mut hits = 0;
        let mut damage = 0;
        let mut attacker_models = attacker.quantity;
        let mut attacker_wounds = attacker.stats.wounds;
        let mut defender_models = defender.quantity;
        let mut defender_wounds = defender.stats.wounds;
        let mut round: i8 = 1;

        debug!("Starting {} inches to enemy", distance);
        // start attacker combat loop

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

        // if attacker not in engagement range then move directly towards enemy unit at maximum speed.
        let distance = attacker.movement(distance);
        debug!("{} inches to enemy", distance);

        // shoot ranged weapons if any available and not prevented from shooting in combat.
        if attacker.ranged_weapons != None {
            let ranged_weapon = attacker.ranged_weapons.unwrap();
            for x in 0..ranged_weapon.len() {
                (hits, damage) = Unit::shoot(&ranged_weapon[x], &defender, distance);
                debug!(
                    "Made {} successfull shots at the enemy, each dealing {} damage",
                    hits, damage
                );
                (defender_models, defender_wounds) =
                    Self::damage(&defender, defender_models, defender_wounds, hits, damage)
            }
        }

        if defender_models <= 0 {
            println!(
                "{} killed after {} rounds, {} still has {} models and {} wounds",
                defender.name, round, attacker.name, attacker_models, attacker_wounds
            );
            return;
        } else {
            println!(
                "After shooting {} {} remain for the defender, with {} wounds on the weakest one.",
                defender_models, defender.name, defender_wounds
            )
        }

        // if all enemies dead then end combat with victory and return the number of models and how many wounds left over.

        // if already in combat then skip, otherwise try to charge if within 12 inches.

        // if charge successful then enter combat with charge=true.

        // if in engagement range then run melee else skip.

        // if all enemies dead then end combat with victory and return the number of models and how many wounds left over.

        // test for combat shock on any units that lost models.

        //start defender turn

        // if defender not in engagement range then move directly towards enemy unit at maximum speed.

        // shoot ranged weapons if any available and not prevented from shooting in combat.

        // if all enemies dead then end combat with defeat and return the number of models and how many wounds left over.

        // if already in combat then skip, otherwise try to charge if within 12 inches.

        // if charge successful then enter combat with charge=true.

        // if in engagement range then run melee else skip.

        // if all enemies dead then end combat with defeat and return the number of models and how many wounds left over.

        // test for combat shock on any units that lost models.

        // restart loop carrying over the number of models and wounds.

        // attacker.attack(&mut defender);
    }
}
