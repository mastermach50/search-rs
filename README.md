# Search-rs

A simple, in terminal file search utility written in rust.

Recursively searches for files and directories.

## From the help command
```
Search-RS
Search for files or directories recursively.

Usage: search.exe [OPTIONS] [SEARCH_DIR] [SEARCH_TERM]

Arguments:
  [SEARCH_DIR]   The directory to search in (default: . )
  [SEARCH_TERM]  The term to search for (default: * )

Options:
  -c, --case-sensitive    Case sensitive (default: false)
  -s, --single-directory  Single directory (i.e no recursion) (default: false)
  -d                      Show directories only (default: false)
  -f                      Show files only (default: false)
  -l                      Show links only only (default: false)
  -h, --help              Print help
  -V, --version           Print version
```