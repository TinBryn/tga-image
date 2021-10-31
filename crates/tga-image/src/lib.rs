use core::slice;
use std::{
    io::{self, Write},
    path::Path,
};

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct TgaHeader {
    // number of bytes in the image ID field
    id_len: u8,

    // 0 if the image has no color map
    // 1 if present
    // 2-127 is reservered
    // 128-255 available for developer use
    color_map_type: u8,

    data_type_code: u8,

    // the origin is the index of the first color map entry
    // the length is the number of entries in the color map
    // the depth is the number of bits per pixel
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,

    // image specification
    // the x,y coordinates of the lower left corner
    // width and height
    // number of bits per pixel
    // descriptor: bits 3-0 give the alpha depth, 5-4 give direction
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TgaHeader {
    fn from_reader<R: io::Read>(mut reader: R) -> io::Result<Self> {
        const HEADER_SIZE: usize = std::mem::size_of::<TgaHeader>();
        let mut buf = [0u8; HEADER_SIZE];

        reader.read_exact(&mut buf)?;

        Self::from_buffer(buf).validate()
    }

    fn from_buffer(buf: [u8; 18]) -> Self {
        fn u16_from_2_u8(lower: u8, higher: u8) -> u16 {
            lower as u16 + ((higher as u16) << 8)
        }
        Self {
            id_len: buf[0],
            color_map_type: buf[1],
            data_type_code: buf[2],
            color_map_origin: u16_from_2_u8(buf[3], buf[4]),
            color_map_length: u16_from_2_u8(buf[5], buf[6]),
            color_map_depth: buf[7],
            x_origin: u16_from_2_u8(buf[8], buf[9]) as i16,
            y_origin: u16_from_2_u8(buf[10], buf[11]) as i16,
            width: u16_from_2_u8(buf[12], buf[13]) as i16,
            height: u16_from_2_u8(buf[14], buf[15]) as i16,
            bits_per_pixel: buf[16],
            image_descriptor: buf[17],
        }
    }

    fn validate(self) -> io::Result<Self> {
        if self.width <= 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "width is too low",
            ));
        }

        if self.height <= 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "height is too low",
            ));
        }

        if let 8 | 24 | 32 = self.bits_per_pixel {
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "unsupported bits per pixel",
            ));
        }

        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TgaColor {
    Empty,
    GreyScale(u8),
    Rgb { bgr: [u8; 3] },
    Rgba { bgra: [u8; 4] },
}

impl Default for TgaColor {
    fn default() -> Self {
        Self::Empty
    }
}

impl TgaColor {
    pub fn grey_scale(v: u8) -> Self {
        Self::GreyScale(v)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { bgr: [b, g, r] }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Rgba { bgra: [b, g, r, a] }
    }

    pub fn try_from_slice(slice: &[u8]) -> Option<Self> {
        match slice {
            [] => Some(Self::Empty),
            &[v] => Some(Self::GreyScale(v)),
            &[r, g, b] => Some(Self::Rgb { bgr: [b, g, r] }),
            &[r, g, b, a] => Some(Self::Rgba { bgra: [b, g, r, a] }),
            _ => None,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            TgaColor::Empty => &[],
            TgaColor::GreyScale(v) => slice::from_ref(v),
            TgaColor::Rgb { bgr } => bgr,
            TgaColor::Rgba { bgra } => bgra,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TagImage {
    width: usize,
    height: usize,
    bytes_pp: usize,
    data: Vec<u8>,
}

impl TagImage {
    pub fn new(width: usize, height: usize, bytes_pp: usize) -> Self {
        let n_bytes = width * height * bytes_pp;
        let data = vec![0u8; n_bytes];
        Self {
            width,
            height,
            bytes_pp,
            data,
        }
    }

    fn load_rle_data<R: io::Read>(
        mut input: R,
        bytes_pp: usize,
        mut data: &mut [u8],
    ) -> io::Result<()> {
        let mut buf = [0u8; 4];
        let buf = &mut buf[..bytes_pp];
        let mut chunk_header = 0;
        loop {
            input.read_exact(slice::from_mut(&mut chunk_header))?;
            let chunk_header = chunk_header;
            if chunk_header < 128 {
                for _ in 0..=chunk_header {
                    input.read_exact(buf)?;
                    data.write_all(buf)?;
                }
            } else {
                input.read_exact(buf)?;
                for _ in 0..(chunk_header - 127) {
                    data.write_all(buf)?;
                }
            };

            if data.is_empty() {
                return Ok(());
            }
        }
    }

    fn unload_rle_data<W: io::Write>(&self, output: W) -> io::Result<()> {
        todo!()
    }

    pub fn read_tga_file<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let file = std::fs::File::open(filename.as_ref())?;

        Self::read_tga(file)
    }

    pub fn write_tga_file<P: AsRef<Path>>(&self, filename: P, rle: bool) -> io::Result<()> {
        todo!()
    }

    pub fn flip_horizontally(&mut self) -> bool {
        todo!()
    }

    pub fn flip_vertically(&mut self) -> bool {
        todo!()
    }

    pub fn scale(&mut self, width: usize, height: usize) -> bool {
        todo!()
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TgaColor> {
        if x >= self.width || y >= self.height {
            return None;
        }
        TgaColor::try_from_slice(
            &self.data[((x + y * self.width) * self.bytes_pp)..][..self.bytes_pp],
        )
    }

    pub fn set(&mut self, x: usize, y: usize, color: TgaColor) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let slice = color.as_slice();

        if slice.len() != self.bytes_pp {
            return false;
        }

        self.data[(x + y * self.width * self.bytes_pp)..][..self.bytes_pp].clone_from_slice(slice);

        true
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn format(&self) -> usize {
        self.bytes_pp
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    fn read_tga<R: io::Read>(mut file: R) -> Result<TagImage, io::Error> {
        let header = TgaHeader::from_reader(&mut file)?;

        let width = header.width as usize;
        let height = header.height as usize;
        let bytes_pp = header.bits_per_pixel as usize / 8;
        let n_bytes = bytes_pp * width * height;
        let data = match header.data_type_code {
            3 | 2 => {
                let mut data = vec![0u8; n_bytes];
                file.read_exact(&mut data)?;
                data
            }
            11 | 10 => {
                let mut data = vec![0u8; n_bytes];
                Self::load_rle_data(&mut file, bytes_pp, &mut data)?;
                data
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown file format {:x}", header.data_type_code),
                ))
            }
        };
        let mut result = Self {
            width,
            height,
            bytes_pp,
            data,
        };
        if (header.image_descriptor & 0x20) == 0 {
            result.flip_vertically();
        }
        if (header.image_descriptor & 0x10) != 0 {
            result.flip_horizontally();
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn header_size() {
        let size = std::mem::size_of::<super::TgaHeader>();

        assert_eq!(size, 18);
    }
}
