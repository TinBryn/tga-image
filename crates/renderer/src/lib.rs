use std::mem::swap;

use geometry::Vec2i;
use tga_image as tga;

pub trait Draw {
    fn draw_line(&mut self, p0: impl Into<Vec2i>, p1: impl Into<Vec2i>, color: tga::Color);
}

impl Draw for tga::Image {
    fn draw_line(&mut self, p0: impl Into<Vec2i>, p1: impl Into<Vec2i>, color: tga::Color) {
        line_vec(p0.into(), p1.into(), self, color);
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
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
}
