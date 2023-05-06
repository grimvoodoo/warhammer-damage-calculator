use rand::Rng;

pub fn d6() -> i8 {
    rand::thread_rng().gen_range(1..=6)
}
