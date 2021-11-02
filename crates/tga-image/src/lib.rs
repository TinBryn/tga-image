#![warn(missing_docs)]

//! # TGA Image
//!
//! utility for loading [Truevision TGA](https://en.wikipedia.org/wiki/Truevision_TGA) images.
//! Provide reading and writing of the format via `std::io::Read` and `std::io::Write` and
//! returns the relevant `io::Result`

use std::{
    fs,
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
        // Safety: the header has a C, packed representation
        unsafe { std::ptr::read(buf.as_ptr() as *const _) }
    }

    fn into_buffer(self) -> [u8; 18] {
        // Safety: the header has a C, packed representation
        unsafe { std::ptr::read(&self as *const Self as *const _) }
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

/// # The color of a single pixel
///
/// TGAs can have multiple formats, currently only a few are supported
///
/// - Greyscale
/// - RGB
/// - RGBA
///
/// other formats will either result in an `io::ErrorKind::InvalidData` or have unexpected results
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    bgra: [u8; 4],
    bytes_pp: usize,
}

impl Color {
    /// Creates a new grey scale color
    pub const fn grey_scale(v: u8) -> Self {
        Self {
            bgra: [v, 0, 0, 0],
            bytes_pp: 1,
        }
    }

    /// Creates a new RGB color
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            bgra: [b, g, r, 0],
            bytes_pp: 3,
        }
    }

    /// Creates a new RGBA color
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            bgra: [b, g, r, a],
            bytes_pp: 4,
        }
    }

    const fn try_from_slice(slice: &[u8]) -> Option<Self> {
        match *slice {
            [v] => Some(Self::grey_scale(v)),
            [b, g, r] => Some(Self::rgb(r, g, b)),
            [b, g, r, a] => Some(Self::rgba(r, g, b, a)),
            _ => None,
        }
    }

    fn as_slice(&self) -> &[u8] {
        &self.bgra[..self.bytes_pp]
    }
}

/// The encoding style for saving an image
pub enum Encoding {
    /// - Rle: run length encode the pixels, bad for natural images, good for
    /// images with large areas of the same color
    Rle,
}

/// # The in memory representation of an image
///
/// This structure allows some basic image manipulation such as flipping vertically and
/// horizontally, as well as single pixel manipulation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Image {
    width: usize,
    height: usize,
    bytes_pp: usize,
    data: Vec<u8>,
}

impl Image {
    /// Creates a new blank image
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
            input.read_exact(std::slice::from_mut(&mut chunk_header))?;
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

    fn save_rle_data<W: io::Write>(&self, mut output: W) -> io::Result<()> {
        const MAX_CHUNK_LENGTH: usize = 128;
        let n_pixels = self.width * self.height;
        let mut current_pixel = 0;

        while current_pixel < n_pixels {
            let chunk_start = current_pixel * self.bytes_pp;
            let mut current_byte = current_pixel * self.bytes_pp;
            let mut run_length = 1;
            let mut raw = true;

            while current_pixel + run_length < n_pixels && run_length < MAX_CHUNK_LENGTH {
                let mut succ_eq = true;
                let mut t = 0;
                while succ_eq && t < self.bytes_pp {
                    succ_eq =
                        self.data[current_byte + t] == self.data[current_byte + t + self.bytes_pp];
                    t += 1;
                }

                current_byte += self.bytes_pp;

                if run_length == 1 {
                    raw = !succ_eq;
                }

                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }

                if !raw && !succ_eq {
                    break;
                }

                run_length += 1;
            }

            current_pixel += run_length;
            let buf = if raw {
                [run_length as u8 - 1]
            } else {
                [run_length as u8 + 127]
            };

            output.write_all(&buf)?;
            if raw {
                output.write_all(&self.data[chunk_start..][..(run_length * self.bytes_pp)])?;
            } else {
                output.write_all(&self.data[chunk_start..][..self.bytes_pp])?;
            }
        }

