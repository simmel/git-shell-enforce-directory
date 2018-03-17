#[macro_use]
extern crate clap;

fn main() {
  include_str!("../Cargo.toml");
  let args = app_from_crate!().get_matches();
  println!("{:?}", args);
}
