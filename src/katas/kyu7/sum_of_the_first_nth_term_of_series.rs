pub fn series_sum(n: u32) -> String {
    if n == 0 {
        return String::from("0.00");
    }
    let mut divider: f32 = 4.0;
    let mut result: f32 = 1.0;
    let mut breaker: u32 = 0;
    while breaker != n - 1 {
        result = result + (1.0 / divider);
        divider = divider + 3.0;
        breaker += 1;
    }

    return format!("{:.2}", result);
}

#[cfg(test)]
mod tests {
    use super::series_sum;

    fn test(input: u32, expected: &str) {
        let actual = series_sum(input);
        assert!(
            actual == expected,
            "Expected series_sum({input}) to be {expected}, but was {actual}"
        );
    }

    #[test]
    fn sample_tests() {
        test(1, "1.00");
        test(2, "1.25");
        test(3, "1.39");
        test(7, "1.68");
        test(39, "2.26");
        test(0, "0.00");
    }
}
