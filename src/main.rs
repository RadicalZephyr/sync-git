use git2::{Repository, RepositoryState, Status, StatusOptions};

use sync_git::{Error, RepositoryStateMap, Result, WalkGitRepos};

fn print_errors(repo: Result<Repository>) -> Option<Repository> {
    match repo {
        Ok(repo) => Some(repo),
        Err(e) => {
            match e {
                Error::Git2(git2_error) => eprintln!("sync-git: {}", git2_error.message()),
                Error::WalkDir(walkdir_error) => eprintln!("sync-git: {}", walkdir_error),
            }
            None
        }
    }
}

fn main() -> Result<()> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_ignored(false).include_untracked(true);

    let mut repositories: RepositoryStateMap =
        WalkGitRepos::new(".").filter_map(print_errors).collect();

    for state in &[
        RepositoryState::Merge,
        RepositoryState::Revert,
        RepositoryState::RevertSequence,
        RepositoryState::CherryPick,
        RepositoryState::CherryPickSequence,
        RepositoryState::Bisect,
        RepositoryState::Rebase,
        RepositoryState::RebaseInteractive,
        RepositoryState::RebaseMerge,
        RepositoryState::ApplyMailbox,
        RepositoryState::ApplyMailboxOrRebase,
    ] {
        for repo in &repositories[state] {
            eprintln!("{} {:?}", repo.path().display(), state);
        }
    }

    for repo in repositories.take(&RepositoryState::Clean) {
        println!("{}", repo.path().display());

        for status in repo.statuses(Some(&mut status_opts))?.iter() {
            if status.status() != Status::CURRENT {
                println!("  file is not current: {}", status.path().unwrap());
            }
        }
    }
    Ok(())
}
