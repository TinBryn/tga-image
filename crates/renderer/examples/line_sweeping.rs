use std::mem::swap;

use geometry::Vec2i;
use renderer::colors::*;
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

    image
        .write_tga_file("line_sweeping.tga", tga::Encoding::Rle)
        .unwrap();
}

pub fn triangle(image: &mut tga::Image, t0: Vec2i, t1: Vec2i, t2: Vec2i, _color: tga::Color) {
    if t0[1] == t1[1] && t0[1] == t2[1] {
        // degenerate triangle can be skipped
        return;
    }

    // the points need to be sorted bottom to top
    let (t0, t1, t2) = sort_3_points_by_y(t0, t1, t2);

    // for every y value in the range
    for y in t0[1]..=t2[1] {
        // one side will go the whole way up
        // get the x from that
        let ax = get_x_on_line_at_y(y, t0, t2);

        // is this the split point
        let second_half = y > t1[1] || t1[1] == t0[1];
        let bx = if second_half {
            // get x from the second shorter (by y) line
            get_x_on_line_at_y(y, t1, t2)
        } else {
            // get x from the first shorter (by y) line
            get_x_on_line_at_y(y, t0, t1)
        };

        // sort to get the lower number first
        let (ax, bx) = sort_2_ints(ax, bx);
        for x in ax..=bx {
            // set each point on this horizontal line
            image.set(x as usize, y as usize, _color);
        }
    }
}

fn sort_2_ints(mut ax: isize, mut bx: isize) -> (isize, isize) {
    if ax > bx {
        swap(&mut ax, &mut bx);
    }
    (ax, bx)
}

fn get_x_on_line_at_y(y: isize, t1: Vec2i, t2: Vec2i) -> isize {
    let delta = t2 - t1;
    t1[0] + (y - t1[1]) * delta[0] / delta[1]
}

fn sort_3_points_by_y(mut t0: Vec2i, mut t1: Vec2i, mut t2: Vec2i) -> (Vec2i, Vec2i, Vec2i) {
    if t0[1] > t1[1] {
        swap(&mut t0, &mut t1);
    }
    if t0[1] > t2[1] {
        swap(&mut t0, &mut t2);
    }
    if t1[1] > t2[1] {
        swap(&mut t1, &mut t2);
    }
    (t0, t1, t2)
}
