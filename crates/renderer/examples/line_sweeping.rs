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

fn interpolate(p1: isize, p2: isize, t: f32) -> isize {
    (p1 as f32 + (p2 - p1) as f32 * t) as isize
}

pub fn triangle(image: &mut tga::Image, t0: Vec2i, t1: Vec2i, t2: Vec2i, _color: tga::Color) {
    if t0[1] == t1[1] && t0[1] == t2[1] {
        return;
    }

    let (t0, t1, t2) = sort_points_by_y(t0, t1, t2);

    let total_height = t2[1] - t0[1];

    for y in t0[1]..=t2[1] {
        let second_half = y > t1[1] || t1[1] == t0[1];
        let segment_height = if second_half {
            t2[1] - t1[1]
        } else {
            t1[1] - t0[1]
        };
        let alpha = (y - t0[1]) as f32 / total_height as f32;
        let beta = if second_half { y - t1[1] } else { y - t0[1] } as f32 / segment_height as f32;
        let mut ax = interpolate(t0[0], t2[0], alpha);
        let (p1, p2) = if second_half {
            (t1[0], t2[0])
        } else {
            (t0[0], t1[0])
        };
        let mut bx = interpolate(p1, p2, beta);
        if ax > bx {
            swap(&mut ax, &mut bx);
        }
        for x in ax..=bx {
            image.set(x as usize, y as usize, _color);
        }
    }
}

fn sort_points_by_y(mut t0: Vec2i, mut t1: Vec2i, mut t2: Vec2i) -> (Vec2i, Vec2i, Vec2i) {
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
