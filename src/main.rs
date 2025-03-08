use clap::Parser;
use image::{imageops, GenericImageView};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    image: std::path::PathBuf,

    //#[arg(short, long)]
    //height: Option<u32>,
    #[arg(short, long)]
    width: Option<u32>,
}

fn main() {
    let args = Args::parse();

    //image
    let mut image = image::open(args.image).unwrap();
    let (mut image_width, image_height) = image.dimensions();

    // output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut c = ColorSpec::new();

    if args.width.as_ref() == None {
        // resize image to terminal width
        let tsize = termsize::get().ok_or("error");
        match tsize {
            Ok(size) => {
                let i = size.cols as u32;
                if image_width > i {
                    println!("Resizing to {}x{}", i, image_height);
                    image = image.resize(i, image_height, imageops::FilterType::Nearest);
                    image_width = i;
                }
            }
            Err(_) => {}
        };
    } else {
        image = image.resize(
            args.width.unwrap(),
            image_height,
            imageops::FilterType::Nearest,
        );
        image_width = args.width.unwrap();
    }

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
            if count == image_width {
                writeln!(&mut stdout, "").unwrap();
                count = 0;
            }
        }
    }
}
