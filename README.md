# file-entropy

This is a little program i wrote to quickly get the entropy of big files (> 4GB)

## Installation
to build from source, just run:

```bash
https://github.com/AngeloSav/entropy.git
cd entropy
cargo build --release
```

or, if you just want the executable:
```bash
cargo install --git https://github.com/AngeloSav/file-entropy.git file-entropy
```

## Usage

```
Returns entropy (in bits) of a file

Usage: file-entropy [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  file on which we find the entroy

Options:
  -n, --n-threads <N_THREADS>    number of threads to spawn
  -s, --show-occs                show the hashmap containing the number of occurrences of each character in the file
  -c, --chunk-size <CHUNK_SIZE>  size (in bytes) of the chunks parsed by each thread [default: 4096]
  -h, --help                     Print help
```

### Example

```bash
➜  ~ ls -lh dataset/big_english           
-rw-rw-r-- 1 anglo anglo 11G nov  7  2023 dataset/big_english
➜  ~ time file-entropy dataset/big_english
entropy: 4.572675914420909
file-entropy dataset/big_english  12,14s user 29,68s system 364% cpu 11,470 total
```