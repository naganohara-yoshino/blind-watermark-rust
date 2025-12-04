use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg64;

use crate::config::WatermarkMode;

#[derive(Debug, Clone)]
pub struct Permutation {
    pub f: Vec<usize>,
}

impl Permutation {
    pub(crate) fn new(n: usize, seed: u64) -> Self {
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut f: Vec<usize> = (0..n).collect();
        f.shuffle(&mut rng);
        Self { f }
    }
}

impl WatermarkMode {
    pub fn corresponding_wmbits_position(&self, block_position: usize, wm_len: usize) -> usize {
        match self {
            // Cycling corresponding watermark bits
            WatermarkMode::Normal => block_position % wm_len,
            // Using Permutation
            WatermarkMode::Strategy(seed) => {
                Permutation::new(wm_len, *seed).f[block_position] % wm_len
            }
        }
    }

    pub fn corresponding_block_positions(
        self,
        wmbits_position: usize,
        wm_len: usize,
        nblocks: usize,
    ) -> Vec<usize> {
        match self {
            // Cycling corresponding watermark bits
            WatermarkMode::Normal => (0..nblocks)
                .filter(|&i| i % wm_len == wmbits_position)
                .collect(),
            // Using Permutation
            WatermarkMode::Strategy(seed) => (0..nblocks)
                .filter(|&i| Permutation::new(wm_len, seed).f[i] % wm_len == wmbits_position)
                .collect(),
        }
    }
}
