use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use geometry::Vec3f;

#[derive(Debug, Default)]
struct Verts {
    coords: Vec<Vec3f>,
    faces: Vec<Vec<usize>>,
}
pub struct Model {
    verts: Verts,
}

impl Model {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let path = filename.as_ref();
        let input = fs::File::open(path)?;
        let input = io::BufReader::new(input);

        let mut verts = Verts::default();

        for line in input.lines() {
            let line = line?;
            let line = line.as_str();

            let ws = line.find(' ');
            if let Some(index) = ws {
                let (prefix, rest) = line.split_at(index);
                match prefix.trim() {
                    "v" => {
                        let args: Result<Vec<_>, _> = rest
                            .trim()
                            .split_ascii_whitespace()
                            .map(str::parse)
                            .take(3)
                            .collect();
                        let args =
                            args.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                        if args.len() == 3 {
                            verts.coords.push(Vec3f::new3(args[0], args[1], args[2]));
                        }
                    }
                    "f" => {
                        let args: Result<Vec<Vec<_>>, _> = rest
                            .trim()
                            .split_ascii_whitespace()
                            .map(|v| {
                                v.split('/')
                                    .take(3)
                                    .map(str::parse)
                                    .map(|r| r.map(|i: usize| i - 1))
                                    .collect()
                            })
                            .collect();
                        let args =
                            args.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                        verts.faces.push(vec![args[0][0], args[1][0], args[2][0]]);
                    }
                    _ => {}
                }
            }
        }
        Ok(Self { verts })
    }

    pub fn verts(&self) -> &[Vec3f] {
        &self.verts.coords
    }

    pub fn faces(&self) -> &[Vec<usize>] {
        &self.verts.faces
    }
}
