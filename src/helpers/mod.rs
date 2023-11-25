pub fn parse_string_to_i32_or_default(input: String, default: i32) -> i32 {
    let parsed_result = input.parse::<i32>();

    // Use the result or default to 0
    match parsed_result {
        Ok(parsed_value) => parsed_value,
        Err(_) => default,
    }
}
