use clap::Parser;
use image::{imageops, GenericImageView};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    image: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    //image
    let mut image = image::open(args.image).unwrap();
    let (mut width, height) = image.dimensions();

    // output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut c = ColorSpec::new();

    // resize image to terminal width
    let tsize = termsize::get().ok_or("error");
    match tsize {
        Ok(size) => {
            let i = size.cols as u32;
            if width > i {
                println!("Resizing to {}x{}", i, height);
                image = image.resize(i, height, imageops::FilterType::Nearest);
                width = i;
            }
        }
        Err(_) => {}
    };

    // full block unicode character
    let chars = ["\u{2588}"];

    // write each pixel to screen as a colored \u
    let mut count = 0;
    for block in chars.iter() {
        println!("STARTING BLOCK {}", block);
        for p in image.pixels() {
            count = count + 1;
            c.set_fg(Some(Color::Rgb(p.2[0], p.2[1], p.2[2])))
                .set_bold(false);
            stdout.set_color(&c).unwrap();
            write!(&mut stdout, "{}", block).unwrap();

            // next line
            if count == width {
                writeln!(&mut stdout, "").unwrap();
                count = 0;
            }
        }
    }
}
