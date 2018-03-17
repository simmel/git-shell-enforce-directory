#[macro_use]
extern crate clap;
use clap::Arg;

fn main() {
  include_str!("../Cargo.toml");
  let args = app_from_crate!()
    .arg(
      Arg::with_name("path")
        .help("Path to Git repository")
        .required(true),
    )
    .get_matches();

  let path = args.value_of("path").unwrap();
  println!("Git repo: {:?}", path);
}
