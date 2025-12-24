use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColumnInfo {
    pub name: &'static str,
    pub sql_type: &'static str,
    pub nullable: bool,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub columns: &'static [ColumnInfo],
    pub create_sql: &'static str,
}

pub static COMMITS_COLUMNS: &[ColumnInfo] = &[
    ColumnInfo {
        name: "id",
        sql_type: "TEXT",
        nullable: false,
        description: "Full SHA-1 hash (40 characters)",
    },
    ColumnInfo {
        name: "short_id",
        sql_type: "TEXT",
        nullable: false,
        description: "Abbreviated hash (7 characters)",
    },
    ColumnInfo {
        name: "tree_id",
        sql_type: "TEXT",
        nullable: false,
        description: "Tree object SHA",
    },
    ColumnInfo {
        name: "author_name",
        sql_type: "TEXT",
        nullable: false,
        description: "Author's name",
    },
    ColumnInfo {
        name: "author_email",
        sql_type: "TEXT",
        nullable: false,
        description: "Author's email",
    },
    ColumnInfo {
        name: "authored_at",
        sql_type: "DATETIME",
        nullable: false,
        description: "When originally written",
    },
    ColumnInfo {
        name: "committer_name",
        sql_type: "TEXT",
        nullable: false,
        description: "Committer's name",
    },
    ColumnInfo {
        name: "committer_email",
        sql_type: "TEXT",
        nullable: false,
        description: "Committer's email",
    },
    ColumnInfo {
        name: "committed_at",
        sql_type: "DATETIME",
        nullable: false,
        description: "When committed",
    },
    ColumnInfo {
        name: "message",
        sql_type: "TEXT",
        nullable: false,
        description: "Full commit message",
    },
    ColumnInfo {
        name: "summary",
        sql_type: "TEXT",
        nullable: false,
        description: "First line of message",
    },
    ColumnInfo {
        name: "body",
        sql_type: "TEXT",
        nullable: true,
        description: "Message body (lines 2+)",
    },
    ColumnInfo {
        name: "parent_count",
        sql_type: "INTEGER",
        nullable: false,
        description: "Number of parents (0=root, 1=normal, 2+=merge)",
    },
    ColumnInfo {
        name: "is_merge",
        sql_type: "BOOLEAN",
        nullable: false,
        description: "True if parent_count > 1",
    },
    ColumnInfo {
        name: "repo",
        sql_type: "TEXT",
        nullable: false,
        description: "Repository path",
    },
];

pub static COMMIT_PARENTS_COLUMNS: &[ColumnInfo] = &[
    ColumnInfo {
        name: "commit_id",
        sql_type: "TEXT",
        nullable: false,
        description: "Child commit SHA",
    },
    ColumnInfo {
        name: "parent_id",
        sql_type: "TEXT",
        nullable: false,
        description: "Parent commit SHA",
    },
    ColumnInfo {
        name: "parent_index",
        sql_type: "INTEGER",
        nullable: false,
        description: "0=first parent, 1=second parent (merge), etc.",
    },
    ColumnInfo {
        name: "repo",
        sql_type: "TEXT",
        nullable: false,
        description: "Repository path",
    },
];

pub static BRANCHES_COLUMNS: &[ColumnInfo] = &[
    ColumnInfo {
        name: "name",
        sql_type: "TEXT",
        nullable: false,
        description: "Branch name",
    },
    ColumnInfo {
        name: "full_name",
        sql_type: "TEXT",
        nullable: false,
        description: "Full refname (refs/heads/main)",
    },
    ColumnInfo {
        name: "target_id",
        sql_type: "TEXT",
        nullable: false,
        description: "Commit SHA branch points to",
    },
    ColumnInfo {
        name: "is_remote",
        sql_type: "BOOLEAN",
        nullable: false,
        description: "Remote tracking branch",
    },
    ColumnInfo {
        name: "is_head",
        sql_type: "BOOLEAN",
        nullable: false,
        description: "Currently checked out",
    },
    ColumnInfo {
        name: "remote_name",
        sql_type: "TEXT",
        nullable: true,
        description: "Remote name for remote branches",
    },
    ColumnInfo {
        name: "upstream",
        sql_type: "TEXT",
        nullable: true,
        description: "Upstream branch name",
    },
    ColumnInfo {
        name: "ahead",
        sql_type: "INTEGER",
        nullable: true,
        description: "Commits ahead of upstream",
    },
    ColumnInfo {
        name: "behind",
        sql_type: "INTEGER",
        nullable: true,
        description: "Commits behind upstream",
    },
    ColumnInfo {
        name: "repo",
        sql_type: "TEXT",
        nullable: false,
        description: "Repository path",
    },
];

pub static TABLES: &[TableInfo] = &[
    TableInfo {
        name: "commits",
        description: "Commit history and metadata",
        category: "CORE",
        columns: COMMITS_COLUMNS,
        create_sql: r#"
            CREATE TABLE IF NOT EXISTS commits (
                id TEXT NOT NULL,
                short_id TEXT NOT NULL,
                tree_id TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_email TEXT NOT NULL,
                authored_at TEXT NOT NULL,
                committer_name TEXT NOT NULL,
                committer_email TEXT NOT NULL,
                committed_at TEXT NOT NULL,
                message TEXT NOT NULL,
                summary TEXT NOT NULL,
                body TEXT,
                parent_count INTEGER NOT NULL,
                is_merge INTEGER NOT NULL,
                repo TEXT NOT NULL,
                PRIMARY KEY (id, repo)
            )
        "#,
    },
    TableInfo {
        name: "commit_parents",
        description: "Parent-child relationships enabling ancestry traversal",
        category: "CORE",
        columns: COMMIT_PARENTS_COLUMNS,
        create_sql: r#"
            CREATE TABLE IF NOT EXISTS commit_parents (
                commit_id TEXT NOT NULL,
                parent_id TEXT NOT NULL,
                parent_index INTEGER NOT NULL,
                repo TEXT NOT NULL,
                PRIMARY KEY (commit_id, parent_id, repo)
            )
        "#,
    },
    TableInfo {
        name: "branches",
        description: "Local and remote tracking branches",
        category: "REFERENCES",
        columns: BRANCHES_COLUMNS,
        create_sql: r#"
            CREATE TABLE IF NOT EXISTS branches (
                name TEXT NOT NULL,
                full_name TEXT NOT NULL,
                target_id TEXT NOT NULL,
                is_remote INTEGER NOT NULL,
                is_head INTEGER NOT NULL,
                remote_name TEXT,
                upstream TEXT,
                ahead INTEGER,
                behind INTEGER,
                repo TEXT NOT NULL,
                PRIMARY KEY (full_name, repo)
            )
        "#,
    },
];

pub fn get_table_info(name: &str) -> Option<&'static TableInfo> {
    TABLES.iter().find(|t| t.name == name)
}

pub fn get_tables_by_category() -> HashMap<&'static str, Vec<&'static TableInfo>> {
    let mut map: HashMap<&'static str, Vec<&'static TableInfo>> = HashMap::new();
    for table in TABLES {
        map.entry(table.category).or_default().push(table);
    }
    map
}
