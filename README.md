# Nx

Contig statistic (N50, L50, ........) for .fastq files (gz and bz2 compresions are supported).

Designed for large data.

## Build

1. Clone this repository
2. Install [rust language](https://www.rust-lang.org)
3. Install [cargo package manager](https://doc.rust-lang.org/cargo)
4. Build the project

```bash
cargo build --release
```

The binary is located at `target/release/nx`

## Usage

```bash
nx BUCKET_COUNT GLOB_PATTERN [*glob_pattern2]
```

- **BUCKET_COUNT**
  - number of bucket
  - e.g. 2 calculate N50, L50
  - e.g. 3 calculate N33, N66, L33, L66
- **GLOB_PATTERN**
  - pattern for .fastq, .fastq.gz or .fastq.buz2 files
  - or files itself

## Examples

Calculate N50 from all .fastq files

```bash
nx 2 data/*.fastq
```

Calculate N10 .. N90 from .gz files

```bash
nx 10 data/data1.fastq.gz data/data2.fastq.gz
```
