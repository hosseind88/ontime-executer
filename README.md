
# ontime-executer

A tiny rust package that lets you run your file on the date/time and with command that you have specified

```
ontimeexecuter 0.1.0
Hossein Dindar <hosseind2017@gmail.com>
A tiny rust package that lets you run your file on the date/time and with command that you have specified

USAGE:
    ontimeexecuter [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --datetime <datetime>                Sets the date/time that should run this program, please set it in this
                                             format dd-mm-yyyy hh:mm:ss
    -f, --file-path <file-path>              Sets the path of file that should be executed, e.g ./script.sh
    -r, --runner-command <runner-command>    Sets the runner command that should run program with, now it just supports sh and node

```
## Installation

Via cargo

```
cargo install ontimeexecuter
``` 

## Example

```
ontimeexecuter --datetime "26-07-2021 20:44:45"  --file-path ./script.js --runner-command node
```  