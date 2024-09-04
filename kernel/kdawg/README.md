# kawg

## Introduction

*****kawg** is on the progress.

#### Note

[libxcb](https://xcb.freedesktop.org/) should be installed for using the copy/paste commands of X11.

e.g: Install `libxcb1-dev` package for Debian/Ubuntu[\*](https://github.com/orhun/kdawg/issues/2) and `libxcb-devel` package for Fedora/openSUSE/Void Linux.

## Usage

```
kdawg [OPTIONS] [COMMAND]
```

### Options

```
-a, --accent-color <COLOR>  Set the accent color using hex or color name [default: white]
-c, --color <COLOR>         Set the main color using hex or color name [default: darkgray]
-t, --tickrate <MS>         Set the refresh rate of the terminal [default: 250]
-r, --reverse               Reverse the kernel module list
-u, --unicode               Show Unicode symbols for the block titles
-h, --help                  Print help information
-V, --version               Print version information
```

### Commands

```
sort  Sort kernel modules
```

#### Sort

```
kdawg sort [OPTIONS]
```

**Options:**

```
-s, --size       Sort modules by their sizes
-n, --name       Sort modules by their names
-d, --dependent  Sort modules by their dependent modules
-h, --help       Print help information
```

## Key Bindings

|                         |                                       |
| ----------------------- | ------------------------------------- |
| `[?], F1`               | Help                                  |
| `right/left, h/l`       | Switch between blocks                 |
| `up/down, k/j, alt-k/j` | Scroll up/down [selected block]       |
| `pgup/pgdown`           | Scroll up/down [kernel activities]    |
| `</>`                   | Scroll up/down [module information]   |
| `alt-h/l`               | Scroll right/left [kernel activities] |
| `ctrl-t/b, home/end`    | Scroll to top/bottom [module list]    |
| `alt-e/s`               | Expand/shrink the selected block      |
| `ctrl-x`                | Change the block position             |
| `ctrl-l/u, alt-c`       | Clear the kernel ring buffer          |
| `[d], alt-d`            | Show the dependent modules            |
| `[1]..[9]`              | Jump to the dependent module          |
| `[\], tab, backtab`     | Show the next kernel information      |
| `[/], s, enter`         | Search a kernel module                |
| `[+], i, insert`        | Load a kernel module                  |
| `[-], u, backspace`     | Unload the kernel module              |
| `[x], b, delete`        | Blacklist the kernel module           |
| `ctrl-r, alt-r`         | Reload the kernel module              |
| `m, o`                  | Show the options menu                 |
| `y/n`                   | Execute/cancel the command            |
| `c/v`                   | Copy/paste                            |
| `r, F5`                 | Refresh                               |
| `q, ctrl-c/d, ESC`      | Quit                                  |

## Features

### Features

Management actions about the Linux kernel should be applicable in kdawg for minimizing the dependence on to command line and other tools.

### Testing

kdawg should be tested and reported on different architectures for further development and support.

### GitHub

Support the development of my projects by supporting me on [GitHub Sponsors](https://github.com/sponsors/orhun).

## License

GNU General Public License ([3.0](https://www.gnu.org/licenses/gpl.txt))

## Copyright

Copyright © 2020-2024, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
