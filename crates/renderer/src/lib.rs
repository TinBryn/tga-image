use std::mem::swap;

use geometry::{Vec2i, Vec3f, Vec3i};
use tga_image as tga;

pub trait Draw {
    fn draw_line(&mut self, p0: impl Into<Vec2i>, p1: impl Into<Vec2i>, color: tga::Color);
}

impl Draw for tga::Image {
    fn draw_line(&mut self, p0: impl Into<Vec2i>, p1: impl Into<Vec2i>, color: tga::Color) {
        line_vec(p0.into(), p1.into(), self, color);
    }
}

pub fn barycentric(points: [Vec2i; 3], point: Vec2i) -> Vec3f {
    let ab = points[2] - points[0];
    let ac = points[1] - points[0];
    let pa = points[0] - point;
    let v1 = Vec3i::new3(ab[0], ac[0], pa[0]);
    let v2 = Vec3i::new3(ab[1], ac[1], pa[1]);
    let u = v1.cross(v2);

    if u[2].abs() < 1 {
        Vec3f::new3(-1.0, -1.0, -1.0)
    } else {

        let d = 1.0 / u[2] as f32;
        let (u, v) = (u[0] as f32, u[1] as f32);
        Vec3f::new3(
            1.0 - (u + v)  * d,
            v * d,
            u * d,
        )
    }
}

pub fn triangle(image: &mut tga::Image, points: [Vec2i; 3], color: tga::Color) {
    let mut bboxmin = Vec2i::new2(image.width() as isize - 1, image.height() as isize - 1);
    let mut bboxmax = Vec2i::new2(0, 0);
    let clamp = Vec2i::new2(image.width() as isize - 1, image.height() as isize - 1);

    for point in points {
        for j in 0..2 {
            bboxmin[j] = 0.max(bboxmin[j].min(point[j]));
            bboxmax[j] = clamp[j].min(bboxmax[j].max(point[j]));
        }
    }

    for x in bboxmin[0]..=bboxmax[0] {
        for y in bboxmin[1]..=bboxmax[1] {
            let bc_screen = barycentric(points, Vec2i::new2(x, y));
            if bc_screen[0] >= 0.0 && bc_screen[1] >= 0.0 && bc_screen[2] >= 0.0 {
                image.set(x as usize, y as usize, color);
            } else {
            }
        }
    }
}

fn line_vec(p0: Vec2i, p1: Vec2i, image: &mut tga::Image, color: tga::Color) {
    let [mut x0, mut y0] = p0.into_array();
    let [mut x1, mut y1] = p1.into_array();
    let steep = if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        true
    } else {
        false
    };
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    };
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
        error += derror;
        if error > dx {
            if y1 > y0 {
                y += 1
            } else {
                y -= 1
            };
            error -= dx * 2;
        }
    }
}

pub mod colors {
    use tga_image::Color;
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
}
