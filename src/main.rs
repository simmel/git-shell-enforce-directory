#[macro_use]
extern crate clap;
use clap::Arg;
use std::{env, fs, path, process};
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
        .required(true)
        .multiple(true),
    )
    .arg(
      Arg::with_name("v")
        .short("v")
        .help("Sets the log level to debug"),
    )
    .arg(
      Arg::with_name("recursive")
        .short("r")
        .long("recursive")
        .help("Allow any subdirectory of the provided paths"),
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

  let recursive = args.is_present("recursive");

  let mut arg_paths = args
    .values_of("path")
    .unwrap()
    .map(|x| fs::canonicalize(x).unwrap());

  debug!(
    "SSH_ORIGINAL_COMMAND: {:#?}",
    env::var("SSH_ORIGINAL_COMMAND")
  );
  let cmd = match env::var("SSH_ORIGINAL_COMMAND") {
    Ok(val) => val,
    Err(_e) => {
      fatal!("SSH_ORIGINAL_COMMAND environment variable isn't set");
    }
  };

  let caps = is_upload_or_receive(&cmd);
  let caps = match caps {
    Some(caps) => caps,
    None => {
      fatal!("Command to run looks dangerous: {:?}", cmd);
    }
  };

  let read_only = args.is_present("read-only");
  debug!("read_only: {}", read_only);
  debug!("command: {}", &caps["command"]);
  if read_only && &caps["command"] == "git-receive-pack" {
    fatal!("No write commands allowed, read-only.");
  }

  let cmd_path = match recursive {
    true => fs::canonicalize(&caps["path"]).unwrap(),
    false => path::PathBuf::from(&caps["path"]),
  };

  debug!("path from SSH_ORIGINAL_COMMAND: {:?}", cmd_path);
  if !arg_paths.any(|arg_path| {
    debug!("path: {:?}", arg_path);
    match recursive {
      true => cmd_path.starts_with(fs::canonicalize(arg_path).unwrap()),
      false => cmd_path == arg_path,
    }
  }) {
    fatal!("Path {:?} not allowed", cmd_path);
  }

  let git_shell = "git-shell";
  let err = exec::Command::new(git_shell).arg("-c").arg(&cmd).exec();
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
