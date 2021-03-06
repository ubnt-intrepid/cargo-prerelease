extern crate cargo_version_sync;
extern crate failure;
extern crate structopt;

use cargo_version_sync::runner::Runner;

mod cli {
    use structopt::StructOpt;

    #[derive(Debug, StructOpt)]
    pub enum Cargo {
        #[structopt(name = "version-sync")]
        VersionSync(VersionSync),
    }

    #[derive(Debug, StructOpt)]
    pub struct VersionSync {
        /// Uses verbose output
        #[structopt(short = "v", long = "verbose")]
        pub verbose: bool,
    }

    pub fn parse_args() -> VersionSync {
        match StructOpt::from_args() {
            Cargo::VersionSync(args) => args,
        }
    }
}

fn main() -> failure::Fallible<()> {
    cli::parse_args();
    let runner = Runner::init()?;

    let changeset = runner.collect_changeset()?;
    if !changeset.diffs.is_empty() {
        println!("{}", changeset);
        std::process::exit(1);
    }

    println!("[cargo-version-sync] no change(s) detected.");
    Ok(())
}
