use image::{GenericImageView, ImageReader};
use yuv::{YuvChromaSubsampling, YuvConversionMode, YuvPlanarImageMut, YuvRange, YuvStandardMatrix, rgb_to_yuv444};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1️⃣ 读取图像
    let img = ImageReader::open("example.jpg")?.decode()?;
    let (width, height) = img.dimensions();

    // 2️⃣ 转换成 RGBA8
    let rgb_image = img.as_rgb8().unwrap();

    // 3️⃣ 分配 YUV444 缓冲区
    let mut planar_image =
        YuvPlanarImageMut::<u8>::alloc(width, height, YuvChromaSubsampling::Yuv444);

    let rgb_stride: u32 = width * 3; 

    // 4️⃣ RGB -> YUV444
    rgb_to_yuv444(
        &mut planar_image,
        rgb_image,
        rgb_stride,
        YuvRange::Full,
        YuvStandardMatrix::Bt2020,
        YuvConversionMode::Balanced
    ).unwrap();

    // 5️⃣ 
    let y:Vec<u8> = planar_image.y_plane.borrow().to_vec();


    let y_matrix = image::GrayImage::from_vec(width, height, y).unwrap();
    y_matrix.save("output_y_channel.png")?;
  

    Ok(())
}
