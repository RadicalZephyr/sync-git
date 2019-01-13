use std::iter::FromIterator;
use std::mem;
use std::ops::{Index, IndexMut};

use git2::{Repository, RepositoryState};

fn to_usize(state: &RepositoryState) -> usize {
    use git2::RepositoryState::*;
    match state {
        Clean => 0,
        Merge => 1,
        Revert => 2,
        RevertSequence => 3,
        CherryPick => 4,
        CherryPickSequence => 5,
        Bisect => 6,
        Rebase => 7,
        RebaseInteractive => 8,
        RebaseMerge => 9,
        ApplyMailbox => 10,
        ApplyMailboxOrRebase => 11,
    }
}

pub struct RepositoryStateMap {
    data: [Vec<Repository>; 12],
}

impl Default for RepositoryStateMap {
    fn default() -> RepositoryStateMap {
        let data = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        RepositoryStateMap { data }
    }
}

impl RepositoryStateMap {
    pub fn new() -> RepositoryStateMap {
        RepositoryStateMap::default()
    }

    pub fn take(&mut self, index: &RepositoryState) -> Vec<Repository> {
        mem::replace(&mut self.data[to_usize(index)], vec![])
    }
}

impl Index<&RepositoryState> for RepositoryStateMap {
    type Output = Vec<Repository>;

    fn index(&self, index: &RepositoryState) -> &Self::Output {
        &self.data[to_usize(index)]
    }
}

impl IndexMut<&RepositoryState> for RepositoryStateMap {
    fn index_mut(&mut self, index: &RepositoryState) -> &mut Self::Output {
        &mut self.data[to_usize(index)]
    }
}

impl FromIterator<Repository> for RepositoryStateMap {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Repository>,
    {
        let mut repo_map = RepositoryStateMap::new();
        for repo in iter {
            repo_map[&repo.state()].push(repo);
        }
        repo_map
    }
}
