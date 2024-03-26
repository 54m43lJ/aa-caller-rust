use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
struct Opts {
    #[arg(short, group="grp")]
    a :bool,
    #[arg(short, value_enum, group="grp")]
    b :Option<Bopts>,
    #[arg(index=1, conflicts_with="action", requires="grp")]
    test :String,
    #[arg(short, exclusive=true, default_value="default", default_missing_value="empty")]
    ls :String,
    #[command(subcommand)]
    action :Action
}

#[derive(Subcommand)]
#[group(id="action")]
enum Action {
    Act,
    But
}

#[derive(Clone, ValueEnum)]
enum Bopts {
    One,
    Two
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts.ls);
}
