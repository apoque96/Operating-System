mod terminal_parser;

use terminal_parser::TerminalParser;
use terminal_parser::Rule;
use pest::Parser;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn parse_command(line: &str) -> bool {
    TerminalParser::parse(Rule::line, line).is_ok()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![parse_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
