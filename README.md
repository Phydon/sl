# sl

**SIMPLE LS**

*simplified ls command*

> simply list everything in a directory

> no fancy stuff => just another (bad) ```ls``` clone

## Examples

![sl](https://github.com/Phydon/sl/blob/master/assets/sl.png)

![sl_c](https://github.com/Phydon/sl/blob/master/assets/sl_c.png)

![sl_l](https://github.com/Phydon/sl/blob/master/assets/sl_l.png)

![sl_cHlF](https://github.com/Phydon/sl/blob/master/assets/sl_cHlF.png)

![sl_Hd_p](https://github.com/Phydon/sl/blob/master/assets/sl_Hd_p.png)

![sl_Hf_p](https://github.com/Phydon/sl/blob/master/assets/sl_Hf_p.png)

![sl_cHlFf_p](https://github.com/Phydon/sl/blob/master/assets/sl_cHlFf_p.png)


## Usage

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

## Installation

### Windows

via Cargo or get the ![binary](https://github.com/Phydon/sl/releases)


## TODO

- more flags:
    - to output [size]
    - to sort output differently
    - show stats
