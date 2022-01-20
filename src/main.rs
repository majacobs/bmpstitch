extern crate base64;
extern crate image;
extern crate rand;
extern crate rayon;

mod color;
mod dithering;
mod floss;
mod render;

use crate::color::Distance;
use crate::dithering::Ditherer;
use crate::floss::flosses::{get_dmc_floss, Floss};
use crate::floss::rcv::vote;
use crate::render::render;
use image::{io::Reader as ImageReader, Rgba, RgbaImage};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return;
    }

    let num_colors: usize = match args[1].parse() {
        Ok(n) => n,
        _ => {
            print_usage();
            return;
        }
    };

    let bitmap_name = args[2].to_string();
    let output_name = args[3].to_string();

    let img = ImageReader::open(bitmap_name)
        .expect("Unable to open image")
        .decode()
        .expect("Unable to decode image");
    let img_rgba = img.as_rgba8().expect("Image is not RGBA");
    let (reduced, palette) = reduce(num_colors, img_rgba);
    render(reduced, palette, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <input bitmap> <output html>");
}

fn reduce(num_colors: usize, img: &RgbaImage) -> (RgbaImage, Vec<Floss>) {
    let all_floss = get_dmc_floss();
    let mut palette = vote(num_colors, img.pixels(), all_floss);

    palette.sort_by_cached_key(|floss| floss.number.parse::<i32>().ok());

    let transparent = Rgba::<u8>([0, 0, 0, 0]);
    let mut reduced = RgbaImage::new(img.width(), img.height());
    let mut ditherer = Ditherer::new(img.width() as usize);
    for (x, y, pixel) in img.enumerate_pixels() {
        if pixel.0[3] < 128u8 {
            reduced.put_pixel(x, y, transparent);
            continue;
        }

        let pixel_with_error = ditherer.apply_error(pixel);
        let closest = find_closest(&pixel_with_error, &palette);
        ditherer.record_error(pixel, closest);

        reduced.put_pixel(x, y, *closest);
        ditherer.next();
    }

    (reduced, palette)
}

fn find_closest<'a>(color: &Rgba<u8>, palette: &'a [Floss]) -> &'a Rgba<u8> {
    let index = palette
        .iter()
        .map(|floss| floss.color.distance(color))
        .enumerate()
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .unwrap()
        .0;
    &palette[index].color
}
