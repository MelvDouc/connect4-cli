pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) const SEEDS: [u64; 2] = [999999, 4100382397009];

    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }

    #[allow(dead_code)]
    pub(crate) fn test_rng() -> Self {
        Self::new(Self::SEEDS[0])
    }

    const fn xor_shift(mut n: u64) -> u64 {
        n ^= n << 13;
        n ^= n >> 7;
        n ^= n << 17;
        n
    }

    const fn next(&mut self) -> u64 {
        self.0 = Self::xor_shift(self.0);
        self.0
    }

    pub(crate) const fn sparse_u64(&mut self) -> u64 {
        self.next() & self.next() & self.next()
    }
}
