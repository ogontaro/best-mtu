mod mtu;
mod mtu_exploler;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 1200)]
    search_mtu_min: u32,

    #[arg(long, default_value_t = 1500)]
    search_mtu_max: u32,
}

fn main() {
    let args = Args::parse();

    println!(
        "Searching MTU from {} to {}",
        args.search_mtu_min, args.search_mtu_max
    );
    let mut explorer = mtu_exploler::MTUExplorer {
        mtu_range_min: args.search_mtu_min,
        mtu_range_max: args.search_mtu_max,
        ..Default::default()
    };
    let result = explorer.search_best_mtu();
    println!("{} is best MTU", result.unwrap());
}
