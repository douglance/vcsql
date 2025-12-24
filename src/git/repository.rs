use crate::error::{Result, VcsqlError};
use git2::{BranchType, Commit, Reference, Repository};
use std::path::Path;

pub struct GitRepo {
    repo: Repository,
    path: String,
}

impl GitRepo {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        let repo = Repository::discover(path_ref).map_err(|e| {
            if e.code() == git2::ErrorCode::NotFound {
                VcsqlError::RepoNotFound(path_ref.display().to_string())
            } else {
                VcsqlError::Git(e)
            }
        })?;

        let workdir = repo
            .workdir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| repo.path().display().to_string());

        Ok(Self {
            repo,
            path: workdir,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn inner(&self) -> &Repository {
        &self.repo
    }

    pub fn head(&self) -> Result<Reference<'_>> {
        Ok(self.repo.head()?)
    }

    pub fn head_commit(&self) -> Result<Commit<'_>> {
        let head = self.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit)
    }

    pub fn walk_commits(&self) -> Result<impl Iterator<Item = Result<Commit<'_>>>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL)?;

        Ok(revwalk.filter_map(move |oid_result| match oid_result {
            Ok(oid) => match self.repo.find_commit(oid) {
                Ok(commit) => Some(Ok(commit)),
                Err(e) => Some(Err(VcsqlError::Git(e))),
            },
            Err(e) => Some(Err(VcsqlError::Git(e))),
        }))
    }

    pub fn branches(&self, branch_type: Option<BranchType>) -> Result<git2::Branches<'_>> {
        Ok(self.repo.branches(branch_type)?)
    }

    pub fn is_head_detached(&self) -> bool {
        self.repo.head_detached().unwrap_or(false)
    }

    pub fn graph_ahead_behind(
        &self,
        local: git2::Oid,
        upstream: git2::Oid,
    ) -> Result<(usize, usize)> {
        Ok(self.repo.graph_ahead_behind(local, upstream)?)
    }
}
