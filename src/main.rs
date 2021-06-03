extern crate rand;
extern crate rayon;

mod bitmap;
mod color;
mod floss;

use crate::bitmap::{Bmp, Pixel};
use crate::color::{Color, Rgb, RgbLinear};
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
    let (reduced, palette) = reduce::<RgbLinear>(num_colors, &bmp.pixels);
    render(reduced, palette, &bmp, output_name).unwrap();
}

fn print_usage() {
    println!("<color count> <input bitmap> <output html>");
}

fn reduce<C>(num_colors: usize, pixels: &Vec<Pixel>) -> (Vec<Option<usize>>, Vec<Floss>)
where
    C: Color + From<Rgb> + Sync + Send + 'static,
{
    let pixel_parts: Vec<C> = pixels.iter().map(|p| p.color.into()).collect();
    let all_floss = get_dmc_floss();
    let mut palette = vote(num_colors, &pixel_parts, all_floss);

    palette.sort_by_cached_key(|floss| floss.number.parse::<i32>().ok());

    let mut reduced = Vec::new();
    for pixel in pixels.iter() {
        if pixel.alpha < 128u8 {
            reduced.push(None);
            continue;
        }

        let color_luv: C = pixel.color.into();

        let index_of_closest = palette
            .iter()
            .map(|floss| C::from(floss.color).dist(&color_luv))
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
    write!(
        file,
        "<html><head><title>Pattern</title><style type=\"text/css\">"
    )?;
    print_css(&mut file, &palette)?;
    write!(file, "</style></head><body>")?;
    print_display_table(&mut file, &bmp, &reduced)?;
    print_palette(&mut file, &palette, counts)?;
    print_printable_table(&mut file, &bmp, &reduced)?;
    write!(file, "</body></html>")?;

    Ok(())
}

fn print_css(file: &mut File, palette: &Vec<Floss>) -> std::io::Result<()> {
    write!(file, "
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
.printable tr:nth-child(10n+10) > td {{ border-bottom: 2px solid #000; }}")?;
    for (i, floss) in palette.iter().enumerate() {
        write!(
            file,
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
        )?;
    }
    write!(
        file,
        "
#palette {{ font-size: 3em; }}
#palette .color {{ width: 3em; height: 1.5em; display: inline-block; border: 1px solid #000; }}
#palette td:nth-child(n+2) {{ padding-left: 0.5em; }}
.stitch-count {{ text-align: right; }}
h2 {{ page-break-before: always; }}"
    )?;

    Ok(())
}

fn print_display_table(
    file: &mut File,
    bmp: &Bmp,
    reduced: &Vec<Option<usize>>,
) -> std::io::Result<()> {
    writeln!(file, "<table class=\"display\">")?;
    let width = bmp.header.width as usize;
    for (i, palette_index) in reduced.iter().enumerate() {
        if i % width == 0 {
            writeln!(file, "<tr>")?;
        }
        if let Some(index) = palette_index {
            writeln!(file, "<td class=\"symbol-{}\"></td>", index)?;
        } else {
            writeln!(file, "<td></td>")?;
        }
        if i % width + 1 == width {
            writeln!(file, "</tr>")?;
        }
    }
    write!(file, "</table>")?;

    Ok(())
}

fn print_palette(file: &mut File, palette: &Vec<Floss>, counts: Vec<i32>) -> std::io::Result<()> {
    write!(
        file,
        "
<div id=\"palette\">
<table>"
    )?;
    for (i, (floss, count)) in palette.iter().zip(counts.iter()).enumerate() {
        let name = format!("#{0} {1}", floss.number, floss.name);

        write!(file, "
<tr><td><span class=\"color color-{0}\"></span></td><td><span class=\"symbol-{0}\"></span></td><td>{1}</td><td class=\"stitch-count\">{2}</td></tr>",
            i, name, count
            )?;
    }

    write!(file, "</table></div>")?;

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

            writeln!(file, "<h2>Block {},{}</h2>", x_block, y_block)?;
            writeln!(file, "<table class=\"printable\">")?;
            for row in block {
                writeln!(file, "<tr>")?;
                for color_index in row {
                    if let Some(index) = color_index {
                        writeln!(file, "<td class=\"symbol-{}\"></td>", index)?;
                    } else {
                        writeln!(file, "<td></td>")?;
                    }
                }
                writeln!(file, "</tr>")?;
            }
            writeln!(file, "</table>")?;
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
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ => '?',
    }
}
