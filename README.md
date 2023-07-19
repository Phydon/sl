# 🗂️📑 sl

**SIMPLE LS**

*simplified ls command*

> simply list everything in a directory

> no fancy stuff => just another (bad) ```ls``` clone

## Examples

![sl](https://github.com/Phydon/sl/blob/master/assets/sl.png)

![sl_l](https://github.com/Phydon/sl/blob/master/assets/sl_l.png)

![sl_lc](https://github.com/Phydon/sl/blob/master/assets/sl_lc.png)


## Usage

### Short Usage

```
sl [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log  Show content of the log file
  help            Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Add a path to a directory

Options:
  -c, --colour    Show coloured output [aliases: color]
  -d, --dirs      Show only dirs [aliases: dir]
  -f, --files     Show only files [aliases: file]
  -F, --fullpath  Show the complete path instead of just the name
  -H, --hidden    Show hidden files [aliases: all]
  -l, --long      Show more output
  -h, --help      Print help (see more with '--help')
  -V, --version   Print version
```
### Long Usage
```
sl [OPTIONS] [PATH] [COMMAND]

Commands:
  log, -L, --log
          Show content of the log file
  help
          Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]
          Add a path to a directory

Options:
  -c, --colour
          Show coloured output

          [aliases: color]

  -d, --dirs
          Show only dirs

          [aliases: dir]

  -f, --files
          Show only files

          [aliases: file]

  -F, --fullpath
          Show the complete path instead of just the name

  -H, --hidden
          Show hidden files

          [aliases: all]

  -l, --long
          Additionaly display [type, size, last modified, read_only]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version  
```


## Installation

### Windows

via Cargo or get the ![binary](https://github.com/Phydon/sl/releases)
