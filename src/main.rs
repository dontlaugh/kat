use molt::types::*;
use molt::{check_args, molt_err, molt_ok, Interp, ResultCode};
use std::collections::HashMap;
use std::env;
use thiserror::Error;
use std::path::PathBuf;

fn main() {
    let config_path = match env::var("KAT_CONFIG") {
        Ok(ref path) => {
            let p = PathBuf::from(&path);
            if !p.exists() {
                println!("KAT_CONFIG path {:?} does not exist", path);
                std::process::exit(1);
            }
            p
        },
        Err(_) => {
            let home = env::var("HOME").expect("HOME env var is not set");
            PathBuf::from(home).join(".config/kat/kat.tcl")
        },
    };
    let mut interp = Interp::new();
    let h: HashMap<String, Project> = HashMap::new();
    let id = interp.save_context(h);
    interp.add_context_command("ls", ls, id);
    interp.add_context_command("proj", proj, id);
    interp.add_context_command("open", open, id);
    molt_shell::script(&mut interp, &[config_path.to_str().unwrap().to_owned()]);
    molt_shell::repl(&mut interp, "% ");
}

pub fn ls(interp: &mut Interp, ctx_id: ContextID, argv: &[Value]) -> MoltResult {
    check_args(1, argv, 1, 1, "")?;
    let projects = interp.context::<HashMap<String, Project>>(ctx_id);

    for k in projects.keys() {
        println!("{:?}", k);
    }

    molt_ok!()
}

pub fn open(interp: &mut Interp, ctx_id: ContextID, argv: &[Value]) -> MoltResult {
    use std::process::{Command};
    check_args(1, argv, 2, 2, "project_name")?;
    let projects = interp.context::<HashMap<String, Project>>(ctx_id);
    let found: &Project = projects.get(&argv[1].as_str().to_owned()).unwrap();
    let mut c = Command::new("kitty");
    c.arg("@");
    c.arg("new-window");
    c.arg("--new-tab");
    c.arg("--tab-title");
    c.arg(&found.name);
    c.arg("--keep-focus");
    c.arg("--cwd");
    c.arg(&found.path);
    let mut _out = c.output().expect("could not execute kitty command");
    
    molt_ok!()
}

pub fn proj(interp: &mut Interp, ctx_id: ContextID, argv: &[Value]) -> MoltResult {
    check_args(1, argv, 2, 2, "definition")?;

    // parse internals
    let p = Project::parse(argv[1].as_str()).or_else(|e| molt_err!(e.to_string()))?;

    let projects = interp.context::<HashMap<String, Project>>(ctx_id);
    projects.insert(p.name.clone(), p);
    // Return empty result
    molt_ok!()
}

#[derive(Debug)]
struct Project {
    pub name: String,
    pub git: String,
    pub path: String,
}

impl Project {
    pub fn parse(raw: &str) -> Result<Self, ParseError> {

        let mut data = HashMap::<&str, Vec<&str>>::new();

        let mut valid_attrs: HashMap<&str, usize> = HashMap::new();
        valid_attrs.insert("name", 0);
        valid_attrs.insert("git", 0);
        valid_attrs.insert("path", 0);
        
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed == "" || trimmed.starts_with("#"){
                continue;
            }
            let mut splitted = trimmed.split_whitespace();
            if let Some(attr) = splitted.next() {
                let entry = valid_attrs.get_mut(attr);
                if entry.is_none() {
                    return Err(ParseError::UnknownAttribute(attr.to_owned()));
                }
                let entry = entry.unwrap();
                if *entry > 0 {
                    return Err(ParseError::DuplicateAttribute(attr.to_owned()));
                }
                *entry += 1;
                let values: Vec<&str> = splitted.collect();
                data.insert(attr, values);
            }
        }
        let name = data.get("name").ok_or(ParseError::Expected("name".to_owned()))?;
        let git = data.get("git").ok_or(ParseError::Expected("git".to_owned()))?;
        let path = data.get("path").ok_or(ParseError::Expected("path".to_owned()))?;
        Ok(Project {
            name: name[0].to_owned(),
            git: git[0].to_owned(),
            path: path[0].to_owned(),
        })
    }
}

#[derive(Error, Debug)]
pub enum AppError {

}


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("duplicate attribute: {0}")]
    DuplicateAttribute(String),
    #[error("unknown attribute: {0}")]
    UnknownAttribute(String),
    #[error("expected: {0}")]
    Expected(String),
}
