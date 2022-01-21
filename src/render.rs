use crate::floss::flosses::Floss;
use image::{png::PngEncoder, ColorType, Rgba, RgbaImage};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const SYMBOLS: [char; 100] = [
    // Upper alpha
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    // Lower alpha
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    // Geometric shapes
    '\u{2bc5}', '\u{2bc0}', '\u{2bc1}', '\u{2b1f}', '\u{2b23}', '\u{1f787}', '\u{1f78a}', '\u{1f791}', '\u{1f794}', '\u{1f79a}',
    // Arrows
    '\u{1f868}', '\u{1f86a}', '\u{1f869}', '\u{1f86b}', '\u{1f86c}', '\u{1f86d}', '\u{1f86f}', '\u{1f86e}',
    // Circled numbers
    '\u{2780}', '\u{2781}', '\u{2782}', '\u{2783}', '\u{2784}', '\u{2785}', '\u{2786}', '\u{2787}', '\u{2788}', '\u{2789}',
    // Card suits
    '\u{2663}', '\u{2666}', '\u{2665}', '\u{2660}',
    // Dice
    '\u{2680}', '\u{2681}', '\u{2682}', '\u{2683}', '\u{2684}', '\u{2685}',
    // Digits
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn render(
    reduced: RgbaImage,
    mut palette: Vec<Floss>,
    output_name: String,
) -> std::io::Result<()> {
    let mut color_counts: HashMap<Rgba<u8>, u32> = HashMap::new();
    for pixel in reduced.pixels() {
        let count = color_counts.entry(*pixel).or_default();
        *count += 1;
    }

    palette.retain(|floss| color_counts.contains_key(&floss.color));
    let color_indices: HashMap<Rgba<u8>, usize> = palette
        .iter()
        .enumerate()
        .map(|(i, floss)| (floss.color, i))
        .collect();

    let mut file = File::create(output_name)?;
    write!(
        file,
        "<html><head><title>Pattern</title><style type=\"text/css\">"
    )?;
    print_css(&mut file, &palette)?;
    write!(file, "</style></head><body>")?;
    print_image(&mut file, &reduced)?;
    print_palette(&mut file, &palette, color_counts)?;
    print_pattern(&mut file, &reduced, color_indices)?;
    write!(file, "</body></html>")?;

    Ok(())
}

fn print_css(file: &mut File, palette: &[Floss]) -> std::io::Result<()> {
    write!(file, "
html {{ font-size: 7pt; font-family: monospace; }}
table {{ border-collapse: collapse; }}
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
.printable .symbol-{0}::before,
#palette .symbol-{0}::before {{ content: '{4}'; }}",
            i,
            floss.color.0[0],
            floss.color.0[1],
            floss.color.0[2],
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

fn print_image(file: &mut File, reduced: &RgbaImage) -> std::io::Result<()> {
    let mut buffer = vec![];
    let encoder = PngEncoder::new(&mut buffer);
    encoder
        .encode(
            reduced.as_raw(),
            reduced.width(),
            reduced.height(),
            ColorType::Rgba8,
        )
        .unwrap();
    let b64 = base64::encode(buffer);
    writeln!(
        file,
        "<img src=\"data:image/png;base64,{}\" style=\"width: {}px; image-rendering: pixelated;\">",
        b64,
        reduced.width() * 5
    )?;

    Ok(())
}

fn print_palette(
    file: &mut File,
    palette: &[Floss],
    counts: HashMap<Rgba<u8>, u32>,
) -> std::io::Result<()> {
    write!(
        file,
        "
<div id=\"palette\">
<table>"
    )?;
    for (i, floss) in palette.iter().enumerate() {
        let count = counts.get(&floss.color).unwrap();
        let name = format!("#{0} {1}", floss.number, floss.name);

        write!(file, "
<tr><td><span class=\"color color-{0}\"></span></td><td><span class=\"symbol-{0}\"></span></td><td>{1}</td><td class=\"stitch-count\">{2}</td></tr>",
            i, name, count
            )?;
    }

    write!(file, "</table></div>")?;

    Ok(())
}

fn print_pattern(
    file: &mut File,
    reduced: &RgbaImage,
    color_indices: HashMap<Rgba<u8>, usize>,
) -> std::io::Result<()> {
    const BLOCK_SIZE: usize = 40;
    let x_block_count = ((reduced.width() as f32) / (BLOCK_SIZE as f32)).ceil() as usize;
    let y_block_count = ((reduced.height() as f32) / (BLOCK_SIZE as f32)).ceil() as usize;

    for y_block in 0..y_block_count {
        for x_block in 0..x_block_count {
            let block = reduced
                .rows()
                .skip(y_block * BLOCK_SIZE)
                .take(BLOCK_SIZE)
                .map(|c| c.skip(x_block * BLOCK_SIZE).take(BLOCK_SIZE));

            writeln!(file, "<h2>Block {},{}</h2>", x_block, y_block)?;
            writeln!(file, "<table class=\"printable\">")?;
            for row in block {
                writeln!(file, "<tr>")?;
                for color in row {
                    if let Some(index) = color_indices.get(color) {
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
    *SYMBOLS.get(n).unwrap_or(&'?')
}
