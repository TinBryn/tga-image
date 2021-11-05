use renderer::{colors::*, Draw};
use tga_image as tga;

fn main() {
    let model = obj::Model::from_file("objs/african_head.obj").unwrap();

    let faces = model.faces();
    let verts = model.verts();

    let width = 800;
    let height = 800;

    let mut image = tga::Image::new(width, height, 3);

    for face in faces {
        for i in 0..face.len() {
            let v0 = verts[face[i]];
            let v1 = verts[face[(i + 1) % face.len()]];
            let x0 = ((v0[0] + 1.0) * width as f32 * 0.5) as isize;
            let y0 = ((v0[1] + 1.0) * height as f32 * 0.5) as isize;
            let x1 = ((v1[0] + 1.0) * width as f32 * 0.5) as isize;
            let y1 = ((v1[1] + 1.0) * height as f32 * 0.5) as isize;
            image.draw_line([x0, y0], [x1, y1], WHITE);
        }
    }
    image
        .write_tga_file("wireframe.tga", tga::Encoding::Rle)
        .unwrap();
}
