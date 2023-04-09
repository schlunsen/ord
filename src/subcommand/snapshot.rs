use super::*;

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

impl Snapshot {
  
}
