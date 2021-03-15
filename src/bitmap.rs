use crate::color::Rgb;
use std::fs::File;
use std::io::prelude::*;

pub struct Bmp {
    pub header: Header,
    pub pixels: Vec<Pixel>,
}

pub struct Header {
    pub width: u32,
    pub height: u32,
    bits_per_pixel: u16,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Pixel {
    pub color: Rgb,
}

impl Bmp {
    pub fn new(file_path: String) -> Result<Self, String> {
        let mut bytes = Vec::new();
        let mut file = File::open(file_path).map_err(|_| "Unable to open file")?;
        file.read_to_end(&mut bytes)
            .map_err(|_| "Unable to read file")?;

        let mut head = bytes.iter();
        expect(&mut head, b'B')?;
        expect(&mut head, b'M')?;

        read_u32(&mut head)?; // Size
        read_u16(&mut head)?; // Unused
        read_u16(&mut head)?; // Unused
        let offset = read_u32(&mut head)?;
        let header = Header::parse(&mut head)?;

        let mut body = bytes.iter().skip(offset as usize);
        let pixels = read_pixels(&mut body, &header)?;

        Ok(Bmp {
            header: header,
            pixels: pixels,
        })
    }
}

impl Header {
    fn parse<'a, I>(iter: &mut I) -> Result<Self, &'static str>
    where
        I: Iterator<Item = &'a u8>,
    {
        const HEADER_V4_SIZE: u32 = 108;
        const HEADER_V5_SIZE: u32 = 124;
        let dib_size = read_u32(iter)?;
        match dib_size {
            HEADER_V4_SIZE => Header::parse_v4(iter),
            HEADER_V5_SIZE => Header::parse_v5(iter),
            _ => Err("Unrecognized DIB size"),
        }
    }

    fn parse_v4<'a, I>(iter: &mut I) -> Result<Self, &'static str>
    where
        I: Iterator<Item = &'a u8>,
    {
        let width = read_u32(iter)?;
        let height = read_u32(iter)?;
        read_u16(iter)?; // Color planes
        let bits_per_pixel = read_u16(iter)?;
        Ok(Header {
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
        })
    }

    fn parse_v5<'a, I>(iter: &mut I) -> Result<Self, &'static str>
    where
        I: Iterator<Item = &'a u8>,
    {
        let width = read_u32(iter)?;
        let height = read_u32(iter)?;
        read_u16(iter)?; // Color planes
        let bits_per_pixel = read_u16(iter)?;
        Ok(Header {
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
        })
    }
}

impl Pixel {
    fn parse<'a, I>(iter: &mut I, bits_per_pixel: u16) -> Result<Self, &'static str>
    where
        I: Iterator<Item = &'a u8>,
    {
        if bits_per_pixel != 24 && bits_per_pixel != 32 {
            return Err("bpp");
        }

        let blue = iter.next().ok_or("Blue")?;
        let green = iter.next().ok_or("Green")?;
        let red = iter.next().ok_or("Red")?;

        if bits_per_pixel == 32 {
            iter.next().ok_or("Extra")?;
        }

        Ok(Pixel {
            color: Rgb::new(*red, *green, *blue),
        })
    }
}

fn expect<'a, I>(iter: &mut I, expected: u8) -> Result<(), String>
where
    I: Iterator<Item = &'a u8>,
{
    match iter.next() {
        Some(v) => {
            if *v == expected {
                Ok(())
            } else {
                Err(format!("Expected '{:02x}', found '{:02x}'", expected, v))
            }
        }
        None => Err(String::from("Expected value, none found")),
    }
}

fn read_u32<'a, I>(iter: &mut I) -> Result<u32, &'static str>
where
    I: Iterator<Item = &'a u8>,
{
    let b0 = iter.next().ok_or("Foo")?;
    let b1 = iter.next().ok_or("Foo")?;
    let b2 = iter.next().ok_or("Foo")?;
    let b3 = iter.next().ok_or("Foo")?;
    Ok(u32::from_le_bytes([*b0, *b1, *b2, *b3]))
}

fn read_u16<'a, I>(iter: &mut I) -> Result<u16, &'static str>
where
    I: Iterator<Item = &'a u8>,
{
    let b0 = iter.next().ok_or("Foo")?;
    let b1 = iter.next().ok_or("Foo")?;
    Ok(u16::from_le_bytes([*b0, *b1]))
}

fn read_pixels<'a, I>(iter: &mut I, header: &Header) -> Result<Vec<Pixel>, &'static str>
where
    I: Iterator<Item = &'a u8>,
{
    let row_padding = (header.width * (header.bits_per_pixel as u32) / 8) % 4;

    let mut rows = Vec::with_capacity(header.height as usize);
    for _ in 0..header.height {
        let mut pixels = Vec::with_capacity(header.width as usize);
        for _ in 0..header.width {
            let pixel = Pixel::parse(iter, header.bits_per_pixel)?;
            pixels.push(pixel);
        }
        rows.push(pixels);
        for _ in 0..row_padding {
            iter.next();
        }
    }
    rows.reverse();
    Ok(rows.concat())
}
