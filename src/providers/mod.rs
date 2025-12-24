mod branches;
mod commit_parents;
mod commits;

pub use branches::BranchesProvider;
pub use commit_parents::CommitParentsProvider;
pub use commits::CommitsProvider;

use crate::error::Result;
use crate::git::GitRepo;
use rusqlite::Connection;

pub trait Provider {
    fn table_name(&self) -> &'static str;
    fn populate(&self, conn: &Connection, repo: &GitRepo) -> Result<()>;
}
