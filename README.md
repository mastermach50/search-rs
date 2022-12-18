# Search-rs

A simple, in terminal file search utility written in rust.

Recursively searches for files and directories.

## From the help command
```
Search [by MasterMach50]
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
    -h         Same as --help
```