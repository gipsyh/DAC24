use clap::Parser;
use ic3::{Args, Ic3};

fn main() {
    let mut args = Args::parse();
    let aig = // Safe
    // "../MC-Benchmark/hwmcc20/aig/2019/beem/pgm_protocol.7.prop1-back-serstep.aag";
    "../MC-Benchmark/hwmcc20/aig/2019/goel/industry/cal143/cal143.aag";
    // "../MC-Benchmark/hwmcc20/aig/2019/goel/industry/cal118/cal118.aag";
    // "../MC-Benchmark/hwmcc20/aig/2019/goel/industry/cal102/cal102.aag";
    // "../MC-Benchmark/hwmcc20/aig/2019/goel/industry/cal112/cal112.aag";
    // "../MC-Benchmark/hwmcc20/aig/2019/goel/industry/cal140/cal140.aag";
    if args.model.is_none() {
        args.model = Some(aig.to_string());
    }

    let mut ic3 = Ic3::new(args);
    println!("result: {}", ic3.check());
}
