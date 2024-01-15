//! Small problems.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::sync::Arc;
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
        DayOfWeek::Fri | DayOfWeek::Sat | DayOfWeek::Sun => DayOfWeek::Mon,
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
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
    let mut v = values;
    v.sort();
    if v.len() % 2 == 0 {
        v.get(v.len() / 2).copied()
    } else {
        v.get((v.len() + 1) / 2 - 1).copied()
    }
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let mut map = std::collections::HashMap::new();
    for value in &values {
        *map.entry(value).or_insert(0) += 1;
    }
    let mut max = 0;
    let mut key = None;
    for value in &values {
        if map[&value] > max {
            key = Some(*value);
            max = map[&value];
        }
    }
    key
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end of the word and add "ay".
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
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut chars = input.chars();
    let first_char = chars.next().unwrap();

    if vowels.contains(&first_char) {
        format!("{}hay", input)
    } else {
        let mut end = first_char.to_string();
        while let Some(next) = chars.next() {
            if vowels.contains(&next) {
                let remain = chars.collect::<String>();
                return format!("{next}{remain}{end}ay");
            } else {
                end.push(next);
            }
        }
        format!("{end}ay")
    }
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```ignore
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```ignore
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add {person} to {department}", "Remove {person} from {department}", and "Move {person} from {department} to {department}".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut organization: HashMap<String, HashSet<String>> = HashMap::new();

    for command in commands {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let person = parts[1];
        let department = parts[3];
        match parts[0] {
            "Add" => {
                let _ = organization
                    .entry(department.to_string())
                    .or_default()
                    .insert(person.to_string());
            }
            "Remove" => {
                if let Some(dept_employees) = organization.get_mut(department) {
                    let _ = dept_employees.remove(person);
                }
            }
            "Move" => {
                let old_department = parts[3];
                let department = parts[5];
                if let Some(old_dept_employees) = organization.get_mut(old_department) {
                    let _ = old_dept_employees.remove(person);
                    let _ = organization
                        .entry(department.to_string())
                        .or_default()
                        .insert(person.to_string());
                }
            }
            _ => {}
        }
    }

    organization.retain(|_, employees| !employees.is_empty());
    organization
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
    let mut buf = String::new();
    let mut clipboard = String::new();
    for event in events {
        match event {
            TypeEvent::Backspace => {
                let _ = buf.pop();
            }
            TypeEvent::Copy => {
                clipboard = buf.clone();
            }
            TypeEvent::Paste => {
                buf.push_str(&clipboard);
            }
            TypeEvent::Type(c) => {
                buf.push(c);
            }
        }
    }
    buf
}
