use super::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::{Write, Read};

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

  let data_dir = options.data_dir()?;

  let path = if let Some(path) = &options.index {
    path.clone()
  } else {
    data_dir.join("index.redb")
  };

  let copyPath = data_dir.join("index.redb.clone");

  fs::copy(path, copyPath)?;

  let tar_gz = File::create("index.tar.gz")?;
  let index_file = File::open("index.redb.clone")?;
  let enc = GzEncoder::new(tar_gz, Compression::default());

  
  
  println!("{}", result.unwrap());

  print_json(Output { done: true })?;

  Ok(())
}

impl Snapshot {}
