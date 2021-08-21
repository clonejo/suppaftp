use std::path::PathBuf;
use std::str::FromStr;

pub enum Command {
    Cdup,
    Connect(String, bool),
    Cwd(String),
    Help,
    List(Option<String>),
    Login,
    Mdtm(String),
    Mkdir(String),
    Noop,
    Put(PathBuf, String),
    Pwd,
    Quit,
    Rename(String, String),
    Retr(String, PathBuf),
    Rm(String),
    Rmdir(String),
    Size(String),
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split string by space
        let mut args = s.split_ascii_whitespace();
        // Match args
        match args.next() {
            Some(cmd) => match cmd.to_ascii_uppercase().as_str() {
                "CDUP" => Ok(Self::Cdup),
                "CONNECT" => match args.next() {
                    Some(addr) => Ok(Self::Connect(addr.to_string(), false)),
                    None => Err("Missing `addr` field"),
                },
                "CONNECT+S" => match args.next() {
                    Some(addr) => Ok(Self::Connect(addr.to_string(), true)),
                    None => Err("Missing `addr` field"),
                },
                "CWD" => match args.next() {
                    Some(p) => Ok(Self::Cwd(p.to_string())),
                    None => Err("Missing `dir` field"),
                },
                "HELP" => Ok(Self::Help),
                "LIST" => match args.next() {
                    Some(dir) => Ok(Self::List(Some(dir.to_string()))),
                    None => Ok(Self::List(None)),
                },
                "LOGIN" => Ok(Self::Login),
                "MDTM" => match args.next() {
                    Some(file) => Ok(Self::Mdtm(file.to_string())),
                    None => Err("Missing `file` field"),
                },
                "MKDIR" => match args.next() {
                    Some(file) => Ok(Self::Mkdir(file.to_string())),
                    None => Err("Missing `file` field"),
                },
                "NOOP" => Ok(Self::Noop),
                "PUT" => {
                    let local: PathBuf = match args.next() {
                        Some(l) => PathBuf::from(l),
                        None => return Err("Missing `source` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Put(local, d.to_string())),
                        None => Err("Missing `dest` field"),
                    }
                }
                "PWD" => Ok(Self::Pwd),
                "QUIT" => Ok(Self::Quit),
                "RENAME" => {
                    let src: String = match args.next() {
                        Some(s) => s.to_string(),
                        None => return Err("Missing `src` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Rename(src, d.to_string())),
                        None => Err("Missing `dest` field"),
                    }
                }
                "RETR" => {
                    let file: String = match args.next() {
                        Some(f) => f.to_string(),
                        None => return Err("Missing `file` field"),
                    };
                    match args.next() {
                        Some(d) => Ok(Self::Retr(file, PathBuf::from(d))),
                        None => Err("Missing `dest` field"),
                    }
                }
                "RM" => match args.next() {
                    Some(file) => Ok(Self::Rm(file.to_string())),
                    None => Err("Missing `file` field"),
                },
                "RMDIR" => match args.next() {
                    Some(dir) => Ok(Self::Rmdir(dir.to_string())),
                    None => Err("Missing `file` field"),
                },
                "SIZE" => match args.next() {
                    Some(dir) => Ok(Self::Size(dir.to_string())),
                    None => Err("Missing `file` field"),
                },
                _ => Err("Unknown command"),
            },
            None => Err("Unknown command"),
        }
    }
}