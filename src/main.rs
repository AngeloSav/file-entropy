use clap::Parser;
use memmap2::MmapOptions;
use minimum_redundancy::Frequencies;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, fs::File};

#[derive(Parser)]
#[command(name = "Entropy")]
#[command(about = "Returns entropy (in bits) of a file", long_about = None)]
struct Cli {
    /// file on which we find the entroy
    input_file: String,

    /// number of threads to use
    #[arg(short, long)]
    n_threads: Option<usize>,

    /// show the hashmap containing the number of occurrences of each character in the file
    #[arg(short, long)]
    show_occs: bool,

    #[clap(short, long, value_parser)]
    #[arg(default_value_t = 4096)]
    chunk_size: usize,
}

fn main() {
    let cli = Cli::parse();

    if let Some(n) = cli.n_threads {
        if let Err(e) = rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
        {
            panic!("error occurred: {}", e);
        }
    }

    let file = File::open(cli.input_file).expect("could not open file");
    let map = unsafe { MmapOptions::new().map(&file).expect("could not mmap file") };
    let chunks = map.chunks(cli.chunk_size).collect::<Vec<_>>();

    let occs: Vec<usize> = chunks
        .par_iter()
        .fold(
            || vec![0; 256],
            |a, &c| {
                c.iter().fold(a, |mut a, &c| {
                    a[c as usize] += 1;
                    a
                })
            },
        )
        .reduce(
            || vec![0; 256],
            |mut a, b| {
                for (i, &v) in b.iter().enumerate() {
                    a[i] += v;
                }
                a
            },
        );

    let mut h = HashMap::new();
    for (c, &v) in occs.iter().enumerate() {
        if v != 0 {
            h.entry(c).or_insert(v);
        }
    }

    if cli.show_occs {
        println!("{:?}", &h);
    }

    println!("entropy: {}", h.entropy());
}
