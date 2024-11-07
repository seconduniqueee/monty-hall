use rand::Rng;

pub fn get_random_int(start: u64, end: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(start..=end);

    return random_number;
}