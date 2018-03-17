#[macro_use]
extern crate clap;
use clap::Arg;
use std::process;

fn main() {
  include_str!("../Cargo.toml");
  let args = app_from_crate!()
    .arg(
      Arg::with_name("path")
        .help("Path to Git repository")
        .required(true),
    )
    .arg(
      Arg::with_name("cmd")
        .env("SSH_ORIGINAL_COMMAND")
        .hidden(true),
    )
    .get_matches();

  let path = args.value_of("path").unwrap();

  match args.is_present("cmd") {
    false => {
      eprintln!("Error: SSH_ORIGINAL_COMMAND environment variable isn't set");
      process::exit(1)
    }
    _ => (),
  }

  println!("Git repo: {:?}", path);
}
