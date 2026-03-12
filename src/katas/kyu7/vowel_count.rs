fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;

    for c in string.chars() {
        match c {
            'a' => vowels_count += 1,
            'e' => vowels_count += 1,
            'i' => vowels_count += 1,
            'o' => vowels_count += 1,
            'u' => vowels_count += 1,
            _ => {}
        }
    }
    vowels_count
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}
