//! Smoke tests for the shared property-based testing setup.
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    #[test]
    fn reversing_twice_preserves_bytes(input in proptest::collection::vec(any::<u8>(), 0..256)) {
        let mut reversed = input.clone();
        reversed.reverse();
        reversed.reverse();
        prop_assert_eq!(reversed, input);
    }
}
