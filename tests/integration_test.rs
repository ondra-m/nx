extern crate nx;

#[test]
fn test_2_buckets() {
  let results1 = nx::run("tests/fixtures/1.fastq".to_string(), 2);
  let results2 = nx::run("tests/fixtures/1.fastq.gz".to_string(), 2);
  let results3 = nx::run("tests/fixtures/2.fastq".to_string(), 2);
  let results4 = nx::run("tests/fixtures/2.fastq.gz".to_string(), 2);

  assert_eq!(results1.len(), 1);
  assert_eq!(results2.len(), 1);
  assert_eq!(results3.len(), 1);
  assert_eq!(results4.len(), 1);

  assert_eq!(results1[0].n_result, 70);
  assert_eq!(results2[0].n_result, 70);
  assert_eq!(results3[0].n_result, 50);
  assert_eq!(results4[0].n_result, 50);
}

#[test]
fn test_3_bucket() {
  let results1 = nx::run("tests/fixtures/1.fastq".to_string(), 3);
  let results2 = nx::run("tests/fixtures/1.fastq.gz".to_string(), 3);
  let results3 = nx::run("tests/fixtures/2.fastq".to_string(), 3);
  let results4 = nx::run("tests/fixtures/2.fastq.gz".to_string(), 3);

  assert_eq!(results1.len(), 2);
  assert_eq!(results2.len(), 2);
  assert_eq!(results3.len(), 2);
  assert_eq!(results4.len(), 2);

  assert_eq!(results1[0].n_result, 70);
  assert_eq!(results1[1].n_result, 50);

  assert_eq!(results2[0].n_result, 70);
  assert_eq!(results2[1].n_result, 50);

  assert_eq!(results3[0].n_result, 70);
  assert_eq!(results3[1].n_result, 40);

  assert_eq!(results4[0].n_result, 70);
  assert_eq!(results4[1].n_result, 40);
}

