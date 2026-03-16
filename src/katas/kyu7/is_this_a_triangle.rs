pub fn is_triangle(a: i64, b: i64, c: i64) -> bool {
    if a < 1 || b < 1 || c < 1 {
        return false;
    }
    if a > b && a > c {
        return a < b + c;
    }

    if b > a && b > c {
        return b < a + c;
    }

    if c > b && c > a {
        return c < b + a;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::is_triangle;

    #[test]
    fn works_for_some_examples() {
        assert_eq!(is_triangle(1, 2, 2), true);
        assert_eq!(is_triangle(7, 2, 2), false);
        assert_eq!(is_triangle(1, 2, 3), false);
        assert_eq!(is_triangle(1, 3, 2), false);
        assert_eq!(is_triangle(3, 1, 2), false);
        assert_eq!(is_triangle(5, 1, 2), false);
        assert_eq!(is_triangle(1, 2, 5), false);
        assert_eq!(is_triangle(2, 5, 1), false);
        assert_eq!(is_triangle(4, 2, 3), true);
        assert_eq!(is_triangle(5, 1, 5), true);
        assert_eq!(is_triangle(2, 2, 2), true);
        assert_eq!(is_triangle(-1, 2, 3), false);
        assert_eq!(is_triangle(1, -2, 3), false);
        assert_eq!(is_triangle(1, 2, -3), false);
        assert_eq!(is_triangle(0, 2, 3), false);
    }
}
