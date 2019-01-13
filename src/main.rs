use git2::{Repository, Status, StatusOptions};
use walkdir::WalkDir;

use sync_git::{Result, WalkGitRepos};

fn is_clean(repo: &Repository) -> bool {
    match repo.state() {
        git2::RepositoryState::Clean => true,
        _ => false,
    }
}

fn main() -> Result<()> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_ignored(false).include_untracked(true);

    for repo in WalkGitRepos::new(".")
        .filter_map(Result::ok)
        .filter(is_clean)
    {
        println!("{}", repo.path().display());

        for status in repo.statuses(Some(&mut status_opts))?.iter() {
            if status.status() != Status::CURRENT {
                println!("  file is not current: {}", status.path().unwrap());
            }
        }
    }
    Ok(())
}
