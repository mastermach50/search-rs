use std::{env, process::exit};
use glob::{glob_with, MatchOptions};

struct Pref {
    case_sensitive: bool,
    recursive: bool,
    show_dirs: bool,
    show_files: bool
}

fn help () {
println!("Search [by MasterMach50]

Search for files or directories recursively.

Format:
    search [Directory] [Keyword] [Options]
    search [Keyword] [Options]               (searches in cwd)
    search [Options]                         (lists all files and directories in cwd)

Options:
    -c         Case Sensitive
    -s         Single Directory (no recursion)

    -f         Show files only
    -d         Show directories only
    -f -d      Same as not using -d and -f
    
    --version  Show version details and exit
    --help     Show Help and exit
    -h         Same as --help");
    exit(0);
}

fn show_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Search by MasterMach50 | version {}", version);
    exit(0)
}

fn parse_preferences(pref_keys: &Vec<String>) -> Pref {
    // Parse preferences
    let mut current_prefs = Pref{
        case_sensitive: false,
        recursive: true,
        show_dirs: false,
        show_files: false
    };

    if pref_keys.contains(&String::from("--help")) | pref_keys.contains(&String::from("-h")) {
        help()
    }
    if pref_keys.contains(&String::from("--version")) {
        show_version()
    }
    if pref_keys.contains(&String::from("-c")) {
        current_prefs.case_sensitive = true
    }
    if pref_keys.contains(&String::from("-s")) {
        current_prefs.recursive = false
    }
    if pref_keys.contains(&String::from("-d")) {
        current_prefs.show_dirs = true;
    }
    if pref_keys.contains(&String::from("-f")) {
        current_prefs.show_files = true;
    }
    if !pref_keys.contains(&String::from("-f")) & !pref_keys.contains(&String::from("-d")) {
        current_prefs.show_dirs = true;
        current_prefs.show_files = true;
    }

    return current_prefs;
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
        command.insert(1, String::from("."));
        command.insert(2, String::from(""));
    }
    if command.len() == 2 {
        command.insert(1, String::from("."));
    }
    
    let search_term = command.pop().unwrap();
    let search_path = command.pop().unwrap();

    let current_prefs = parse_preferences(&pref_keys);

    let mut recursor_string = "/**/";
    let mut search_string = format!("*{}*", search_term);

    if current_prefs.recursive == false {
        recursor_string = "/"
    }
    if search_term == "" {
        search_string = String::from("*");
    }


    // Get paths and search for file
    println!("[ DIR = {} | SEARCH = {} | PREF = {} ]", &search_path, &search_term, &pref_keys.concat());

    let options = MatchOptions {
        case_sensitive: current_prefs.case_sensitive,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with(&format!("{}{}{}", search_path, recursor_string, search_string), options).unwrap() {
        if let Ok(path) = entry {
            if path.is_dir() & current_prefs.show_dirs {
                println!("DIR  | {}", path.display())
            } else if path.is_symlink() & current_prefs.show_files {
                println!("LINK | {}", path.display())
            } else if path.is_file() & current_prefs.show_files{
                println!("FILE | {}", path.display())
            } else if !path.is_dir() & !path.is_file() & !path.is_symlink() {
                println!("???? | {}", path.display())
            }
        }
    }
}
