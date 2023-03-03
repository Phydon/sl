# sl

**SIMPLE LS**

*simplified ls command*

> simply list everything in a directory
> no fancy stuff => for more use ```ls```

* show files just white (or your default normal terminal colour)
* show directories white bold
* everything else is italic and greyish dimmed
* by default it only lists names 
* accepts a path as an argument via the ```-p``` / ```--path``` flag

## Usage
=> not fully completed yet

```
sl [OPTIONS] [COMMAND]

Commands:
  log, -L  Show content of the log file
  help     Print this message or the help of the given subcommand(s)

Options:
  -p, --path <PATH>  Add a path to a directory
  -l, --long         Show more
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version
```

## TODO

- don`t show hidden files by default
- use flags:
    - to output [type, size, last modified]
    - to customize colours
    - sort output differently
    - to show hidden
    - to show the path and not only the name
    - to show idx per entry
