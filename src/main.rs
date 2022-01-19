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
use image::{io::Reader as ImageReader, RgbaImage};
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
    render(reduced, palette, img_rgba, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <input bitmap> <output html>");
}

fn reduce(num_colors: usize, img: &RgbaImage) -> (Vec<Option<usize>>, Vec<Floss>) {
    let all_floss = get_dmc_floss();
    let mut palette = vote(num_colors, img.pixels(), all_floss);

    palette.sort_by_cached_key(|floss| floss.number.parse::<i32>().ok());

    let mut reduced = Vec::new();
    let mut ditherer = Ditherer::new(img.width() as usize);
    for pixel in img.pixels() {
        if pixel.0[3] < 128u8 {
            reduced.push(None);
            continue;
        }

        let pixel_with_error = ditherer.apply_error(pixel);
        let palette_index = find_index_of_closest(&pixel_with_error, &palette);
        ditherer.record_error(pixel, &palette[palette_index].color);

        reduced.push(Some(palette_index));
        ditherer.next();
    }

    (reduced, palette)
}

fn find_index_of_closest(color: &image::Rgba<u8>, palette: &[Floss]) -> usize {
    palette
        .iter()
        .map(|floss| floss.color.distance(color))
        .enumerate()
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .unwrap()
        .0
}
