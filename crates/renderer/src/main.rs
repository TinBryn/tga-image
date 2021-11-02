use std::mem::swap;

use tga_image as tga;

const WHITE: tga::Color = tga::Color::rgb(255, 255, 255);
const RED: tga::Color = tga::Color::rgb(255, 0, 0);

fn main() -> std::io::Result<()> {
    let mut image = tga::Image::new(100, 100, 3);

    line(13, 25, 80, 45, &mut image, WHITE);
    line(20, 13, 40, 80, &mut image, RED);
    line(80, 40, 13, 20, &mut image, RED);

    image.flip_vertically();
    image.write_tga_file("line1.tga", tga::Encoding::Rle)?;
    Ok(())
}

fn line(
    mut x0: isize,
    mut y0: isize,
    mut x1: isize,
    mut y1: isize,
    image: &mut tga::Image,
    color: tga::Color,
) {
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
    }

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
