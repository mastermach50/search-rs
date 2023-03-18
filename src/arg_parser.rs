use clap::{Parser};
use colored::*;

#[derive(Debug, Parser)]
#[command(
    about = ("Search-RS".green().bold().underline().to_string())+
    "\nSearch for files or directories recursively.",
    version,
    author)]
pub struct SearchArgs{
    /// The directory to search in (default: . )
    #[arg(name = "SEARCH_DIR")]
    pub first_option: Option<String>,

    /// The term to search for (default: * )
    #[arg(name = "SEARCH_TERM")]
    pub second_option: Option<String>,

    /// Case sensitive (default: false)
    #[arg(short, long = "case-sensitive", default_value_t = false)]
    pub case_sensitive: bool,

    /// Single directory (i.e no recursion) (default: false)
    #[arg(short = 's', long = "single-directory", default_value_t = false)]
    pub no_recursion: bool,

    /// Show directories only (default: false)
    #[arg(short = 'd', default_value_t = false)]
    pub dirs_only: bool,

    /// Show files only (default: false)
    #[arg(short = 'f', default_value_t = false)]
    pub files_only: bool,

    /// Show links only only (default: false)
    #[arg(short = 'l', default_value_t = false)]
    pub links_only: bool,
}