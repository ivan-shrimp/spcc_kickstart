#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
/// Input data generator for Problem A6 from SPCC Kickstart.
///
/// Without any additional numbers, correct A6 solutions should respond with "-1".
/// The data is designed to exercise slow paths of most Problem A6 solutions.
struct Args {
    /// In addition to the default data, add some custom numbers.
    #[clap(long)]
    additional_numbers: Vec<u32>,

    /// Add duplicates (randomly) to the data so that exactly 10^6 numbers are generated
    #[clap(long)]
    fill: bool,

    /// Output numbers in ascending order
    #[clap(long)]
    sorted: bool,

    /// Make all random data generation determinstic
    #[clap(long)]
    deterministic: bool,
}

fn main() -> anyhow::Result<()> {
    use std::io::Write;

    let Args {
        additional_numbers: mut bad_case,
        fill,
        sorted,
        deterministic,
    } = <Args as clap::Parser>::parse();

    // Add our bad case data on top of anything the user has provided.
    bad_case.extend(a6_benchgen::bad_case());

    let seed = if deterministic { 0 } else { rand::random() };
    let mut randomizer = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);

    // If the `fill` argument is provided,
    // shuffle and duplicate the data at the beginning of the slice such that there are 10^6 inputs.
    if fill {
        rand::seq::SliceRandom::shuffle(bad_case.as_mut_slice(), &mut randomizer);
        // It should be fair to assume that the user won't provide tens of thousands of additional numbers,
        // but let's just ensure that no overflow occurs.
        let dup_len = 1_000_000usize.saturating_sub(bad_case.len());
        bad_case.extend_from_within(..dup_len);
    }

    // If the `sorted` argument is provided, sort the data;
    // otherwise shuffle the data.
    if sorted {
        bad_case.sort_unstable();
    } else {
        rand::seq::SliceRandom::shuffle(bad_case.as_mut_slice(), &mut randomizer);
    }

    // Acquire a lock to stdout.
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    // Output the numbers as a valid A6 input file.
    writeln!(stdout, "{len}", len = bad_case.len())?;
    for i in &bad_case[..bad_case.len() - 1] {
        write!(stdout, "{i} ")?;
    }
    writeln!(stdout, "{last}", last = bad_case.last().unwrap())?;

    Ok(())
}
