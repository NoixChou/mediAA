use crate::graymap::Graymap;

pub fn image_to_graymap(image_path: String, output_scale: f64) -> Result<Graymap, image::ImageError> {
    let image = image::open(image_path)?.to_rgb8();
    let width = image.width();
    let height = image.height();
    
    let mut pixels = Vec::new();
    
    for y in 0..height {
        let mut line = Vec::new();
        for x in 0..width {
            let image_pixel = image.get_pixel(x, y);
            line.push((image_pixel[0] as f64 * 0.2126 + image_pixel[1] as f64 * 0.7152 + image_pixel[2] as f64 * 0.0722) as u8);
        }
        pixels.push(line);
    }
    
    Ok(Graymap {
        pixels,
        width,
        height,
        output_scale
    })
}