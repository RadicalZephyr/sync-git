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

struct GitDirs<I> {
    it: I,
}

impl GitDirs<IntoIter> {
    pub fn new(it: IntoIter) -> GitDirs<IntoIter> {
        GitDirs { it }
    }
}

impl Iterator for GitDirs<IntoIter> {
    type Item = Result<DirEntry>;

    fn next(&mut self) -> Option<Result<DirEntry>> {
        loop {
            let dent = match self.it.next() {
                Some(Ok(dent)) => dent,
                Some(Err(err)) => return Some(Err(Error::from(err))),
                None => return None,
            };
            if dent.file_type().is_dir() && is_git_dir(&dent) {
                self.it.skip_current_dir();
                return Some(Ok(dent));
            }
        }
    }
}

fn main() -> Result<()> {
    let walker = WalkDir::new(".").into_iter();
    for entry in GitDirs::new(walker) {
        if let Ok(entry) = entry {
            println!("{}", entry.path().display());
            let _repo = Repository::open(entry.path())?;
        }
    }
    Ok(())
}
