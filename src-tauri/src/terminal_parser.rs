use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "terminal_grammar.pest"]
pub struct TerminalParser;