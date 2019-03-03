extern crate nx;

use std::io::{Result};
use std::env::args;

fn main() -> Result<()> {
  let fastq_file = args().nth(1).expect("No fastq were given");
  let bucket_count: u8 = args().nth(2).unwrap_or("2".to_string()).parse().unwrap();

  nx::run_and_print(fastq_file, bucket_count)
}
