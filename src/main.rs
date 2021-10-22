extern crate rand;
extern crate rayon;

mod bitmap;
mod color;
mod dithering;
mod floss;
mod render;

use crate::bitmap::{Bmp, Pixel};
use crate::color::{Color, Rgb, RgbLinear};
use crate::dithering::Ditherer;
use crate::floss::flosses::{get_dmc_floss, Floss};
use crate::floss::rcv::vote;
use crate::render::render;
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

    let bmp = Bmp::new(bitmap_name).unwrap();
    let (reduced, palette) = reduce::<RgbLinear>(num_colors, &bmp.pixels, bmp.header.width);
    render(reduced, palette, &bmp, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <input bitmap> <output html>");
}

fn reduce<C>(num_colors: usize, pixels: &[Pixel], width: u32) -> (Vec<Option<usize>>, Vec<Floss>)
where
    C: Color + From<Rgb> + Sync + Send + 'static,
{
    let pixel_parts: Vec<C> = pixels.iter().map(|p| p.color.into()).collect();
    let all_floss = get_dmc_floss();
    let mut palette = vote(num_colors, &pixel_parts, all_floss);

    palette.sort_by_cached_key(|floss| floss.number.parse::<i32>().ok());

    let mut reduced = Vec::new();
    let mut ditherer = Ditherer::new(width as usize);
    for pixel in pixels.iter() {
        if pixel.alpha < 128u8 {
            reduced.push(None);
            continue;
        }

        let color_c: C = ditherer.apply_error(&pixel.color).into();
        let palette_index = find_index_of_closest(&color_c, &palette);
        ditherer.record_error(&pixel.color, &palette[palette_index].color);

        reduced.push(Some(palette_index));
        ditherer.next();
    }

    (reduced, palette)
}

fn find_index_of_closest<'a, C>(color: &C, palette: &[Floss<'a>]) -> usize
where
    C: Color + From<Rgb> + Sync + Send + 'static,
{
    palette
        .iter()
        .map(|floss| C::from(floss.color).dist(color))
        .enumerate()
        .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
        .unwrap()
        .0
}
