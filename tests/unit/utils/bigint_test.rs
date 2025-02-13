/// @dfns-sdk-rs/tests/unit/utils/bigint_test.rs

#[cfg(test)]
mod tests {
    use dfns_sdk_rs::utils::bigint::minimize_bigint;

    #[test]
    fn test_minimize_bigint_zero() {
        let input = vec![0];
        assert_eq!(minimize_bigint(&input), vec![0]);
    }

    #[test]
    fn test_minimize_bigint_single_byte() {
        let input = vec![42];
        assert_eq!(minimize_bigint(&input), vec![42]);
    }

    #[test]
    fn test_minimize_bigint_leading_zeros() {
        let input = vec![0, 0, 0, 42];
        assert_eq!(minimize_bigint(&input), vec![42]);
    }

    #[test]
    fn test_minimize_bigint_negative_number() {
        let input = vec![0xFF];
        assert_eq!(minimize_bigint(&input), vec![0, 0xFF]);
    }

    #[test]
    fn test_minimize_bigint_large_positive() {
        let input = vec![0x7F, 0xFF];
        assert_eq!(minimize_bigint(&input), vec![0x7F, 0xFF]);
    }

    #[test]
    fn test_minimize_bigint_large_negative() {
        let input = vec![0xFF, 0xFF];
        assert_eq!(minimize_bigint(&input), vec![0, 0xFF, 0xFF]);
    }

    #[test]
    fn test_minimize_bigint_multiple_leading_zeros() {
        let input = vec![0, 0, 0, 0, 0x7F];
        assert_eq!(minimize_bigint(&input), vec![0x7F]);
    }

    #[test]
    fn test_minimize_bigint_empty() {
        let input: Vec<u8> = vec![];
        assert_eq!(minimize_bigint(&input), vec![0]);
    }

    #[test]
    fn test_minimize_bigint_all_zeros() {
        let input = vec![0, 0, 0, 0];
        assert_eq!(minimize_bigint(&input), vec![0]);
    }

    #[test]
    fn test_minimize_bigint_boundary_values() {
        let input = vec![0x80];
        assert_eq!(minimize_bigint(&input), vec![0, 0x80]);

        let input = vec![0x7F];
        assert_eq!(minimize_bigint(&input), vec![0x7F]);
    }

    #[test]
    fn test_minimize_bigint_complex_sequence() {
        let input = vec![0, 0, 0xFF, 0x7F, 0xFF];
        assert_eq!(minimize_bigint(&input), vec![0, 0xFF, 0x7F, 0xFF]);
    }

    #[test]
    fn test_minimize_bigint_alternating_values() {
        let input = vec![0, 0xFF, 0, 0xFF];
        assert_eq!(minimize_bigint(&input), vec![0, 0xFF, 0, 0xFF]);
    }
}
