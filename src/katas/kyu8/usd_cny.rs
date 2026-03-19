pub fn usdcny(usd: u16) -> String {
    let res: f32 = usd as f32 * 6.75;
    format!("{:.2} Chinese Yuan", res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(usdcny(15), "101.25 Chinese Yuan");
        assert_eq!(usdcny(465), "3138.75 Chinese Yuan");
    }
}
