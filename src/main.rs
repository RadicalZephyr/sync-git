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

fn repository_statuses(repo: &Repository, status_opts: &mut StatusOptions) -> Vec<String> {
    match repo.statuses(Some(status_opts)) {
        Ok(statuses) => statuses
            .iter()
            .filter_map(|status| {
                if status.status() != Status::CURRENT {
                    let path = match status.path() {
                        Some(path) => path,
                        None => "_",
                    };
                    Some(format!("  {} {:?}", path, status.status()))
                } else {
                    None
                }
            })
            .collect(),
        Err(e) => {
            eprintln!(
                "sync-git: {}: could not check status on repository at {}",
                e,
                repo.path().display()
            );
            vec![]
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
            println!("{} {}", repo.path().display(), display_state(&repo));
        }
    }

    for repo in repositories.take(&RepositoryState::Clean) {
        let statuses = repository_statuses(&repo, &mut status_opts);

        if !statuses.is_empty() {
            println!("{}", repo.path().display());
        }
        for status in &statuses {
            println!("{}", status);
        }
    }
    Ok(())
}
