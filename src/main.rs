use molt::types::*;
use molt::{molt_ok, Interp, check_args, ResultCode};
use molt_shell::repl;
use std::env;
use std::collections::HashMap;
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
    let p = Project::parse(argv[1].as_str())
    .or_else(|e| Err(ResultCode::Error(Value::empty())))?;

    let projects = interp.context::<HashMap<String, Project>>(ctx_id);
    projects.insert(p.name.clone(), p);
    
    // Return empty result
    molt_ok!()
}

struct Project {
    pub name: String,
}

impl Project {
    pub fn parse(raw: &str) -> Result<Self, SyntaxError> {

        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed == "" {
                continue;
            }
            let mut splitted = trimmed.split_whitespace();
            if let Some(attr) = splitted.next() {
                match attr {
                    "" => continue,
                    "name" => return Ok(Project{name: splitted.next().unwrap().to_owned()}),
                    x @ _ => return Err(SyntaxError::UnknownAttribute(x.to_owned())),
                }
            }

        }
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("unknown attribute: {0}")]
    UnknownAttribute(String)
}
