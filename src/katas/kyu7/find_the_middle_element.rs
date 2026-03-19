pub fn gimme(input_array: [i32; 3]) -> usize {
    if input_array[0] > input_array[1] && input_array[0] < input_array[2]
        || input_array[0] < input_array[1] && input_array[0] > input_array[2]
    {
        return 0;
    }
    if input_array[1] > input_array[0] && input_array[1] < input_array[2]
        || input_array[1] < input_array[0] && input_array[1] > input_array[2]
    {
        return 1;
    }
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gimme() {
        assert_eq!(gimme([2, 3, 1]), 0);
        // assert_eq!(gimme([-2, -3, -1]), 0);
        // assert_eq!(gimme([5, 10, 14]), 1);
    }
}
