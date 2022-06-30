use num_traits::WrappingAdd;

/// Construct a delta-decoder
#[derive(Debug, Default, Copy, Clone)]
pub struct DeltaDecoder<T> {
    current: T,
}

impl<T: WrappingAdd + Copy> DeltaDecoder<T> {
    pub fn decode(&mut self, value: T) -> T {
        self.current = self.current.wrapping_add(&value);
        self.current
    }
}
