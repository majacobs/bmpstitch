extern crate rand;

mod bitmap;
mod color;
mod floss;
mod kmeans;
mod misc;

use crate::bitmap::{Bmp, Pixel};
use crate::floss::algorithm::reduce_to_known;
use crate::floss::flosses::get_dmc_floss;
use crate::kmeans::run_kmeans;
use crate::misc::distance_u8;
use crate::color::Hsl;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
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

    let fitting_method: FittingMethod = match args[2].as_str() {
        "kmeans" => FittingMethod::Kmeans,
        "floss" => FittingMethod::FlossSelect,
        _ => {
            print_usage();
            return;
        }
    };

    let bitmap_name = args[3].to_string();
    let output_name = args[4].to_string();

    let mut bmp = Bmp::new(bitmap_name).unwrap();
    match fitting_method {
        FittingMethod::Kmeans => kmeans_reduce(num_colors, &mut bmp.pixels),
        FittingMethod::FlossSelect => floss_reduce(num_colors, &mut bmp.pixels),
    };
    render(&bmp, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <fitting method> <input bitmap> <output html>");
    println!("Fitting methods: kmeans, floss");
}

fn kmeans_reduce(num_colors: usize, pixels: &mut Vec<Pixel>) {
    let pixel_parts: Vec<_> = pixels.iter().map(|p| p.parts()).collect();
    let means = run_kmeans(num_colors, &pixel_parts);
    let colors: Vec<_> = means
        .iter()
        .map(|c| (c, Pixel::from(c.0, c.1, c.2)))
        .collect();

    for pixel in pixels.iter_mut() {
        let mut replacement = Pixel::from(255, 0, 255);
        let mut best_dist = std::f32::MAX;
        let parts = &pixel.parts();

        for (coords, color) in colors.iter() {
            let dist = distance_u8(coords, &parts);
            if dist > best_dist {
                continue;
            }

            best_dist = dist;
            replacement = color.clone();
        }

        *pixel = replacement;
    }
}

fn floss_reduce(num_colors: usize, pixels: &mut Vec<Pixel>) {
    let pixel_parts: Vec<Hsl> = pixels.iter().map(|p| p.color.into()).collect();
    let all_floss = get_dmc_floss();
    let chosen_floss = reduce_to_known(num_colors, &pixel_parts, all_floss);
    let colors: Vec<_> = chosen_floss
        .iter()
        .map(|f| {
            (
                (f.color.r, f.color.g, f.color.b),
                Pixel::from(f.color.r, f.color.g, f.color.b),
            )
        })
        .collect();

    for pixel in pixels.iter_mut() {
        let mut replacement = Pixel::from(255, 0, 255);
        let mut best_dist = std::f32::MAX;
        let parts = &pixel.parts();

        for (coords, color) in colors.iter() {
            let dist = distance_u8(coords, &parts);
            if dist > best_dist {
                continue;
            }

            best_dist = dist;
            replacement = color.clone();
        }

        *pixel = replacement;
    }
}

