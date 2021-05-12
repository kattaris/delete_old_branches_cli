pub mod data {
    use std::convert::TryFrom;
    use std::fmt;
    use std::fmt::Formatter;

    use chrono::NaiveDateTime;
    use git2::Oid;

    use crate::errors::term_errors::Errors;

    pub const HELP: &str = r#"quit or q - Exit from app
local or l - Get local branches
remote or r - Get remote branches"#;

    pub struct Branch<'repo> {
        id: Oid,
        pub name: String,
        pub time: NaiveDateTime,
        pub branch: git2::Branch<'repo>,
    }

    impl Branch<'_> {
        pub fn new(id: Oid, name: String, time: NaiveDateTime, branch: git2::Branch) -> Branch {
            Branch {
                id,
                name,
                time,
                branch,
            }
        }
    }

    impl fmt::Display for Branch<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} - Last commit {} Hash {}", self.name, self.time, self.id)
        }
    }

    pub enum Commands {
        Quit(),
        Local(),
        Remote(),
        Help(),
        Delete(String),
    }

    impl TryFrom<Vec<&str>> for Commands {
        type Error = Errors;

        fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
            match value[0].to_lowercase().as_str() {
                "quit" | "q" => Ok(Commands::Quit()),
                "local" | "l" => Ok(Commands::Local()),
                "remote" | "r" => Ok(Commands::Remote()),
                "help" | "h" | "?" => Ok(Commands::Help()),
                "delete" | "d" => Ok(Commands::Delete(value[1].to_string())),
                _ => {Err(Errors::InvalidInput(value.join(" ")))
                }
            }
        }
    }

    pub type Result<T, E = Errors> = std::result::Result<T, E>;
}