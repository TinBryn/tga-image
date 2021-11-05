use geometry::Vec2i;
use tga_image as tga;

fn main() {
    let mut frame = tga::Image::new(200, 200, 3);
    let points: [Vec2i; 3] = [[10, 10].into(), [100, 30].into(), [190, 160].into()];

    renderer::triangle(&mut frame, points, renderer::colors::RED);
    frame
        .write_tga_file("barycentric.tga", tga::Encoding::Rle)
        .unwrap();
}
