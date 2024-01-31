use rand::seq::SliceRandom;
use rand::thread_rng;

pub const MAX_REFRESH_RATE: f32 = 33.33;
pub const MS_IN_SECS: f32 = 1000.;

pub fn parse_string_to_i32_or_default(input: String, default: i32) -> i32 {
    let parsed_result = input.parse::<i32>();

    // Use the result or default to 0
    match parsed_result {
        Ok(parsed_value) => parsed_value,
        Err(_) => default,
    }
}

pub fn shuffle<T>(mut data: Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    data
}

pub fn get_new_generation(items_count: &i32) -> Vec<i32> {
    shuffle((1..=*items_count).collect())
}
