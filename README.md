# sl

**SIMPLE LS**

*simplified ls command*

> simply list everything in a directory

> no fancy stuff => just another (bad) ```ls``` clone

## Examples

![sl](https://github.com/Phydon/sl/assets/sl)

![sl_c](https://github.com/Phydon/sl/assets/sl_c)

![sl_l](https://github.com/Phydon/sl/assets/sl_l)

![sl_cHlF](https://github.com/Phydon/sl/assets/sl_cHlF)

![sl_Hd_p](https://github.com/Phydon/sl/assets/sl_Hd_p)

![sl_Hf_p](https://github.com/Phydon/sl/assets/sl_Hf_p)

![sl_cHlFf_p](https://github.com/Phydon/sl/assets/sl_cHlFf_p)



## Why?

* I am forced to work on windows
* -> only default shells (powershell/cmd) allowed
* -> no external programs allowed
* My solution => I write my own stuff as needed

*I don\`t recommend using this program.
If you can, use something like ```exa```*


## Usage

```
sl [OPTIONS] [COMMAND]

Commands:
  log, -L  Show content of the log file
  help     Print this message or the help of the given subcommand(s)

Options:
  -c, --colour       Show coloured output
  -d, --dirs         Show only dirs
  -f, --files        Show only files
  -F, --fullpath     Show the complete path instead of just the name
  -H, --hidden       Show hidden files
  -l, --long         Show more output
  -p, --path <PATH>  Add a path to a directory
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

## Installation
via Cargo or get the ![binary](https://github.com/Phydon/sl/releases)


## TODO

- more flags:
    - to output [size]
    - to sort output differently
    - show stats
