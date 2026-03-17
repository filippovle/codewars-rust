pub fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();

    let mut result = String::new();
    for i in digits {
        result.push_str(i.to_string().as_str());
    }
    return result.parse().unwrap();
}


#[cfg(test)]

mod sample_tests {
    use super::min_value;
   
    fn do_test(digits: Vec<i32>, expected: i32) {
        let actual = min_value(digits.clone());
        assert_eq!(actual, expected, "\nmin_value({:?}) returned an incorrect answer.", digits);
    }

    #[test]
    fn test_sample_inputs() {
        do_test(vec![1, 3, 1], 13);
        do_test(vec![4, 7, 5, 7], 457);
        do_test(vec![4, 8, 1, 4], 148);
    }
}