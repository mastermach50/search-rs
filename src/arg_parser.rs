use std::process::exit;
use colored::*;

pub struct Pref {
    pub case_sensitive: bool,
    pub recursive: bool,
    pub show_dirs: bool,
    pub show_files: bool
}

pub fn parse_preferences(pref_keys: &Vec<String>) -> Pref {
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


fn help () {
    println!("Search [by MasterMach50]
    https://github.com/MasterMach50/search-rs
    
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
    println!("Search by MasterMach50 | version {}", version.blue());
    exit(0)
}
