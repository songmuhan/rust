//! Parsing a shell command.
//!
//! Shell commands are text-based instructions that you can enter in a command-line interface (CLI)
//! to interact with operating systems (e.g. Linux) and others.
//! For example, you can use the `ls` command to list files in a directory.
//!
//! You will parse a given string consists of a small number of shell commands.

use itertools::Itertools;

/// Parse the string as a shell command.
///
/// Usually, a shell command is whitespace-separated array of strings.
/// ```text
/// cat file  -->  ["cat", "file"]
/// ```
/// But sometimes, you may want to include whitespaces in each argument.
/// In that case, you can use quotes.
/// ```text
/// ls 'VirtualBox VMs'  -->  ["ls", 'VirtualBox VMs']
/// ls VirtualBox' 'VMs  -->  ["ls", 'VirtualBox VMs']
/// ```
///
/// For simplicity, you may assume that the string only contains alphanumeric characters, spaces
/// (" "), and single quotes ("'").
///
/// See `test_shell` for more examples.
pub fn parse_shell_command(command: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut arg = command.split_whitespace().peekable();

    while let Some(current) = arg.next() {
        if current.starts_with('\'') {
            let mut new_arg = String::new();
            new_arg.push_str(current.trim_start_matches('\''));
            for next in arg.by_ref() {
                if next.ends_with('\'') {
                    new_arg.push(' ');
                    new_arg.push_str(next.trim_end_matches('\''));
                    break;
                } else {
                    new_arg.push_str(&(" ".to_owned() + next));
                }
            }
            result.push(new_arg);
        } else if current.ends_with('\'') {
            let mut new_arg = String::new();
            new_arg.push_str(current.trim_end_matches('\''));
            for next in arg.by_ref() {
                if next.starts_with('\'') {
                    new_arg.push(' ');
                    new_arg.push_str(next.trim_start_matches('\''));
                    break;
                } else {
                    new_arg.push_str(&(" ".to_owned() + next));
                }
            }
            result.push(new_arg);
        } else {
            result.push(current.to_string());
        }
    }
    result
}
