use rand::Rng;

pub fn roll_dice() -> i8 {
    rand::thread_rng().gen_range(1..=6)
}
