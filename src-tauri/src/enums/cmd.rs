#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Ls, Pwd, Mkdir, Cd, Rm, Touch, Mv, Ps, Run, Kill, Mem, Help,
}

// impl Cmd {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             Cmd::Ls    => "ls",    Cmd::Pwd   => "pwd",
//             Cmd::Mkdir => "mkdir", Cmd::Cd    => "cd",
//             Cmd::Rm    => "rm",    Cmd::Touch => "touch",
//             Cmd::Mv    => "mv",    Cmd::Ps    => "ps",
//             Cmd::Run   => "run",   Cmd::Kill  => "kill",
//             Cmd::Mem   => "mem",   Cmd::Help  => "help",
//         }
//     }
// }