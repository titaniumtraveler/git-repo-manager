use anyhow::Context;
use git2::{Repository, Worktree};

fn main() -> anyhow::Result<()> {
    let worktree_repo = Repository::open(std::env::args().nth(1).context("missing repo path")?)?;
    println!("{:?}", worktree_repo.path());
    let tree = Worktree::open_from_repository(&worktree_repo)?;
    println!("{:?}", tree.path());
    let repo = Repository::open(worktree_repo.commondir())?;
    println!("{:?}", repo.path());
    Ok(())
}
