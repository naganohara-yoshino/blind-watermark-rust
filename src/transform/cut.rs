use crate::{Block, BlockCutted, DwtedYCrBrAMat, BLOCK_SIZE};

impl DwtedYCrBrAMat {
    /// Cut the DWT transformed matrix into blocks
    pub fn cut(self) -> BlockCutted {
        let mut y_ll_blocks = Vec::new();
        let mut cb_ll_blocks = Vec::new();
        let mut cr_ll_blocks = Vec::new();

        let y_ll = self.y.0.as_ref();
        let cb_ll = self.cb.0.as_ref();
        let cr_ll = self.cr.0.as_ref();

        let (height, width) = y_ll.shape();
        let block_count_height = height / BLOCK_SIZE;
        let block_count_width = width / BLOCK_SIZE;

        for i in 0..block_count_height {
            for j in 0..block_count_width {
                let y_ll_block = y_ll
                    .submatrix(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .to_owned();
                let cb_ll_block = cb_ll
                    .submatrix(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .to_owned();
                let cr_ll_block = cr_ll
                    .submatrix(i * BLOCK_SIZE, j * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE)
                    .to_owned();
                y_ll_blocks.push(Block {
                    mat_data: y_ll_block,
                });
                cb_ll_blocks.push(Block {
                    mat_data: cb_ll_block,
                });
                cr_ll_blocks.push(Block {
                    mat_data: cr_ll_block,
                });
            }
        }
        BlockCutted {
            y_ll_blocks,
            cb_ll_blocks,
            cr_ll_blocks,
            y: self.y,
            cb: self.cb,
            cr: self.cr,
            a: self.a,
            original_dimensions: self.original_dimensions,
            blocks_dimensions: (block_count_height, block_count_width),
        }
    }
}
