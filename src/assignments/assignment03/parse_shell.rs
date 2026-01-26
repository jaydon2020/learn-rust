//! Parsing a shell command.
//!
//! Shell commands are text-based instructions that you can enter in a command-line interface (CLI)
//! to interact with operating systems (e.g. Linux) and others. For example, you can use the `ls`
//! command to list files in a directory.
//!
//! You will parse a given string consists of a small number of shell commands.

use core::prelude::v1;

use itertools::Itertools;

/// Parse the string as a shell command.
///
/// Usually, a shell command is whitespace-separated array of strings.
///
/// ```text
/// cat file  -->  ["cat", "file"]
/// ```
///
/// But sometimes, you may want to include whitespaces in each argument.  In that case, you can use
/// quotes.
///
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
    let mut result: Vec<String> = Vec::new();
    let v: Vec<&str> = command.splitn(2, ' ').collect();
    let mut buf: String;

    result.push(v[0].to_string());

    match v[1].find('\'') {
        Some(0) => {
            buf = v[1].to_string();
            if let Some(x) = buf.pop() {}
            let _unused = buf.remove(0);
            result.push(buf);
        }
        Some(_) => {
            let tmp: String = v[1].split('\'').collect();
            result.push(tmp);
        }
        None => {
            let tmp: Vec<String> = v[1].split_whitespace().map_into().collect();
            result.extend(tmp);
        }
    }

    result
}
