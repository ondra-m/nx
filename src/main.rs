extern crate nx;

use std::io::{Result};
use std::env::args;

fn main() -> Result<()> {
  let bucket_count: u8 = args().nth(1).unwrap_or("2".to_string()).parse().unwrap();
  let fastq_files_globs: Vec<String> = args().skip(2).collect();

  nx::run_and_print(fastq_files_globs, bucket_count)
}
