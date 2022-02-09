use clap::Parser;
use std::path;
use std::fs;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(required = true)]
    image_path: String
}

fn exit_failure_file_not_found() {
    std::process::exit(2);
}
fn exit_failure_operation_not_permitted() {
    std::process::exit(1);
}
fn exit_success() {
    std::process::exit(0);
}

fn main() {
    let args = Args::parse();

    let exists = path::Path::new(&args.image_path).exists();
    if !exists  {
        eprintln!("Error: File \"{}\" does not exist.", args.image_path);
        exit_failure_file_not_found();
    }

    let mut image_file = fs::File::open(&args.image_path).expect("File does not exist.");
    let mut contents = vec![];
    image_file.read_to_end(&mut contents).expect("Could not read file.");

    let image = stb::image::stbi_load_from_memory(&contents, stb::image::Channels::RgbAlpha);
    if image.is_none() {
        eprintln!("Error: Could not load file \"{}\".", args.image_path);
        exit_failure_operation_not_permitted();
    }

    let image_info = image.as_ref().unwrap().0;
    let width = image_info.width;
    let height = image_info.height;

    let image_data = &image.as_ref().unwrap().1;

    let mut byte_array_str : String = String::from("[\n");

    for hi in 0..height {
        for wi in 0..width {
            for pi in 0..4 {
                let i : usize = (hi * width * 4 + wi * 4 + pi).try_into().unwrap();
                byte_array_str.push_str(format!("0x{:X?},", image_data.as_slice()[i]).as_str());
            }
            byte_array_str.push_str(" ");
        }
        byte_array_str.push_str("\n");
    }
    byte_array_str.push_str("]");

    println!("{}", byte_array_str);

    exit_success();
}
