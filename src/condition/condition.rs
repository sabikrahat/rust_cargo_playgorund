pub fn check_condition(x: i64) -> String {
    if x < 10 {
        return "less than 10".to_string();
    } else if x > 10 {
        return "greater than 10".to_string();
    } else {
        return "equal to 10".to_string();
    }
}
