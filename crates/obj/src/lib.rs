use std::{
    fs,
    io::{self, BufRead},
    path::Path,
};

use geometry::Vec3f;

pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec<i32>>,
}

impl Model {
    pub fn from_file<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let path = filename.as_ref();
        let input = fs::File::open(path)?;
        let input = io::BufReader::new(input);

        let mut verts = vec![];
        let mut faces = vec![];
        for line in input.lines() {
            let line = line?;
            let line = line.as_str();

            let ws = line.find(' ');
            if let Some(index) = ws {
                let (prefix, rest) = line.split_at(index);
                match prefix {
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
                            verts.push(Vec3f::new(args[0], args[1], args[2]));
                        }
                    }
                    "f" => {
                        let args: Result<Vec<_>, _> = rest
                            .trim()
                            .split_ascii_whitespace()
                            .map(str::parse)
                            .collect();
                        let args =
                            args.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                        faces.push(args);
                    }
                    _ => {}
                }
            }
        }
        Ok(Self { verts, faces })
    }

    pub fn verts(&self) -> &[Vec3f] {
        &self.verts
    }

    pub fn faces(&self) -> &[Vec<i32>] {
        &self.faces
    }
}
