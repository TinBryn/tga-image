use geometry::Vec2i;
use renderer::{Draw, colors::*};
use tga_image as tga;

fn main() {
    let mut image = tga::Image::new(200, 200, 3);
    let t0 = [
        Vec2i::new2(10, 70),
        Vec2i::new2(50, 160),
        Vec2i::new2(70, 80),
    ];
    let t1 = [
        Vec2i::new2(180, 50),
        Vec2i::new2(150, 1),
        Vec2i::new2(70, 180),
    ];
    let t2 = [
        Vec2i::new2(180, 150),
        Vec2i::new2(120, 160),
        Vec2i::new2(130, 180),
    ];

    triangle(&mut image, t0[0], t0[1], t0[2], RED);
    triangle(&mut image, t1[0], t1[1], t1[2], WHITE);
    triangle(&mut image, t2[0], t2[1], t2[2], GREEN);

    image.write_tga_file("line_sweeping.tga", tga::Encoding::Rle).unwrap();
}

pub fn triangle(image: &mut tga::Image, t0: Vec2i, t1: Vec2i, t2: Vec2i, color: tga::Color) {
    image.draw_line(t0, t1, color);
    image.draw_line(t1, t2, color);
    image.draw_line(t2, t0, color);
}
