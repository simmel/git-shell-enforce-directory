#[macro_use]
extern crate clap;
use clap::Arg;
use std::process;
extern crate regex;
use regex::Regex;
extern crate env_logger;
extern crate exec;
#[macro_use]
extern crate log;

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

  env_logger::Builder::from_default_env()
    .default_format_level(false)
    .default_format_module_path(false)
    .default_format_timestamp(false)
    .init();

  let path = args.value_of("path").unwrap();

  let cmd = match args.is_present("cmd") {
    false => {
      eprintln!("Error: SSH_ORIGINAL_COMMAND environment variable isn't set");
      process::exit(1)
    }
    true => args.value_of("cmd").unwrap(),
  };

  let re = Regex::new(r"^(?P<command>git-(?:receive|upload)-pack) '(?P<path>.+)'$").unwrap();
  let caps = match re.captures(cmd) {
    Some(caps) => caps,
    None => {
      eprintln!("Command to run looks dangerous: {:?}", cmd);
      process::exit(1)
    }
  };

  if path != &caps["path"] {
    eprintln!("Path {:?} not allowed, only {:?}", &caps["path"], path);
    process::exit(1)
  }

  let err = exec::Command::new("/usr/bin/git-shell")
    .arg("-c")
    .arg(cmd)
    .exec();
  println!("Error: {}", err);
  process::exit(1);
}
