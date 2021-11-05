use geometry::Vec3f;
use tga_image as tga;

fn main() {
    let model = obj::Model::from_file("objs/african_head.obj").unwrap();

    let faces = model.faces();
    let verts = model.verts();

    let width = 800;
    let height = 800;

    let mut image = tga::Image::new(width, height, 3);
    let mut z_buffer = vec![0f32; width * height];

    let light_dir = Vec3f::new3(0.0, 0.0, -1.0);

    for face in faces {
        let mut screen_coords = [Vec3f::default(); 3];
        let mut world_coords = [Vec3f::default(); 3];
        for j in 0..3 {
            let v = verts[face[j]];
            screen_coords[j] = Vec3f::new3(
                (v[0] + 1.0) * width as f32 * 0.5,
                (v[1] + 1.0) * height as f32 * 0.5,
                (v[2] + 1.0) * 0.5,
            );
            world_coords[j] = v;
        }

        let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]);
        let n = n * (1.0 / n.len());
        let intensity = n.dot(light_dir);
        if intensity > 0.0 {
            let v = (intensity * 255.0) as u8;
            let color = tga::Color::rgb(v, v, v);
            renderer::triangle_with_depth_test(&mut image, screen_coords, &mut z_buffer, color);
        }
    }
    image
        .write_tga_file("flat_shading.tga", tga::Encoding::Rle)
        .unwrap();
}
