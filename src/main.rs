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

fn display_state(repo: &Repository) -> &'static str {
    use git2::RepositoryState::*;
    match repo.state() {
        Merge => "merge",
        Revert => "revert",
        RevertSequence => "revert-sequence",
        CherryPick => "cherrypick",
        CherryPickSequence => "cherrypick-sequence",
        Bisect => "bisect",
        Rebase => "rebase",
        RebaseInteractive => "rebase-interactive",
        RebaseMerge => "rebase-merge",
        ApplyMailbox => "apply-mailbox",
        ApplyMailboxOrRebase => "apply-mailbox-or-rebase",
        _ => unreachable!(),
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
            println!("{} {}", repo.path().display(), display_state(&repo));
        }
    }

    for repo in repositories.take(&RepositoryState::Clean) {
        println!("{}", repo.path().display());

        for status in repo.statuses(Some(&mut status_opts))?.iter() {
            if status.status() != Status::CURRENT {
                println!("  {} {:?}", status.path().unwrap(), status.status());
            }
        }
    }
    Ok(())
}
