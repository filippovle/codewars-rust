pub fn contamination(text: &str, character: &str) -> String {
    let mut result: String = String::new();
    if character.is_empty() {
        return result;
    }
    let ch: char = character.chars().next().unwrap();

    let size: usize = text.len();

    for _c in 0..size {
        result.push(ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::contamination;

    fn dotest(a: &str, b: &str, expected: &str) {
        let actual = contamination(a, b);
        assert!(
            actual == expected,
            "With text = \"{a}\", character = \"{b}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("abc", "z", "zzz");
        dotest("", "z", "");
        dotest("abc", "", "");
        dotest("_3ebzgh4", "&", "&&&&&&&&");
        dotest("//case", " ", "      ");
    }
}
