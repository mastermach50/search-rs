mod arg_parser;

use glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use regex::Regex;
use arg_parser::SearchArgs;
use clap::Parser;
use colored::*;

struct Showables {
    dirs : bool,
    files: bool,
    links: bool}

fn main() {
    let args = SearchArgs::parse();

    // Parse arguments
    // Default values
    let mut search_term = "".to_string();
    let mut search_dir = ".".to_string();
    let mut recursion_str = "/**/";
    let mut pref_string = " ".to_string();
    let mut to_show = Showables{dirs:true, files: true, links: true};


    // Change default values based on arguments

    // Set whether to show files / dirs / symlinks
    if !args.dirs_only & !args.files_only & !args.links_only {
    } else {
        to_show.dirs = false;
        to_show.files = false;
        to_show.links = false;
        if args.files_only {
            to_show.files  = true;
            pref_string += "files_only "};
        if args.dirs_only  {
            to_show.dirs = true;
            pref_string += "dirs_only "};
        if args.links_only {
            to_show.links = true;
            pref_string += "links_only "};
        }

    // Set the search term and search directory
    if args.first_option.is_some() & args.second_option.is_some() {
        search_term = args.second_option.unwrap();
        search_dir = args.first_option.unwrap();
    } else if args.first_option.is_some() & args.second_option.is_none() {
        search_term = args.first_option.unwrap();
    }

    // Set whether recursion should be allowed
    if args.no_recursion {
        recursion_str = "/";
        pref_string += "no_recursion ";
    }

    // Set whether search should be case sensitive
    if args.case_sensitive {
        pref_string += "case_sensitive ";
    }

    // Create a string to match to
    let match_string = format!("{}{}*{}*", search_dir, recursion_str, search_term);


    // Print the header
    if !args.no_decorations {
        let header = format!("[ DIR = {} | SEARCH = {} | PREF ={}]", &search_dir, &search_term, &pref_string).green();
        println!("{}", header);
    }

    // MatchOptions for glob_with
    let options = MatchOptions {
        case_sensitive: args.case_sensitive,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut result_count: u32 = 0;

    // GLOB
    for path in glob_with(&match_string, options).unwrap().flatten() {
        // Print the coloured paths
        if path.is_file() & to_show.files{
            print_path(path, &search_term, &args.no_decorations, "file");
            result_count +=1;
        } else if path.is_dir() & to_show.dirs {
            print_path(path, &search_term, &args.no_decorations, "dir ");
            result_count +=1;
        } else if path.is_symlink() & to_show.links{
            print_path(path, &search_term, &args.no_decorations, "link");
            result_count +=1;
        } else if !path.is_file() & !path.is_dir() & !path.is_symlink(){
            print_path(path, &search_term, &args.no_decorations, "????");
            result_count +=1;
        }
    }
    // Print the total number of results
    if !args.no_decorations {
        let result_num = format!("[{} Results]", &result_count.to_string()).green();
        println!("{}", result_num);
    }

}

fn print_path(path:PathBuf, search_term: &String, no_decorations: &bool, path_type: &str) {

    if !no_decorations {
        // Create a regex using the search term
        let re = Regex::new(&format!("(?i){}", &search_term)).unwrap();

        // Match the section in the path string with the search term using the regex created
        let path_str = &path.display().to_string();
        let section = re.find(path_str).unwrap().as_str();

        // Replace the matched section with a colorized version of it
        let newpath = &path.display().to_string()
            .replace(section, &section.blue().to_string());

        // Print the coloured path
        println!("{} | {}", path_type, newpath);
    } else { 
        println!("{}", &path.display().to_string());
    }

}