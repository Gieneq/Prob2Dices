mod generator;

use clap::Parser;
use generator::{find_best_coverage, measure_coverage_deviation, print_coverage};


#[derive(Parser, Debug)]
#[command(about = "Generate 2 dices rolls values lists so that it matches input probabilities")]
struct Cli {
    /// Probabilities list 1 to 5 floats
    #[arg(short, long, value_name = "PROBABILITIES", num_args = 1..=5)]
    probabilities: Vec<f32>,
}

fn main() {
    let cli_args = Cli::parse();
    println!("Finding coverage for probabilities: {:?} ...", cli_args.probabilities);

    let target_probabilities = cli_args.probabilities;

    match find_best_coverage(&target_probabilities) {
        Some(coverage) => {
            let deviation = measure_coverage_deviation(&target_probabilities, &coverage);
            print_coverage(&coverage, &target_probabilities, deviation);
        }
        None => {
            println!("No valid coverage found for the target probabilities.");
        }
    }

}