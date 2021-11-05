use renderer::{Draw, colors::*};
use tga_image as tga;

fn main() -> std::io::Result<()> {
    let mut image = tga::Image::new(100, 100, 3);

    image.draw_line([13, 25], [80, 45], WHITE);
    image.draw_line([20, 13], [40, 80], RED);
    image.draw_line([80, 40], [13, 20], RED);

    image.write_tga_file("line1.tga", tga::Encoding::Rle)?;
    Ok(())
}
