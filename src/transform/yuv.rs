use crate::YCrBrAMat;
use faer::prelude::*;
use faer::traits::ComplexField;
use image::Rgba32FImage;
use image::{Rgb, Rgba};
use num::Float;

/// Floating point YCrBr pixel, Cb/Cr centered at [-0.5, 0.5]
pub struct YCrBrPixel<T: Float + ComplexField + Send + Sync> {
    pub y: T,
    pub cb: T,
    pub cr: T,
}

impl From<Rgb<f32>> for YCrBrPixel<f32> {
    fn from(Rgb([r, g, b]): Rgb<f32>) -> Self {
        // BT.709 linear
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let cb = (b - y) / 1.8556; // Blue difference
        let cr = (r - y) / 1.5748; // Red difference

        YCrBrPixel { y, cb, cr }
    }
}

impl From<YCrBrPixel<f32>> for Rgb<f32> {
    fn from(YCrBrPixel { y, cb, cr }: YCrBrPixel<f32>) -> Self {
        let r = y + 1.5748 * cr;
        let g = y - 0.187324 * cb - 0.468124 * cr;
        let b = y + 1.8556 * cb;

        Rgb([r, g, b])
    }
}

/// Floating point YCrBrA pixel, with alpha
pub struct YCrBrAPixel<T: Float + ComplexField + Send + Sync> {
    pub y: T,
    pub cb: T,
    pub cr: T,
    pub a: T,
}

impl From<Rgba<f32>> for YCrBrAPixel<f32> {
    fn from(Rgba([r, g, b, a]): Rgba<f32>) -> Self {
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let cb = (b - y) / 1.8556;
        let cr = (r - y) / 1.5748;

        YCrBrAPixel { y, cb, cr, a }
    }
}

impl From<YCrBrAPixel<f32>> for Rgba<f32> {
    fn from(YCrBrAPixel { y, cb, cr, a }: YCrBrAPixel<f32>) -> Self {
        let r = y + 1.5748 * cr;
        let g = y - 0.187324 * cb - 0.468124 * cr;
        let b = y + 1.8556 * cb;

        Rgba([r, g, b, a])
    }
}

impl From<Rgba32FImage> for YCrBrAMat {
    fn from(img: Rgba32FImage) -> Self {
        let (width, height) = img.dimensions();
        let width = width as usize;
        let height = height as usize;
        let mut y = Mat::<f32>::zeros(height, width);
        let mut cb = Mat::<f32>::zeros(height, width);
        let mut cr = Mat::<f32>::zeros(height, width);
        let mut a = Mat::<f32>::zeros(height, width);

        for i in 0..height {
            for j in 0..width {
                let &pixel = img.get_pixel(j as u32, i as u32);
                let ycbcra: YCrBrAPixel<f32> = pixel.into();
                y[(i, j)] = ycbcra.y;
                cb[(i, j)] = ycbcra.cb;
                cr[(i, j)] = ycbcra.cr;
                a[(i, j)] = ycbcra.a;
            }
        }
        YCrBrAMat {
            y,
            cb,
            cr,
            a,
            dimensions: (height, width),
        }
    }
}

impl From<YCrBrAMat> for Rgba32FImage {
    fn from(mat: YCrBrAMat) -> Self {
        let (height, width) = mat.dimensions;
        let mut img = Rgba32FImage::new(width as u32, height as u32);
        for i in 0..height {
            for j in 0..width {
                let ycbcra = YCrBrAPixel {
                    y: mat.y[(i, j)],
                    cb: mat.cb[(i, j)],
                    cr: mat.cr[(i, j)],
                    a: mat.a[(i, j)],
                };
                img.put_pixel(j as u32, i as u32, ycbcra.into());
            }
        }
        img
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq; // For floating point comparison

    #[test]
    fn test_rgb_to_ycrbr_and_back() {
        let rgb = Rgb([0.2, 0.5, 0.7]);
        let ycrbr: YCrBrPixel<f32> = rgb.into();
        let rgb_converted: Rgb<f32> = ycrbr.into();

        // Because floating point arithmetic may have small errors, use approx for comparison
        assert_relative_eq!(rgb.0[0], rgb_converted.0[0], max_relative = 1e-6);
        assert_relative_eq!(rgb.0[1], rgb_converted.0[1], max_relative = 1e-6);
        assert_relative_eq!(rgb.0[2], rgb_converted.0[2], max_relative = 1e-6);
    }

    #[test]
    fn test_rgba_to_ycrbra_and_back() {
        let rgba = Rgba([0.1, 0.6, 0.3, 0.8]);
        let ycrbra: YCrBrAPixel<f32> = rgba.into();
        let rgba_converted: Rgba<f32> = ycrbra.into();

        assert_relative_eq!(rgba.0[0], rgba_converted.0[0], max_relative = 1e-6);
        assert_relative_eq!(rgba.0[1], rgba_converted.0[1], max_relative = 1e-6);
        assert_relative_eq!(rgba.0[2], rgba_converted.0[2], max_relative = 1e-6);
        assert_relative_eq!(rgba.0[3], rgba_converted.0[3], max_relative = 1e-6);
    }

    #[test]
    fn test_ycrbr_centered_values() {
        let rgb = Rgb([0.0, 0.0, 0.0]);
        let ycrbr: YCrBrPixel<f32> = rgb.into();
        // For black, cb/cr should be close to 0
        assert_relative_eq!(ycrbr.cb, 0.0, max_relative = 1e-6);
        assert_relative_eq!(ycrbr.cr, 0.0, max_relative = 1e-6);

        let rgb = Rgb([1.0, 1.0, 1.0]);
        let ycrbr: YCrBrPixel<f32> = rgb.into();
        // For white, cb/cr should be close to 0
        assert_relative_eq!(ycrbr.cb, 0.0, max_relative = 1e-6);
        assert_relative_eq!(ycrbr.cr, 0.0, max_relative = 1e-6);
    }

    #[test]
    fn test_ycrbra_alpha_preserved() {
        let rgba = Rgba([0.3, 0.4, 0.5, 0.9]);
        let ycrbra: YCrBrAPixel<f32> = rgba.into();
        // alpha is not involved in conversion, should remain unchanged
        assert_relative_eq!(ycrbra.a, 0.9, max_relative = 1e-6);
    }

    #[test]
    fn test_ycbcr_round_trip() {
        let width = 10;
        let height = 10;
        let mut img = Rgba32FImage::new(width, height);

        // Fill with some test data
        for y in 0..height {
            for x in 0..width {
                let pixel = Rgba([
                    (x as f32) / (width as f32),
                    (y as f32) / (height as f32),
                    0.5,
                    1.0,
                ]);
                img.put_pixel(x, y, pixel);
            }
        }

        let mat: YCrBrAMat = img.clone().into();
        let img_back: Rgba32FImage = mat.into();

        assert_eq!(img.dimensions(), img_back.dimensions());

        for y in 0..height {
            for x in 0..width {
                let p1 = img.get_pixel(x, y);
                let p2 = img_back.get_pixel(x, y);

                assert_relative_eq!(p1[0], p2[0], epsilon = 1e-5);
                assert_relative_eq!(p1[1], p2[1], epsilon = 1e-5);
                assert_relative_eq!(p1[2], p2[2], epsilon = 1e-5);
                assert_relative_eq!(p1[3], p2[3], epsilon = 1e-5);
            }
        }
    }
}
