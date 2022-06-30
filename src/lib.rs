#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod encoder;
pub use encoder::{DeltaEncoder, DeltaEncoderExt, DeltaEncoderIter};
mod decoder;
pub use decoder::{DeltaDecoder, DeltaDecoderExt, DeltaDecoderIter};

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    fn run(original: &[i64], encoded: &[i64]) {
        assert_eq!(original.len(), encoded.len());

        let mut enc = DeltaEncoder::default();
        let mut dec = DeltaDecoder::default();
        for (&o, &e) in zip(original, encoded) {
            assert_eq!(enc.encode(o), e, "individual encoded value mismatch");
            assert_eq!(dec.decode(e), o, "individual decoded value mismatch");
        }

        let mut enc = DeltaEncoder::default();
        let result: Vec<i64> = original.iter().map(|&v| enc.encode(v)).collect();
        assert_eq!(result, encoded, "encoded from: {original:?}");

        let mut dec = DeltaDecoder::default();
        let result: Vec<i64> = encoded.iter().map(|&v| dec.decode(v)).collect();
        assert_eq!(result, original, "decoded from: {encoded:?}");

        let result: Vec<i64> = original.iter().copied().deltas().collect();
        assert_eq!(result, encoded, "iter().copied() original: {original:?}");

        let result: Vec<i64> = encoded.iter().copied().original().collect();
        assert_eq!(result, original, "iter().copied() encoded: {encoded:?}");

        let result: Vec<i64> = encoded.iter().copied().original().deltas().collect();
        assert_eq!(result, encoded, "round-trip decoded: {encoded:?}");

        let result: Vec<i64> = original.iter().copied().deltas().original().collect();
        assert_eq!(result, original, "round-trip original: {original:?}");
    }

    #[test]
    fn test() {
        // Delta encoding cannot support deltas bigger than i64::MAX (half the size of u64)
        let min = i64::MIN;
        let max = i64::MAX;

        run(&[], &[]);
        run(&[0], &[0]);
        run(&[1], &[1]);
        run(&[1, 2], &[1, 1]);
        run(&[1, -2], &[1, -3]);
        run(&[1, 3, 5], &[1, 2, 2]);
        run(&[1, 3, 10], &[1, 2, 7]);
        run(&[min], &[min]);
        run(&[max], &[max]);
        run(&[max, min], &[max, min.wrapping_sub(max)]);
        run(&[0, max], &[0, max]);
        run(
            &[0, max, min, max],
            &[0, max, min.wrapping_sub(max), max.wrapping_add(max) + 1],
        );
    }
}
