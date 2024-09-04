use clap::{command, Parser};
use miette::{Error, WrapErr};
use shared::{Day, Part, DayRunner};


#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
struct Args {
    // day to run
    #[arg(short, long, default_value_t = Day::DayOne, required = false)]
    day: Day,

    // part to run
    #[arg(short, long, default_value_t = Part::PartOne, required = false)]
    part: Part,
}

fn main() -> miette::Result<(), Error> {
    let args = Args::parse();
    
    let day = args.day;
    let part = args.part;
    println!("running day `{day:?}` part `{part:?}`");

    match day {
        Day::DayOne => {
            use day_one::Runner;

            let day_runner = Runner::new(None)
                .wrap_err("when creating day one runner")?;
            day_runner.run(&args.part)
                .wrap_err(format!("when running day one part: {:#?}", &args.part))?;
        }
    }
    Ok(())
}
