mod arg_parser;

use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use regex::Regex;
use arg_parser::SearchArgs;
use clap::Parser;
use colored::*;


struct Showables {dirs : bool, files: bool, links: bool}
fn main() {
    let args = SearchArgs::parse();

    // Parse arguments
    // Default values
    let mut search_term = "".to_string();
    let mut search_dir = ".".to_string();
    let mut recursion_str = "/**/";
    let mut pref_string = " ".to_owned();
    let mut current_show = Showables{dirs:true, files: true, links: true};


    // Change default values based on no of arguments

    if !args.dirs_only & !args.files_only & !args.links_only {
    } else {
        current_show.dirs = false;
        current_show.files = false;
        current_show.links = false;
        if args.files_only {
            current_show.files  = true;
            pref_string = pref_string + "files_only "};
        if args.dirs_only  {
            current_show.dirs = true;
            pref_string = pref_string + "dirs_only "};
        if args.links_only {
            current_show.links = true;
            pref_string = pref_string + "links_only "};
        }


    if args.first_option.is_some() & args.second_option.is_some() {
        search_term = args.second_option.unwrap();
        search_dir = args.first_option.unwrap();
    } else if args.first_option.is_some() & args.second_option.is_none() {
        search_term = args.first_option.unwrap();
    }

    if args.no_recursion {
        recursion_str = "/";
        pref_string = pref_string + "no_recursion ";
    }

    if args.case_sensitive {
        pref_string = pref_string + "case_sensitive ";
    }

    let match_string = format!("{}{}*{}*", search_dir, recursion_str, search_term);


    // Print the header
    let header = format!("[ DIR = {} | SEARCH = {} | PREF ={}]", &search_dir, &search_term, &pref_string).green();
    println!("{}", header);

    // Match Options for glob_with
    let options = MatchOptions {
        case_sensitive: args.case_sensitive,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    // GLOB
    for entry in glob_with(&match_string, options).unwrap() {
        if let Ok(path) = entry {
            print_path(path, &search_term, &current_show)
        }
    }

}

fn print_path(path:PathBuf, search_term: &String, to_show: &Showables) {
    // Create a regex using the search term
    let re = Regex::new(&format!("(?i){}", &search_term)).unwrap();

    // Match the section in the path string withe the search term using the regex
    let path_str = &path.display().to_string();
    let section = re.find(path_str).unwrap().as_str();

    // Replace the matched section with a colorized version
    let newpath = &path.display().to_string()
        .replace(&section.clone(), &section.clone().blue().to_string());

    if path.is_dir() & to_show.dirs {
        println!("dir  | {}", newpath)
    } else if path.is_symlink() & to_show.links{
        println!("link | {}", newpath)
    } else if path.is_file() & to_show.files{
        println!("file | {}", newpath)
    } else if !path.is_file() & !path.is_dir() & !path.is_symlink(){
        println!("???? | {}", newpath)
    }
}