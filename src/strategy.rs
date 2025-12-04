use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg64;


/// A permutation strategy for randomizing watermark embedding positions.
///
/// This struct handles the generation of a random permutation sequence based on a seed,
/// which is used to scatter watermark bits across image blocks to enhance security.
#[derive(Debug, Clone)]
pub struct Permutation {
    /// The permuted indices.
    pub f: Vec<usize>,
    pub n: usize,
}

impl Permutation {
    /// Creates a new `Permutation` instance.
    ///
    /// # Arguments
    ///
    /// * `n` - The size of the permutation (number of blocks).
    /// * `seed` - The seed for the random number generator.
    pub(crate) fn new(n: usize, seed: u64) -> Self {
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut f: Vec<usize> = (0..n).collect();
        f.shuffle(&mut rng);
        Self { f, n }
    }


    /// Calculates the position of the watermark bit corresponding to a given block.
    ///
    /// # Arguments
    ///
    /// * `block_position` - The index of the image block.
    /// * `wm_len` - The length of the watermark.
    ///
    /// # Returns
    ///
    /// The index of the watermark bit to be embedded in/extracted from the block.
    pub fn corresponding_wmbits_position(&self, block_position: usize, wm_len: usize) -> usize {
        self.f[block_position] % wm_len
    }

    /// Finds all block positions that correspond to a specific watermark bit position.
    ///
    /// # Arguments
    ///
    /// * `wmbits_position` - The index of the watermark bit.
    /// * `wm_len` - The length of the watermark.
    ///
    /// # Returns
    ///
    /// A vector of block indices that should contain the specified watermark bit.
    pub fn corresponding_block_positions(
        &self,
        wmbits_position: usize,
        wm_len: usize,
    ) -> Vec<usize> {

        (0..self.n)
            .filter(|&i| self.f[i] % wm_len == wmbits_position)
            .collect()
    }
}
