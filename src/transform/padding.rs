use crate::{PaddedYCrBrAMat, YCrBrAMat};
use faer::prelude::*;

impl YCrBrAMat {
    pub fn add_padding(mut self) -> PaddedYCrBrAMat {
        let (height, width) = self.dimensions;

        match (height % 2 == 1, width % 2 == 1) {
            (true, true) => {
                self.add_zero_row_yuv();
                self.add_zero_col_yuv();
                self.internal_into_padded()
            }
            (true, false) => {
                self.add_zero_row_yuv();
                self.internal_into_padded()
            }
            (false, true) => {
                self.add_zero_col_yuv();
                self.internal_into_padded()
            }
            (false, false) => self.internal_into_padded(),
        }
    }

    fn add_zero_row_yuv(&mut self) {
        let cols = self.y.ncols();
        let zero_row = Row::<f32>::zeros(cols);
        let zero_row_view = zero_row.as_ref();

        self.y.push_row(zero_row_view);
        self.cb.push_row(zero_row_view);
        self.cr.push_row(zero_row_view);
    }

    fn add_zero_col_yuv(&mut self) {
        let rows = self.y.nrows();
        let zero_col = Col::<f32>::zeros(rows);
        let zero_col_view = zero_col.as_ref();

        self.y.push_col(zero_col_view);
        self.cb.push_col(zero_col_view);
        self.cr.push_col(zero_col_view);
    }

    fn internal_into_padded(self) -> PaddedYCrBrAMat {
        PaddedYCrBrAMat {
            y: self.y,
            cb: self.cb,
            cr: self.cr,
            a: self.a,
            original_dimensions: self.dimensions,
        }
    }
}

impl PaddedYCrBrAMat {
    pub fn remove_padding(self) -> YCrBrAMat {
        YCrBrAMat {
            y: self
                .y
                .as_ref()
                .submatrix(0, 0, self.original_dimensions.0, self.original_dimensions.1)
                .to_owned(),
            cb: self
                .cb
                .as_ref()
                .submatrix(0, 0, self.original_dimensions.0, self.original_dimensions.1)
                .to_owned(),
            cr: self
                .cr
                .as_ref()
                .submatrix(0, 0, self.original_dimensions.0, self.original_dimensions.1)
                .to_owned(),
            a: self.a,
            dimensions: self.original_dimensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_padding() {
        let yuv = YCrBrAMat {
            y: Mat::<f32>::ones(2, 3),
            cb: Mat::<f32>::ones(2, 3),
            cr: Mat::<f32>::ones(2, 3),
            a: Mat::<f32>::ones(2, 3),
            dimensions: (2, 3),
        };
        let padded = yuv.add_padding();
        assert_eq!(padded.y.ncols(), 4);
        assert_eq!(padded.cr.ncols(), 4);
        assert_eq!(padded.cb.ncols(), 4);
        assert_eq!(padded.y.nrows(), 2);
        assert_eq!(padded.cb.nrows(), 2);
        assert_eq!(padded.cr.nrows(), 2);
        assert_eq!(padded.a.nrows(), 2);
        assert_eq!(padded.a.ncols(), 3);
    }

    #[test]
    fn test_remove_padding() {
        let yuv = YCrBrAMat {
            y: Mat::<f32>::ones(2, 3),
            cb: Mat::<f32>::ones(2, 3),
            cr: Mat::<f32>::ones(2, 3),
            a: Mat::<f32>::ones(2, 3),
            dimensions: (2, 3),
        };
        let padded = yuv.add_padding();
        let yuv_recovered = padded.remove_padding();
        assert_eq!(yuv_recovered.y.nrows(), 2);
        assert_eq!(yuv_recovered.y.ncols(), 3);
        assert_eq!(yuv_recovered.cb.nrows(), 2);
        assert_eq!(yuv_recovered.cb.ncols(), 3);
        assert_eq!(yuv_recovered.cr.nrows(), 2);
        assert_eq!(yuv_recovered.cr.ncols(), 3);
        assert_eq!(yuv_recovered.a.nrows(), 2);
        assert_eq!(yuv_recovered.a.ncols(), 3);
    }
}
