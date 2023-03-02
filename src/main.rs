mod Types;
use crate::Types::Type;

use clap::Parser;

///Simple progrma to make my life easier
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Framework using
    #[arg(short, long)]
    framwork: String,

    /// Type of componet ex: hook,or page
    #[arg(short, long)]
    component: String,
}
fn main() {
    let args = Args::parse();
    let mut t = Type::new(&args.name, &args.framwork, &args.component);
    t.set_path();

    print!("{:?}", t)
}
