pub fn feast(beast: &str, dish: &str) -> bool {
    beast.starts_with(dish.chars().next().unwrap()) && beast.ends_with(dish.chars().last().unwrap())
}


// Rust test example:
#[test]
fn sample_test_cases() {
  assert_eq!(feast("great blue heron", "garlic naan"), true);
  assert_eq!(feast("chickadee", "chocolate cake"), true);
  assert_eq!(feast("brown bear", "bear claw"), false);
}