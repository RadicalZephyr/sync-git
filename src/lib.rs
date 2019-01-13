use failure::Fail;
use git2::Repository;
use walkdir::{DirEntry, IntoIter, WalkDir};

mod status_map;
use crate::status_map::RepositoryStateMap;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "walkdir error: {}", _0)]
    WalkDir(#[cause] walkdir::Error),

    #[fail(display = "git2 error: {}", _0)]
    Git2(#[cause] git2::Error),
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git2(err)
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Error {
        Error::WalkDir(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn is_git_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".git"))
        .unwrap_or(false)
}

pub struct WalkGitRepos<I> {
    it: I,
}

impl WalkGitRepos<IntoIter> {
    pub fn new(root: impl AsRef<str>) -> WalkGitRepos<IntoIter> {
        let it = WalkDir::new(root.as_ref()).into_iter();
        WalkGitRepos { it }
    }
}

/// Like try, but for iterators that return [`Option<Result<_, _>>`].
///
/// [`Option<Result<_, _>>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
macro_rules! itry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(From::from(err))),
        }
    };
}

impl Iterator for WalkGitRepos<IntoIter> {
    type Item = Result<Repository>;

    fn next(&mut self) -> Option<Result<Repository>> {
        loop {
            let dent = match self.it.next() {
                Some(dent) => itry!(dent),
                None => return None,
            };
            if dent.file_type().is_dir() && is_git_dir(&dent) {
                self.it.skip_current_dir();
                let repo = itry!(Repository::open(dent.path()));
                return Some(Ok(repo));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::{panic, process::Command, sync::Once};

    use git2::RepositoryState;

    static START: Once = Once::new();

    fn setup() {
        START.call_once(|| {
            println!("Setting up test cases");
            let mut child = Command::new("just")
                .arg("unpack-test-data")
                .spawn()
                .expect("failed to start just");
            child.wait().expect("failed to wait on just");
        });
    }

    fn teardown() {}

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();

        let result = panic::catch_unwind(|| test());

        teardown();

        assert!(result.is_ok())
    }

    fn repository_paths(repositories: &[Repository]) -> Vec<String> {
        repositories
            .iter()
            .map(|r| r.path().to_string_lossy().into())
            .collect()
    }

    macro_rules! relative_string_vec {
        { $( $val:expr ),* } => { {
            let current_dir = std::env::current_dir().expect("something wrong with current directory");
            vec![ $( format!("{}/{}", current_dir.display(), $val) ),* ]}
        }
    }

    #[test]
    fn test_git_repo_iterator() {
        run_test(|| {
            let dir_names: Result<Vec<Repository>> = WalkGitRepos::new("test-cases").collect();
            let dir_names = dir_names.expect("unexpected deceptively named folder in test-cases");
            let expected: Vec<String> = relative_string_vec![
                "test-cases/mid-state/rebase/.git/",
                "test-cases/mid-state/rebase-interactive/.git/"
            ];
            assert_eq!(expected, repository_paths(&dir_names));
        });
    }

    #[test]
    fn test_partition_git_repo() {
        run_test(|| {
            let repositories: Result<RepositoryStateMap> =
                WalkGitRepos::new("test-cases").collect();
            let repositories =
                repositories.expect("unexpected deceptively named folder in test-cases");

            let expected = relative_string_vec!["test-cases/mid-state/rebase/.git/"];
            assert_eq!(
                expected,
                repository_paths(&repositories[&RepositoryState::Rebase])
            );

            let expected = relative_string_vec!["test-cases/mid-state/rebase-interactive/.git/"];
            assert_eq!(
                expected,
                repository_paths(&repositories[&RepositoryState::RebaseInteractive])
            );
        });
    }
}
