use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(last = true, required = true)]
    image_path: String
}

fn main() {
    let _args = Args::parse();
}
