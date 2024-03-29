use std::io::{BufReader, Result, stdout};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use glob::glob;

pub struct NxResult {
  pub index: u8,
  pub n_result: u64,
  pub l_result: u64,
  pub total_length: u128,
}

fn file_decoder(path: &String) -> Box<Read> {
  let file = File::open(path).expect("Cannot open a file");

  if path.ends_with(".fastq.gz") {
    Box::new(GzDecoder::new(file))
  }
  else if path.ends_with(".fastq.bz2") {
    Box::new(BzDecoder::new(file))
  }
  else if path.ends_with(".fastq") {
    Box::new(file)
  }
  else {
    panic!("Unknow fastq format");
  }
}

fn parse_files(fastq_files_globs: Vec<String>, all_length_counts: &mut HashMap<u64, u64>) {
  let mut fastq_files: Vec<String> = vec![];

  for files_glob in &fastq_files_globs {
    for entry in glob(files_glob).expect("Failed to read glob pattern") {
      match entry {
        Err(e) => println!("{:?}", e),
        Ok(path) => fastq_files.push(path.display().to_string()),
      }
    }
  }

  let mut file_index = 0;

  for path in &fastq_files {
    file_index += 1;
    println!("[{}/{}] {} ", file_index, fastq_files.len(), path);

    let mut read_index: u64 = 0;
    let reader = BufReader::new(file_decoder(&path));

    let mut block = 0;
    for line in reader.lines() {
      if block == 1 {
        let length = line.expect("line").len();
        let counter = all_length_counts.entry(length as u64).or_insert(0);
        *counter += 1;

        read_index += 1;
        if read_index % 100000 == 0 {
          print!("\r  read_index = {}", read_index);
          stdout().flush().unwrap();
        }
      }

      block = (block + 1) % 4;
    }

    println!("\r  reads = {}\x1b[0K", read_index);
  }

}

fn calc_total_length(total_length: &mut u128, all_length_counts: &mut HashMap<u64, u64>) {
  let mut prev_total_length = *total_length;

  // u128 has max at 340282366920938463463374607431768211455
  //
  for (length, count) in all_length_counts {
    *total_length += (length * (*count)) as u128;

    if prev_total_length > *total_length {
      panic!("total_length overflow");
    }

    prev_total_length = *total_length;
  }
}

fn calc_result(nx_results: &mut Vec<NxResult>, bucket_count: u8, bucket_by: u8, total_length: u128, all_length_counts: &HashMap<u64, u64>) {
  // Limit where values should be checked
  //
  let mut bucket_lengths_to_resolve: HashMap<u8, u128> = HashMap::new();
  //
  for i in 1..bucket_count {
    let limit = (total_length / (bucket_count as u128)) * (i as u128);
    bucket_lengths_to_resolve.insert(i, limit);
  }

  // Get sorted lenghts by biggest
  //
  let mut all_sorted_lengths: Vec<u64> = all_length_counts.keys().cloned().collect();
  all_sorted_lengths.sort();
  all_sorted_lengths.reverse();

  let mut tmp_total_length: u128 = 0;
  let mut tmp_total_count: u64 = 0;
  let mut indexes_to_remove: Vec<u8> = vec![];

  for length in &all_sorted_lengths {
    tmp_total_length += (length * all_length_counts[length]) as u128;
    tmp_total_count += all_length_counts[length];

    for (index, limit) in &bucket_lengths_to_resolve {
      if tmp_total_length > *limit {
        let stat_index = bucket_by * index;
        let mut count_for_l_result: u64 = 0;

        loop {
          count_for_l_result += 1;

          if (tmp_total_length - ((length * count_for_l_result) as u128)) < *limit {
            count_for_l_result -= 1;
            break;
          }

          if count_for_l_result > all_length_counts[length] {
            panic!("count_for_l_result cannot be higher than all_length_counts[length]");
          }
        }

        nx_results.push(NxResult {
          index: stat_index,
          n_result: *length,
          l_result: (tmp_total_count - count_for_l_result),
          total_length: tmp_total_length,
        });

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
}

pub fn run(fastq_files_globs: Vec<String>, bucket_count: u8) -> Vec<NxResult> {
  let bucket_by: u8 = 100 / bucket_count;

  println!("Bucket count = {}", bucket_count);
  println!("Bucket by = {}", bucket_by);
  println!("");

  // Load lengths and their counts from fastqfile
  //
  let mut all_length_counts: HashMap<u64, u64> = HashMap::new();
  //
  parse_files(fastq_files_globs, &mut all_length_counts);
  println!("");

  // Full sequence lenght
  // Sum of lenghts of all reads
  //
  let mut total_length: u128 = 0;
  //
  calc_total_length(&mut total_length, &mut all_length_counts);
  //
  println!("Total length = {:?}", total_length);
  println!("");

  let mut nx_results: Vec<NxResult> = vec![];

  calc_result(&mut nx_results, bucket_count, bucket_by, total_length, &all_length_counts);

  nx_results.sort_by_key(|r| r.index );
  nx_results
}

pub fn run_and_print(fastq_files_globs: Vec<String>, bucket_count: u8) -> Result<()> {
  let nx_results = run(fastq_files_globs, bucket_count);

  for result in &nx_results {
    println!("N{} = {} (at {})", result.index, result.n_result, result.total_length);
  }

  println!("");

  for result in &nx_results {
    println!("L{} = {} (at {})", result.index, result.l_result, result.total_length);
  }

  Ok(())
}
