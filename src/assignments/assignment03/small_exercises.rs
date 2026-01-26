//! Small problems.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::{default, fmt};

use itertools::Itertools;
use rayon::collections::hash_set;

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Sun => DayOfWeek::Mon,
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri => DayOfWeek::Mon,
        DayOfWeek::Sat => DayOfWeek::Mon,
    }
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```ignore
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```ignore
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    let mut sort: Vec<isize> = values.clone();
    sort.sort();
    if sort.is_empty() {
        None
    } else if sort.len() % 2 == 0 {
        let element = sort.len() / 2;
        Some(sort[element])
    } else {
        let element = (sort.len() + 1) / 2 - 1;
        Some(sort[element])
    }
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash
/// map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let mut map: HashMap<isize, isize> = HashMap::new();
    for element in values {
        let count = map.entry(element).and_modify(|v| *v += 1).or_insert(1);
    }

    map.iter()
        .max_by(|(k1, v1), (k2, v2)| match v1.cmp(v2) {
            Ordering::Equal => k2.cmp(k1), // reverse so smaller key wins in max_by
            other => other,
        })
        .map(|(k, _)| *k)
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig
/// Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end
///    of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    let mut move_str = input.clone();
    let ch: char = input.chars().next().unwrap();
    if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
        return input + "hay";
    }
    for i in input.chars() {
        if i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u' {
            return move_str + "ay";
        } else {
            let mut chars = move_str.chars();
            let _ = chars.next().ok_or(0);
            move_str = chars.as_str().to_owned();
            move_str.push(i);
        }
    }

    input + "hay"
}

/// Converts HR commands to the organization table.
///
/// - Map from department -> set of employees
/// - Empty departments are removed / do not appear
/// - Commands:
///   - "Add {person} to {department}"
///   - "Remove {person} from {department}"
///   - "Move {person} from {department} to {department}"
/// - Ignore invalid / non-executable commands
/// - No spaces in names or department identifiers
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut org: HashMap<String, HashSet<String>> = HashMap::new();

    for cmd in commands {
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        match parts.as_slice() {
            // Add Sally to Sales
            ["Add", person, "to", dept] => {
                // Get or create the set for the department, then insert person
                let _unused = org
                    .entry((*dept).to_string())
                    .or_default()
                    .insert((*person).to_string());
            }

            // Remove Jeehoon from Sales
            ["Remove", person, "from", dept] => {
                if let Some(set) = org.get_mut(*dept) {
                    let _unused = set.remove(*person);
                    if set.is_empty() {
                        // Remove empty department
                        let _unused = org.remove(*dept);
                    }
                }
                // If dept doesn't exist, or person not in dept -> ignore
            }

            // Move Amir from Engineering to Sales
            ["Move", person, "from", from_dept, "to", to_dept] => {
                // Must exist in from_dept first; otherwise ignore
                let mut moved = false;
                if let Some(from_set) = org.get_mut(*from_dept) {
                    if from_set.remove(*person) {
                        moved = true;
                        if from_set.is_empty() {
                            // Removing from empty department after move
                            // (we'll remove it below after borrowing ends)
                        }
                    }
                }

                if moved {
                    let _unused = org
                        .entry((*to_dept).to_string())
                        .or_default()
                        .insert((*person).to_string());

                    // Now clean up `from_dept` if it exists and became empty
                    // (Need a second lookup because of previous mutable borrow scope)
                    if let Some(from_set) = org.get_mut(*from_dept) {
                        if from_set.is_empty() {
                            let _unused = org.remove(*from_dept);
                        }
                    }
                }
            }

            // Anything else is ignored
            _ => { /* ignore non-matching or malformed commands */ }
        }
    }

    org
}

/// Events in a text editor.
#[derive(Debug)]
pub enum TypeEvent {
    /// A character is typed.
    Type(char),
    /// The last character is removed.
    Backspace,
    /// The whole string is copied to the clipboard.
    Copy,
    /// The string in the clipboard is appended.
    Paste,
}

/// Starting from an empty string and an empty clipboard,
/// processes the given `events` in order and returns the resulting string.
///
/// See the test function `test_editor` for examples.

pub fn use_editor(events: Vec<TypeEvent>) -> String {
    let mut text = String::new();
    let mut clipboard = String::new();

    for event in events {
        match event {
            TypeEvent::Type(ch) => {
                text.push(ch);
            }
            TypeEvent::Backspace => {
                // Remove last Unicode scalar if any
                if let Some(last) = text.pop() {};
            }
            TypeEvent::Copy => {
                // Copy entire current buffer to clipboard
                // Avoid new allocation strategy if you prefer:
                // clipboard.clear();
                // clipboard.push_str(&text);
                clipboard = text.clone();
            }
            TypeEvent::Paste => {
                // Append clipboard contents
                text.push_str(&clipboard);
            }
        }
    }

    text
}
