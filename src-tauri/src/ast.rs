use std::error::Error;
use pest::iterators::Pair;
use crate::{
    enums::{cmd::Cmd, fs_node::FsNode},
    structs::{
        fs_tree::FileSystem, terminal_parser::Rule
    },
};


/// Parse a `command` pair into a `(Cmd, Vec<String>)` tuple.
fn build_cmd(pair: Pair<Rule>) -> Result<(Cmd, Vec<String>), Box<dyn Error>> {
    let keyword = pair.as_str().split_whitespace().next().unwrap_or("").to_string();
 
    let mut is_help = false;
    let mut args: Vec<String> = Vec::new();
 
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::help   => is_help = true,
            Rule::string => args.push(inner.as_str().to_string()),
            _            => {}
        }
    }
 
    let cmd = if is_help {
        Cmd::Help
    } else {
        match keyword.as_str() {
            "ls"    => Cmd::Ls,    "pwd"   => Cmd::Pwd,
            "mkdir" => Cmd::Mkdir, "cd"    => Cmd::Cd,
            "rm"    => Cmd::Rm,    "touch" => Cmd::Touch,
            "mv"    => Cmd::Mv,    "ps"    => Cmd::Ps,
            "run"   => Cmd::Run,   "kill"  => Cmd::Kill,
            "mem"   => Cmd::Mem,
            other   => return Err(format!("unknown command token: {other}").into()),
        }
    };
 
    Ok((cmd, args))
}


/// Walk a parsed `line` pair, execute each command against `fs`, and return
/// a single string describing what happened.
pub fn build_line(
    pair: Pair<Rule>,
    fs:   &mut FileSystem,
) -> Result<String, Box<dyn Error>> {
    let mut output: Vec<String> = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::command => {
                let (cmd, args) = build_cmd(child)?;
                let result      = exec_cmd(cmd, fs, &args)?;
                output.push(result);
            }
            Rule::pipe | Rule::EOI => {}
            _ => {}
        }
    }

    Ok(output.join("\n"))
}

fn exec_cmd(
    cmd:  Cmd,
    fs:   &mut FileSystem,
    args: &[String],
) -> Result<String, Box<dyn Error>> {

    let output = match cmd {

    
        Cmd::Ls => {
            let dir = fs.current_dir();
            if dir.children.is_empty() {
                "(empty)".to_string()
            } else {
                let mut entries: Vec<&str> =
                    dir.children.keys().map(String::as_str).collect();
                entries.sort();
                entries
                    .iter()
                    .map(|name| {
                        let tag = if dir.children[*name].is_dir() { "/" } else { "" };
                        format!("{name}{tag}")
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }

    
        Cmd::Pwd => fs.pwd_string(),

    
        Cmd::Mkdir => {
            let name = require_arg(args, 0, "mkdir")?;
            let dir  = fs.current_dir_mut();
            if dir.children.contains_key(name) {
                return Err(format!("mkdir: '{name}' already exists").into());
            }
            dir.children.insert(name.to_string(), FsNode::new_dir(name));
            format!("created directory '{name}'")
        }

    
        Cmd::Touch => {
            let name = require_arg(args, 0, "touch")?;
            let dir  = fs.current_dir_mut();
            if dir.children.contains_key(name) {
                return Err(format!("touch: '{name}' already exists").into());
            }
            dir.children.insert(name.to_string(), FsNode::new_file(name));
            format!("created file '{name}'")
        }

    
        Cmd::Cd => {
            let target = require_arg(args, 0, "cd")?;
            if target == ".." {
                if fs.cwd.is_empty() {
                    return Err("cd: already at root".into());
                }
                fs.cwd.pop();
            } else {
                let node = fs.current_dir()
                    .children.get(target)
                    .ok_or_else(|| format!("cd: '{target}' not found"))?;
                if !node.is_dir() {
                    return Err(format!("cd: '{target}' is not a directory").into());
                }
                fs.cwd.push(target.to_string());
            }
            format!("changed directory to '{}'", fs.pwd_string())
        }

    
        Cmd::Rm => {
            let name = require_arg(args, 0, "rm")?;
            let dir  = fs.current_dir_mut();
            if dir.children.remove(name).is_none() {
                return Err(format!("rm: '{name}' not found").into());
            }
            format!("removed '{name}'")
        }

    
        Cmd::Mv => {
            let src  = require_arg(args, 0, "mv (source)")?;
            let dest = require_arg(args, 1, "mv (destination)")?;
            let dir  = fs.current_dir_mut();
            let mut node = dir.children.remove(src)
                .ok_or_else(|| format!("mv: '{src}' not found"))?;
            node.name = dest.to_string();
            dir.children.insert(dest.to_string(), node);
            format!("renamed '{src}' to '{dest}'")
        }

    
        Cmd::Help => {
            "Available commands: ls, pwd, mkdir, cd, rm, touch, mv, ps, run, kill, mem, help"
                .to_string()
        }

    
        Cmd::Ps   => "ps: not yet implemented".to_string(),
        Cmd::Run  => "run: not yet implemented".to_string(),
        Cmd::Kill => "kill: not yet implemented".to_string(),
        Cmd::Mem  => "mem: not yet implemented".to_string(),
    };

    Ok(output)
}


fn require_arg<'a>(
    args: &'a [String],
    idx:  usize,
    cmd:  &str,
) -> Result<&'a str, Box<dyn Error>> {
    args.get(idx)
        .map(String::as_str)
        .ok_or_else(|| format!("{cmd}: missing argument").into())
}