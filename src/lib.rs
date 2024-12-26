#![allow(clippy::needless_doctest_main)]
#![doc = include_str!("../README.md")]

mod decoder;
mod encoder;

pub use decoder::{DeltaDecoder, DeltaDecoderExt, DeltaDecoderIter};
pub use encoder::{DeltaEncoder, DeltaEncoderExt, DeltaEncoderIter};

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;

    pub(crate) const TEST_DATA: &[(&[i64], &[i64])] = &[
        (&[], &[]),
        (&[0], &[0]),
        (&[1], &[1]),
        (&[1, 2], &[1, 1]),
        (&[1, -2], &[1, -3]),
        (&[1, 3, 5], &[1, 2, 2]),
        (&[1, 3, 10], &[1, 2, 7]),
        (&[i64::MIN], &[i64::MIN]),
        (&[i64::MAX], &[i64::MAX]),
        (
            &[i64::MAX, i64::MIN],
            &[i64::MAX, i64::MIN.wrapping_sub(i64::MAX)],
        ),
        (&[0, i64::MAX], &[0, i64::MAX]),
        (
            &[0, i64::MAX, i64::MIN, i64::MAX],
            &[
                0,
                i64::MAX,
                i64::MIN.wrapping_sub(i64::MAX),
                i64::MAX.wrapping_add(i64::MAX) + 1,
            ],
        ),
    ];

    fn run(original: &[i64], encoded: &[i64]) {
        assert_eq!(original.len(), encoded.len());

        let mut enc = DeltaEncoder::default();
        let mut dec = DeltaDecoder::default();
        for (&o, &e) in zip(original, encoded) {
            assert_eq!(enc.encode(o), e, "individual encoded value mismatch");
            assert_eq!(dec.decode(e), o, "individual decoded value mismatch");
        }

        let result: Vec<i64> = encoded.iter().copied().original().deltas().collect();
        assert_eq!(result, encoded, "round-trip decoded: {encoded:?}");

        let result: Vec<i64> = original.iter().copied().deltas().original().collect();
        assert_eq!(result, original, "round-trip original: {original:?}");
    }

    #[test]
    fn test() {
        for &(original, encoded) in TEST_DATA {
            run(original, encoded);
        }
    }
}
