use configparser::ini::Ini;
use csv::ReaderBuilder;
use ndarray::Array2;
use ndarray_csv::Array2Reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

mod cluster;
use cluster::*;
mod generation;
use generation::*;
mod genetic;
use genetic::*;

fn read_vars(config_file: &str) -> (Option<i64>, Option<i64>, Option<i64>, Option<f64>) {
    let mut config = Ini::new();
    config.load(config_file).expect("File not found.");

    (
        config.getint("vars", "budget").unwrap(),
        config.getint("vars", "streams").unwrap(),
        config.getint("vars", "num_of_ind").unwrap(),
        config.getfloat("vars", "pclo").unwrap(),
    )
}

fn read_csv(path_to_file: &str) -> Result<Array2<u32>, Box<dyn Error>> {
    let file = File::open(path_to_file)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    Ok(reader.deserialize_array2((640992, 6))?)
}

fn main() {
    let config_file = "config.ini";
    let _data = read_csv("basket_1h.csv");
    let _dim: usize = 6;
    let _generation_count = 0;
    let (_budget, _streams, _num_of_ind, _pclo) = read_vars(config_file);

    let initial = Generation { num_of_ind: _num_of_ind.unwrap(), deterministic: HashMap::new(), random: Vec::new(), streamChromosomes: Vec::new(), chromosomes: HashMap::new(), generationCount: _generation_count, data: _data, k: Vec::new(), streams: _streams.unwrap(), dim: _dim };
    initial.generate_chromosomes();

    let _kmax = 14;

    let clustering = Clustering {generation: initial, data: _data, dim: _dim, kmax: _kmax};

    let generation = clustering.calcChromosomesFit();
    generation.sortChromosomes();

    let ga = Genetic {
        num_of_ind: _num_of_ind.unwrap(),
        pclo: _pclo.unwrap(),
        budget: _budget.unwrap(),
        data: _data,
        generationCount: _generation_count,
        kmax: _kmax,
        dim: _dim,
        prevGeneration: HashMap::new(),
    };

    let (generation, _generation_count) = ga.geneticProcess(initial, initial.deterministic);
    while _generation_count <= _budget.unwrap() {
        let (generation, _generation_count) = ga.geneticProcess(generation, generation.deterministic);
    }

    let iBest = generation.getBestChromosome();
    clustering.printIBest(iBest);
}
