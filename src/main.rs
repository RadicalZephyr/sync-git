use walkdir::{DirEntry, IntoIter, Result, WalkDir};

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
                Some(err @ Err(_)) => return Some(err),
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
        println!("{}", entry?.path().display());
    }
    Ok(())
}
