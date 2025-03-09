use clap::Parser;
use image::{imageops, GenericImageView};

mod output;

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    #[arg(short, long)]
    image: std::path::PathBuf,

    #[arg(long)]
    height: Option<u32>,
    #[arg(short, long)]
    width: Option<u32>,
}

fn main() {
    let args = Args::parse();

    //image
    let mut image = image::open(args.image).unwrap();

    if args.width.as_ref() == None {
        let tsize = termsize::get().ok_or("Error getting terminal size.");
        // resize image to fit terminal
        match tsize {
            Ok(size) => {
                if image.dimensions().1 > size.rows as u32 {
                    image = image.resize(
                        size.cols as u32,
                        size.rows as u32 * 2 - 10,
                        imageops::FilterType::Triangle,
                    );
                }
            }
            Err(_) => {}
        };
    } else {
        //resize image to defined size
        image = image.resize(
            args.width.unwrap(),
            image.dimensions().1,
            imageops::FilterType::Triangle,
        );
    }

    output::output_image(&image);
}
