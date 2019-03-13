extern crate nx;

use nx::NxResult;

fn run_on_one(fastq_file: &str, bucket_count: u8) -> Vec<NxResult> {
  let fastq_files_globs: Vec<String> = vec![fastq_file.to_string()];

  nx::run(fastq_files_globs, bucket_count)
}

#[test]
fn test_2_buckets() {
  let results1 = run_on_one("tests/fixtures/1.fastq", 2);
  let results2 = run_on_one("tests/fixtures/1.fastq.gz", 2);
  let results3 = run_on_one("tests/fixtures/1.fastq.bz2", 2);
  let results4 = run_on_one("tests/fixtures/2.fastq", 2);
  let results5 = run_on_one("tests/fixtures/2.fastq.gz", 2);
  let results6 = run_on_one("tests/fixtures/2.fastq.bz2", 2);

  assert_eq!(results1.len(), 1);
  assert_eq!(results2.len(), 1);
  assert_eq!(results3.len(), 1);
  assert_eq!(results4.len(), 1);
  assert_eq!(results5.len(), 1);
  assert_eq!(results6.len(), 1);

  assert_eq!(results1[0].n_result, 70);
  assert_eq!(results2[0].n_result, 70);
  assert_eq!(results3[0].n_result, 70);
  assert_eq!(results4[0].n_result, 50);
  assert_eq!(results5[0].n_result, 50);
  assert_eq!(results6[0].n_result, 50);
}

#[test]
fn test_3_bucket() {
  let results1 = run_on_one("tests/fixtures/1.fastq", 3);
  let results2 = run_on_one("tests/fixtures/1.fastq.gz", 3);
  let results3 = run_on_one("tests/fixtures/1.fastq.bz2", 3);
  let results4 = run_on_one("tests/fixtures/2.fastq", 3);
  let results5 = run_on_one("tests/fixtures/2.fastq.gz", 3);
  let results6 = run_on_one("tests/fixtures/2.fastq.bz2", 3);

  assert_eq!(results1.len(), 2);
  assert_eq!(results2.len(), 2);
  assert_eq!(results3.len(), 2);
  assert_eq!(results4.len(), 2);
  assert_eq!(results5.len(), 2);
  assert_eq!(results6.len(), 2);

  assert_eq!(results1[0].n_result, 70);
  assert_eq!(results1[1].n_result, 50);

  assert_eq!(results2[0].n_result, 70);
  assert_eq!(results2[1].n_result, 50);

  assert_eq!(results3[0].n_result, 70);
  assert_eq!(results3[1].n_result, 50);

  assert_eq!(results4[0].n_result, 70);
  assert_eq!(results4[1].n_result, 40);

  assert_eq!(results5[0].n_result, 70);
  assert_eq!(results5[1].n_result, 40);

  assert_eq!(results6[0].n_result, 70);
  assert_eq!(results6[1].n_result, 40);
}

#[test]
fn test_glob() {
  let globs: Vec<String> = vec!["tests/fixtures/*.fastq".to_string()];

  let results = nx::run(globs, 5);

  assert_eq!(results[0].n_result, 80);
  assert_eq!(results[1].n_result, 70);
  assert_eq!(results[2].n_result, 50);
  assert_eq!(results[3].n_result, 40);
}
