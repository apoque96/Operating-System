mod structs;
mod enums;
mod ast;

use std::sync::Mutex;

use structs::terminal_parser::TerminalParser;
use structs::terminal_parser::Rule;
use pest::Parser;

use crate::ast::build_line;
use crate::structs::fs_tree::FileSystem;

#[tauri::command]
fn parse_command(line: &str, fs: tauri::State<Mutex<FileSystem>>) -> String {
    let parsed = match TerminalParser::parse(Rule::line, line) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => return format!("parse error: {e}"),
    };

    let mut fs = fs.lock().unwrap();
    match build_line(parsed, &mut fs) {
        Ok(msg) => msg,
        Err(e)  => format!("error: {e}"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(FileSystem::new()))
        .invoke_handler(tauri::generate_handler![parse_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