fn render(bmp: &Bmp, output_name: String) -> std::io::Result<()> {
    let map: HashSet<_> = bmp.pixels.iter().collect();
    let map: HashMap<_, _> = map.iter().enumerate().map(|(i, p)| (p, i)).collect();

    let mut file = File::create(output_name)?;
    file.write_fmt(format_args!("<html>
<head>
<title>Pattern</title>
<style type=\"text/css\">
html {{ font-size: 7pt; font-family: monospace; }}
table {{ border-collapse: collapse; }}
.display td {{ width: 0.4em; line-height: 0.4em; padding: 0; }}
.display tr {{ height: 0.4em; }}
.printable td {{ width: 1em; line-height: 1em; padding: 0; text-align: center; vertical-align: middle; }}
.printable tr {{ height: 1em; }}
.printable td {{ border: 1px solid #ccc; }}
.printable td:first-child {{ border-left: 2px solid #000; }}
.printable td:nth-child(10n+10) {{ border-right: 2px solid #000; }}
.printable tr:first-child > td {{ border-top: 2px solid #000; }}
.printable tr:nth-child(10n+10) > td {{ border-bottom: 2px solid #000; }}"))?;
    for (p, i) in map.iter() {
        file.write_fmt(format_args!(
            "
.color-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}; }}
table.printable .symbol-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}40; }}
table.display .symbol-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}; }}
.printable .symbol-{0}::before,
#palette .symbol-{0}::before {{ content: '{4}'; }}",
            i,
            p.color.r,
            p.color.g,
            p.color.b,
            symbol(*i)
        ))?;
    }
    file.write_fmt(format_args!(
        "
#palette {{ font-size: 3em; }}
/*#palette::after {{ clear: both; display: table; content: ''; }}
#palette > div {{ float: left; width: 4em; height: 2em; }}
#palette > div:nth-child(6n) {{ clear: left; }}
#palette > div * {{ vertical-align: top; }}*/
#palette .color {{ width: 3em; height: 1.5em; display: inline-block; border: 1px solid #000; }}
h2 {{ page-break-before: always; }}
</style>
</head>
<body>"
    ))?;
    print_display_table(&mut file, &bmp, &map)?;
    file.write_fmt(format_args!(
        "
<div id=\"palette\">
<table>"
    ))?;
    for i in 0..map.len() {
        file.write_fmt(format_args!("
<tr><td><span class=\"color color-{0}\"></span></td><td><span class=\"symbol-{0}\"></span></td></tr>",
            i
            ))?;
    }

    file.write_fmt(format_args!("</table></div>"))?;
    print_printable_table(&mut file, &bmp, &map)?;
    file.write_fmt(format_args!("</body></html>"))?;

    Ok(())
}

fn print_display_table(
    file: &mut File,
    bmp: &Bmp,
    map: &HashMap<&&Pixel, usize>,
) -> std::io::Result<()> {
    file.write_fmt(format_args!("<table class=\"display\">\n"))?;
    let width = bmp.header.width as usize;
    for (i, pixel) in bmp.pixels.iter().enumerate() {
        if i % width == 0 {
            file.write_fmt(format_args!("<tr>\n"))?;
        }
        let color_index = map.get(&pixel).unwrap();
        file.write_fmt(format_args!("<td class=\"symbol-{}\"></td>\n", color_index))?;
        if i % width + 1 == width {
            file.write_fmt(format_args!("</tr>\n"))?;
        }
    }
    file.write_fmt(format_args!("</table>"))?;

    Ok(())
}

fn print_printable_table(
    file: &mut File,
    bmp: &Bmp,
    map: &HashMap<&&Pixel, usize>,
) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 40;
    let width = bmp.header.width as usize;
    let x_block_count = ((bmp.header.width as f32) / (BLOCK_SIZE as f32)).ceil() as usize;
    let y_block_count = ((bmp.header.height as f32) / (BLOCK_SIZE as f32)).ceil() as usize;

    for y_block in 0..y_block_count {
        for x_block in 0..x_block_count {
            let block = bmp
                .pixels
                .chunks(width)
                .skip(y_block * BLOCK_SIZE)
                .take(BLOCK_SIZE)
                .map(|c| c.iter().skip(x_block * BLOCK_SIZE).take(BLOCK_SIZE));

            file.write_fmt(format_args!("<h2>Block {},{}</h2>\n", x_block, y_block))?;
            file.write_fmt(format_args!("<table class=\"printable\">\n"))?;
            for row in block {
                file.write_fmt(format_args!("<tr>\n"))?;
                for pixel in row {
                    let color_index = map.get(&pixel).unwrap();
                    file.write_fmt(format_args!("<td class=\"symbol-{}\"></td>\n", color_index))?;
                }
                file.write_fmt(format_args!("</tr>\n"))?;
            }
            file.write_fmt(format_args!("</table>\n"))?;
        }
    }

    Ok(())
}

fn symbol(n: usize) -> char {
    match n {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'H',
        5 => 'I',
        6 => 'J',
        7 => 'K',
        8 => 'L',
        9 => 'M',
        10 => 'O',
        11 => 'P',
        12 => 'S',
        13 => 'T',
        14 => 'U',
        15 => 'V',
        16 => 'X',
        17 => 'Y',
        18 => 'Z',
        _ => '?',
    }
}

enum FittingMethod {
    Kmeans,
    FlossSelect,
}
