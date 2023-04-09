use super::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
extern crate fs_extra;
use fs_extra::file::{copy, CopyOptions};

#[derive(Debug, Parser)]
pub(crate) struct Snapshot {
  #[clap(help = "List sats in subsidy at <HEIGHT>.")]
  done: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
  pub done: bool,
}
pub(crate) fn run(options: Options) -> Result {
  let index = Index::open(&options)?;

  let result = index.perform_snapshot();

  println!("{}", result.unwrap());

  print_json(Output { done: true })?;

  Ok(())
}

impl Snapshot {}
