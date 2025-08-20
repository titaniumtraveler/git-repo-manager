use clap::Parser;
use git_repo_manager::cli::Cli;

fn main() -> anyhow::Result<()> {
    // CompleteEnv::with_factory(Cli::command).complete();
    let res = Cli::parse().run();
    match res {
        Ok(()) => (),
        Err(err) => println!("{err:#?}"),
    }
    Ok(())
}
