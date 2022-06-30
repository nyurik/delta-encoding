#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

mod encoder;
pub use encoder::{DeltaEncoder, DeltaEncoderExt, DeltaEncoderIter};
mod decoder;
pub use decoder::DeltaDecoder;

#[cfg(test)]
mod tests {
    use super::*;

    fn run(original: &[i64], encoded: &[i64]) {
        let mut dlt = DeltaEncoder::default();
        let result: Vec<i64> = original.iter().map(|&v| dlt.encode(v)).collect();
        assert_eq!(result, encoded, "encoded from: {original:?}");

        let mut dlt = DeltaDecoder::default();
        let result: Vec<i64> = encoded.iter().map(|&v| dlt.decode(v)).collect();
        assert_eq!(result, original, "decoded from: {encoded:?}");

        let result: Vec<i64> = original.iter().copied().to_deltas().collect();
        assert_eq!(result, encoded, "into_iter() original: {original:?}");

        // TODO: implement non-consuming iterator
        // let data = original.to_vec();
        // let encoded: Vec<i64> = data.iter().to_deltas().collect();
        // assert_eq!(encoded, encoded, "iter() original: {original:?}");
    }

    #[test]
    fn test() {
        // Delta encoding cannot support deltas bigger than i64::MAX (half the size of u64)
        let min = i64::MIN / 2;
        let max = i64::MAX / 2;

        run(&[], &[]);
        run(&[0], &[0]);
        run(&[1], &[1]);
        run(&[1, 2], &[1, 1]);
        run(&[1, -2], &[1, -3]);
        run(&[1, 3, 5], &[1, 2, 2]);
        run(&[1, 3, 10], &[1, 2, 7]);
        run(&[min], &[min]);
        run(&[max], &[max]);
        run(&[max, min], &[max, (min - max)]);
        run(&[0, max], &[0, max]);
        run(&[0, max, min, max], &[0, max, (min - max), max + max + 1]);
    }
}
