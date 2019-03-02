use std::io::{BufReader, Result, stdout};
use std::io::prelude::*;
use std::fs::File;
use std::env::{args};
use std::collections::HashMap;
// use std::process;


fn main() -> Result<()> {
  let fastq_file = args().nth(1).expect("No fastq were given");
  let bucket_count: u8 = args().nth(2).unwrap_or("2".to_string()).parse().unwrap();
  let bucket_by = 100 / bucket_count;

  println!("fastq_file = {}", fastq_file);
  println!("bucket_count = {}", bucket_count);
  println!("bucket_by = {}", bucket_by);

  // process::exit(0x0100);

  let file = File::open(fastq_file)?;
  let file = BufReader::new(file);

  let mut all_sizes_counts: HashMap<u64, u64> = HashMap::new();

  // Get all sizes and their count
  //
  let mut block = 0;
  let mut line_index: u32 = 0;
  for line in file.lines() {
    if block == 1 {
      let size = line?.len();
      let counter = all_sizes_counts.entry(size as u64).or_insert(0);
      *counter += 1;

      line_index += 1;
      if line_index % 100000 == 0 {
        print!("\r  line_index = {}", line_index);
        stdout().flush().unwrap();
      }
    }

    block = (block + 1) % 4;
  }

  let mut total_length: u128 = 0;
  let mut prev_total_length = total_length;

  // Calculate total length and check overflow
  // u128 has max at 340282366920938463463374607431768211455
  //
  for (size, count) in &all_sizes_counts {
    total_length += (size * count) as u128;

    if prev_total_length > total_length {
      panic!("total_length overflow");
    }

    prev_total_length = total_length;
  }

  // Limit where values should be checked
  //
  let mut bucket_lengths_to_resolve: HashMap<u8, u128> = HashMap::new();
  // let mut bucket_results: HashMap<u8, u128> = HashMap::new();

  for i in 1..bucket_count {
    let limit = (total_length / (bucket_count as u128)) * (i as u128);
    bucket_lengths_to_resolve.insert(i, limit);
  }

  let mut all_sorted_sizes: Vec<u64> = all_sizes_counts.keys().cloned().collect();
  all_sorted_sizes.sort();
  all_sorted_sizes.reverse();

  println!("");
  println!("total_length = {:?}", total_length);

  let mut tmp_total_length: u128 = 0;
  let mut indexes_to_remove: Vec<u8> = vec![];

  for size in &all_sorted_sizes {
    tmp_total_length += (size * all_sizes_counts[size]) as u128;

    for (index, limit) in &bucket_lengths_to_resolve {
      if tmp_total_length > *limit {
        let n = bucket_by * index;
        println!("tmp_total_length = {:?}", tmp_total_length);
        println!("N{} = {:?}", n, size);

        indexes_to_remove.push(*index);
      }
    }

    if indexes_to_remove.len() > 0 {
      for index in &indexes_to_remove {
        bucket_lengths_to_resolve.remove(index);
      }

      indexes_to_remove.clear();
    }

    if bucket_lengths_to_resolve.len() == 0 {
      break;
    }
  }

  // let mut block = 0;
  // let mut buf = Vec::<u8>::new();
  // while file.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
  //   if block == 1 {
  //     // this moves the ownership of the read data to s
  //     // there is no allocation
  //     // let s = String::from_utf8(buf).expect("from_utf8 failed");
  //     // for c in s.chars() {
  //     //   println!("Character: {}", c);
  //     // }

  //     println!("1");

  //     // this returns the ownership of the read data to buf
  //     // there is no allocation
  //     // buf = s.into_bytes();
  //   }
  //     // buf.clear();

  //   block = (block + 1) % 4;
  // }

  // // let mut byte_vec: Vec<u8> = Vec::new();
  // loop {
  //   let mut buf = vec![];
  //   let my_bytes = file.read_until(b'\n', &mut buf)?;
  //   if my_bytes == 0 { break };

  //   // println!("{:?}", buf);
  //   println!("{}", buf.len());
  // }


  Ok(())
}