        Ok(())
    }

    /// # Reads an image from a file
    ///
    /// ## errors
    ///
    /// - while opening the file see [https://doc.rust-lang.org/std/fs/struct.File.html#errors]
    /// - while reading the file see [`from_reader`]
    pub fn read_tga_file<P: AsRef<Path>>(filename: P) -> io::Result<Self> {
        let file = std::fs::File::open(filename.as_ref())?;

        Self::from_reader(file)
    }

    /// # Writes an image to a file
    ///
    /// ## errors
    ///
    /// - while creating the file see [https://doc.rust-lang.org/std/fs/struct.File.html#errors]
    pub fn write_tga_file<P: AsRef<Path>, E: Into<Option<Encoding>>>(
        &self,
        filename: P,
        encoding: E,
    ) -> io::Result<()> {
        let mut file = fs::File::create(filename.as_ref())?;
        let developer_area_ref = [0, 0, 0, 0];
        let extension_area_ref = [0, 0, 0, 0];
        let footer = b"TRUEVISION-XFILE.\0";

        let encoding = encoding.into();

        let data_type_code = match self.bytes_pp {
            1 => match encoding {
                Some(Encoding::Rle) => 11,
                None => 3,
            },
            _ => match encoding {
                Some(Encoding::Rle) => 10,
                None => 2,
            },
        };

        eprintln!("{:x}", data_type_code);

        let header = TgaHeader {
            data_type_code,
            width: self.width as i16,
            height: self.height as i16,
            bits_per_pixel: self.bytes_pp as u8 * 8,
            image_descriptor: 0x20,
            ..Default::default()
        };

        eprintln!("{:#x?}", header);

        let header = header.into_buffer();

        eprintln!("{:x?}", header);

        file.write_all(&header)?;

        match encoding {
            None => file.write_all(&self.data)?,
            Some(Encoding::Rle) => self.save_rle_data(&mut file)?,
        }

        file.write_all(&developer_area_ref)?;
        file.write_all(&extension_area_ref)?;
        file.write_all(footer)?;

        Ok(())
    }

    /// Turns the image into its mirror image along the horizontal axis,
    /// this occurs in place
    pub fn flip_horizontally(&mut self) {
        let half = self.width / 2;
        for i in 0..half {
            for j in 0..self.height {
                let c1 = self.get(i, j).unwrap();
                let c2 = self.get(self.width - 1 - i, j).unwrap();
                self.set(i, j, c2);
                self.set(self.width - 1 - i, j, c1);
            }
        }
    }

    /// Turns the image into its mirror image along the vertical axis.
    pub fn flip_vertically(&mut self) {
        let half = self.height / 2;
        for i in 0..self.width {
            for j in 0..half {
                let c1 = self.get(i, j).unwrap();
                let c2 = self.get(i, self.height - 1 - j).unwrap();
                self.set(i, j, c2);
                self.set(i, self.height - 1 - j, c1);
            }
        }
    }

    /// Changes the width and height of the image
    ///
    /// ??? it may stretch the image, this is not yet implemented
    #[deprecated(note = "Not implemented yet")]
    pub fn scale(&mut self, width: usize, height: usize) -> bool {
        if width == 0 || height == 0 {
            return false;
        }

        todo!()
    }

    /// Access a single pixel within the image
    ///
    /// ## returns
    ///
    /// - Some(Color) if the indicies are inside the image area
    /// - None otherwise
    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Color::try_from_slice(&self.data[((x + y * self.width) * self.bytes_pp)..][..self.bytes_pp])
    }

    /// Sets a single pixel to a color
    ///
    /// ## returns
    ///
    /// - true if this color matches the format of this image and is inside the image
    /// - false otherwise
    ///
    pub fn set(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let slice = color.as_slice();

        if slice.len() != self.bytes_pp {
            return false;
        }

        self.data[(x + y * self.width) * self.bytes_pp..][..self.bytes_pp].clone_from_slice(slice);

        true
    }

    /// Getter for the width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Getter for the height
    pub fn height(&self) -> usize {
        self.height
    }

    /// Getter for the pixel format represented as bytes per pixel
    pub fn format(&self) -> usize {
        self.bytes_pp
    }

    /// represents the raw data as an immutable slice
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    /// Represents the raw data as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    /// Sets the data of this image to all 0s, this is usually black, or fully transparent black
    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|b| *b = 0);
    }

    /// Reads an image from an `io::Read`
    ///
    /// ## Errors
    ///
    /// - see [https://doc.rust-lang.org/std/fs/struct.File.html#errors]
    /// - if the format isn't supported and `io::Error` of kind `io::ErrorKind::InvalidData`
    ///  is returned
    pub fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, io::Error> {
        let header = TgaHeader::from_reader(&mut reader)?;

        let width = header.width as usize;
        let height = header.height as usize;
        let bytes_pp = header.bits_per_pixel as usize / 8;
        let n_bytes = bytes_pp * width * height;
        let data = match header.data_type_code {
            3 | 2 => {
                let mut data = vec![0u8; n_bytes];
                reader.read_exact(&mut data)?;
                data
            }
            11 | 10 => {
                let mut data = vec![0u8; n_bytes];
                Self::load_rle_data(&mut reader, bytes_pp, &mut data)?;
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
