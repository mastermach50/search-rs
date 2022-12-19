use std::{env, path::PathBuf};
use glob::{glob_with, MatchOptions};
use colored::*;
use regex::Regex;

mod arg_parser;
use crate::arg_parser::{Pref, parse_preferences};

fn print_path(path:PathBuf, current_prefs: &Pref, search_term: &String) {
    // Create a regex using the search term
    let re = Regex::new(&format!("(?i){}", &search_term)).unwrap();

    // Match the section in the path string withe the search term using the regex
    let path_str = &path.display().to_string();
    let section = re.find(path_str).unwrap().as_str();

    // Replace the matched section with a colorized version
    let newpath = &path.display().to_string()
        .replace(&section.clone(), &section.clone().blue().to_string());

    if path.is_dir() {
        if current_prefs.show_dirs {println!("DIR  | {}", newpath)}
    } else if path.is_symlink() {
        if current_prefs.show_files {println!("LINK | {}", newpath)}
    } else if path.is_file(){
        if current_prefs.show_files {println!("FILE | {}", newpath)}
    } else {
        println!("???? | {}", newpath)
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    // Parse arguments
    let mut pref_keys:Vec<String> = Vec::new();
    let mut command:Vec<String> = Vec::new();

    for arg in &args {
        if &arg[0..1] == "-" {
            pref_keys.push(String::from(arg));
        } else {
            command.push(String::from(arg))
        }
    }
    
    if command.len() == 1 {
        command.insert(1, ".".to_string());
        command.insert(2, "".to_string());
    }
    if command.len() == 2 {
        command.insert(1, ".".to_string());
    }
    
    let search_term = command.pop().unwrap();
    let search_path = command.pop().unwrap();

    let current_prefs = parse_preferences(&pref_keys);

    let mut recursor_string = "/**/";
    let mut search_string = format!("*{}*", search_term);

    if !current_prefs.recursive {
        recursor_string = "/"
    }
    if search_term == "" {
        search_string = String::from("*");
    }


    // Get paths and search for file
    let header = String::from(format!("[ DIR = {} | SEARCH = {} | PREF = {} ]", &search_path, &search_term, &pref_keys.concat())).green();
    println!("{}", header);

    let options = MatchOptions {
        case_sensitive: current_prefs.case_sensitive,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with(&format!("{}{}{}", search_path, recursor_string, search_string), options).unwrap() {
        if let Ok(path) = entry {
            print_path(path, &current_prefs, &search_term)
        }
    }
}
