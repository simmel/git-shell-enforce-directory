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
    .arg(
      Arg::with_name("read-only")
        .long("read-only")
        .help("Disable write operations"),
    )
    .get_matches();

  let level = match args.occurrences_of("v") {
    0 => log::LevelFilter::Info,
    _ => log::LevelFilter::Debug,
  };

  env_logger::Builder::from_default_env()
    .default_format_level(true)
    .default_format_module_path(false)
    .default_format_timestamp(false)
    .filter(Some(""), level)
    .init();

  let path = args.value_of("path").unwrap();

  debug!("SSH_ORIGINAL_COMMAND: {:#?}", args.value_of("cmd"));
  let cmd = match args.is_present("cmd") {
    false => {
      fatal!("SSH_ORIGINAL_COMMAND environment variable isn't set");
    }
    true => args.value_of("cmd").unwrap(),
  };

  let caps = is_upload_or_receive(cmd);
  let caps = match caps {
    Some(caps) => caps,
    None => {
      fatal!("Command to run looks dangerous: {:?}", cmd);
    }
  };

  let read_only = args.is_present("read-only");
  debug!("read_only: {}", read_only);
  debug!("command: {}", &caps["command"]);
  if read_only && &caps["command"] == "git-upload-pack" {
    fatal!("No write commands allowed, read-only.");
  }

  debug!("path: {:?}", path);
  debug!("path from SSH_ORIGINAL_COMMAND: {:?}", &caps["path"]);
  if path != &caps["path"] {
    fatal!("Path {:?} not allowed, only {:?}", &caps["path"], path);
  }

  let git_shell = "/usr/bin/git-shell";
  let err = exec::Command::new(git_shell).arg("-c").arg(cmd).exec();
  fatal!("{}: {:?}", err, git_shell);
}

fn is_upload_or_receive(cmd: &str) -> Option<regex::Captures> {
  let re = Regex::new(r"^(?P<command>git-(?:receive|upload)-pack) '(?P<path>.+)'$").unwrap();
  let caps = re.captures(cmd);
  debug!("caps: {:#?}", caps);
  caps
}

#[test]
fn correct_command_works() {
  let caps = is_upload_or_receive("git-upload-pack '/ok/path/'").unwrap();
  assert_eq!(&caps["command"], "git-upload-pack");
  assert_eq!(&caps["path"], "/ok/path/");
}

#[test]
fn malicious_command_fails() {
  let caps = is_upload_or_receive("git-upload-archive '/danger/zone/'");
  assert!(caps.is_none());
}

#[test]
fn newline_command_fails() {
  let caps = is_upload_or_receive("git-upload-pack '/danger/zone/'\n");
  assert!(caps.is_none());
}
