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

macro_rules! fatal {
    ($msg:expr) => ({
        error!($msg);
        process::exit(1)
    });
    ($fmt:expr, $($arg:tt)*) => ({
      error!($fmt, $($arg)*);
      process::exit(1)
    });
}

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
    .arg(
      Arg::with_name("v")
        .short("v")
        .help("Sets the log level to debug"),
    )
    .get_matches();

  let level = match args.occurrences_of("v") {
    0 => log::LevelFilter::Info,
    _ => log::LevelFilter::Debug,
  };

  env_logger::Builder::from_default_env()
    .default_format_level(false)
    .default_format_module_path(false)
    .default_format_timestamp(false)
    .filter(Some(""), level)
    .init();

  let path = args.value_of("path").unwrap();

  let cmd = match args.is_present("cmd") {
    false => {
      fatal!("SSH_ORIGINAL_COMMAND environment variable isn't set");
    }
    true => args.value_of("cmd").unwrap(),
  };

  let re = Regex::new(r"^(?P<command>git-(?:receive|upload)-pack) '(?P<path>.+)'$").unwrap();
  let caps = match re.captures(cmd) {
    Some(caps) => caps,
    None => {
      fatal!("Command to run looks dangerous: {:?}", cmd);
    }
  };

  if path != &caps["path"] {
    fatal!("Path {:?} not allowed, only {:?}", &caps["path"], path);
  }

  let err = exec::Command::new("/usr/bin/git-shell")
    .arg("-c")
    .arg(cmd)
    .exec();
  fatal!("{}", err);
}
