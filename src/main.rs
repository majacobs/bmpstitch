extern crate rand;

mod bitmap;
mod color;
mod floss;

use crate::bitmap::{Bmp, Pixel};
use crate::color::{Color, Hsl};
use crate::floss::algorithm::reduce_to_known;
use crate::floss::flosses::{get_dmc_floss, Floss};
use crate::floss::rcv::vote;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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
    let (reduced, palette) = reduce(num_colors, &bmp.pixels);
    render(reduced, palette, &bmp, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <input bitmap> <output html>");
}

const USE_VOTING: bool = true;

fn reduce(num_colors: usize, pixels: &Vec<Pixel>) -> (Vec<Option<usize>>, Vec<Floss>) {
    let pixel_parts: Vec<Hsl> = pixels.iter().map(|p| p.color.into()).collect();
    let all_floss = get_dmc_floss();
    let palette = if USE_VOTING {
        vote(num_colors, &pixel_parts, all_floss)
    } else {
        reduce_to_known(num_colors, &pixel_parts, all_floss)
    };

    let mut reduced = Vec::new();
    for pixel in pixels.iter() {
        if pixel.alpha < 128u8 {
            reduced.push(None);
            continue;
        }

        let index_of_closest = palette
            .iter()
            .map(|floss| floss.color.dist(&pixel.color))
            .enumerate()
            .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap())
            .unwrap()
            .0;

        reduced.push(Some(index_of_closest));
    }

    (reduced, palette)
}

fn render(
    reduced: Vec<Option<usize>>,
    palette: Vec<Floss>,
    bmp: &Bmp,
    output_name: String,
) -> std::io::Result<()> {
    let mut counts: Vec<i32> = palette.iter().map(|_| 0).collect();
    for &palette_index in reduced.iter() {
        if let Some(index) = palette_index {
            counts[index] += 1;
        }
    }

    let mut file = File::create(output_name)?;
    file.write_fmt(format_args!(
        "<html><head><title>Pattern</title><style type=\"text/css\">"
    ))?;
    print_css(&mut file, &palette)?;
    file.write_fmt(format_args!("</style></head><body>"))?;
    print_display_table(&mut file, &bmp, &reduced)?;
    print_palette(&mut file, &palette, counts)?;
    print_printable_table(&mut file, &bmp, &reduced)?;
    file.write_fmt(format_args!("</body></html>"))?;

    Ok(())
}

fn print_css(file: &mut File, palette: &Vec<Floss>) -> std::io::Result<()> {
    file.write_fmt(format_args!("
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
    for (i, floss) in palette.iter().enumerate() {
        file.write_fmt(format_args!(
            "

.color-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}; }}
table.printable .symbol-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}40; }}
table.display .symbol-{0} {{ background-color: #{1:02x}{2:02x}{3:02x}; }}
.printable .symbol-{0}::before,
#palette .symbol-{0}::before {{ content: '{4}'; }}",
            i,
            floss.color.r,
            floss.color.g,
            floss.color.b,
            symbol(i)
        ))?;
    }
    file.write_fmt(format_args!(
        "
#palette {{ font-size: 3em; }}
#palette .color {{ width: 3em; height: 1.5em; display: inline-block; border: 1px solid #000; }}
#palette td:nth-child(n+2) {{ padding-left: 0.5em; }}
.stitch-count {{ text-align: right; }}
h2 {{ page-break-before: always; }}"
    ))?;

    Ok(())
}

fn print_display_table(
    file: &mut File,
    bmp: &Bmp,
    reduced: &Vec<Option<usize>>,
) -> std::io::Result<()> {
    file.write_fmt(format_args!("<table class=\"display\">\n"))?;
    let width = bmp.header.width as usize;
    for (i, palette_index) in reduced.iter().enumerate() {
        if i % width == 0 {
            file.write_fmt(format_args!("<tr>\n"))?;
        }
        if let Some(index) = palette_index {
            file.write_fmt(format_args!("<td class=\"symbol-{}\"></td>\n", index))?;
        } else {
            file.write_fmt(format_args!("<td></td>\n",))?;
        }
        if i % width + 1 == width {
            file.write_fmt(format_args!("</tr>\n"))?;
        }
    }
    file.write_fmt(format_args!("</table>"))?;

    Ok(())
}

fn print_palette(file: &mut File, palette: &Vec<Floss>, counts: Vec<i32>) -> std::io::Result<()> {
    file.write_fmt(format_args!(
        "
<div id=\"palette\">
<table>"
    ))?;
    for (i, (floss, count)) in palette.iter().zip(counts.iter()).enumerate() {
        let name = format!("#{0} {1}", floss.number, floss.name);

        file.write_fmt(format_args!("
<tr><td><span class=\"color color-{0}\"></span></td><td><span class=\"symbol-{0}\"></span></td><td>{1}</td><td class=\"stitch-count\">{2}</td></tr>",
            i, name, count
            ))?;
    }

    file.write_fmt(format_args!("</table></div>"))?;

    Ok(())
}

fn print_printable_table(
    file: &mut File,
    bmp: &Bmp,
    reduced: &Vec<Option<usize>>,
) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 40;
    let width = bmp.header.width as usize;
    let x_block_count = ((bmp.header.width as f32) / (BLOCK_SIZE as f32)).ceil() as usize;
    let y_block_count = ((bmp.header.height as f32) / (BLOCK_SIZE as f32)).ceil() as usize;

    for y_block in 0..y_block_count {
        for x_block in 0..x_block_count {
            let block = reduced
                .chunks(width)
                .skip(y_block * BLOCK_SIZE)
                .take(BLOCK_SIZE)
                .map(|c| c.iter().skip(x_block * BLOCK_SIZE).take(BLOCK_SIZE));

            file.write_fmt(format_args!("<h2>Block {},{}</h2>\n", x_block, y_block))?;
            file.write_fmt(format_args!("<table class=\"printable\">\n"))?;
            for row in block {
                file.write_fmt(format_args!("<tr>\n"))?;
                for color_index in row {
                    if let Some(index) = color_index {
                        file.write_fmt(format_args!("<td class=\"symbol-{}\"></td>\n", index))?;
                    } else {
                        file.write_fmt(format_args!("<td></td>\n"))?;
                    }
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
        4 => 'E',
        5 => 'H',
        6 => 'I',
        7 => 'J',
        8 => 'K',
        9 => 'L',
        10 => 'M',
        11 => 'O',
        12 => 'P',
        13 => 'S',
        14 => 'T',
        15 => 'U',
        16 => 'V',
        17 => 'X',
        18 => 'Y',
        19 => 'Z',
        _ => '?',
    }
}
