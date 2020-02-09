use molt::types::*;
use molt::{check_args, molt_err, molt_ok, Interp, ResultCode};
use molt_shell::repl;
use std::collections::HashMap;
use std::env;
use thiserror::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interp = Interp::new();
    let h: HashMap<String, Project> = HashMap::new();
    let id = interp.save_context(h);
    interp.add_context_command("proj", proj, id);
    molt_shell::script(&mut interp, &args[1..]);
}

pub fn proj(interp: &mut Interp, ctx_id: ContextID, argv: &[Value]) -> MoltResult {
    check_args(1, argv, 2, 2, "proj definition")?;

    // parse internals
    let p = Project::parse(argv[1].as_str()).or_else(|e| molt_err!(e.to_string()))?;

    let projects = interp.context::<HashMap<String, Project>>(ctx_id);
    projects.insert(p.name.clone(), p);
    // Return empty result
    molt_ok!()
}

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
            if trimmed == "" {
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
pub enum ParseError {
    #[error("duplicate attribute: {0}")]
    DuplicateAttribute(String),
    #[error("unknown attribute: {0}")]
    UnknownAttribute(String),
    #[error("expected: {0}")]
    Expected(String),
}
