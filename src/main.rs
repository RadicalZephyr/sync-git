use failure::Fail;
use git2::Repository;
use walkdir::{DirEntry, IntoIter, WalkDir};

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

type Result<T> = std::result::Result<T, Error>;

fn is_git_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".git"))
        .unwrap_or(false)
}

struct GitRepos<I> {
    it: I,
}

impl GitRepos<IntoIter> {
    pub fn new(it: IntoIter) -> GitRepos<IntoIter> {
        GitRepos { it }
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

impl Iterator for GitRepos<IntoIter> {
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

fn main() -> Result<()> {
    let walker = WalkDir::new(".").into_iter();
    for repo in GitRepos::new(walker).filter_map(Result::ok) {
        println!("{}", repo.path().display());
    }
    Ok(())
}
