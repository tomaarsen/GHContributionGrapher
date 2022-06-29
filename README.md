
# GitHub Contribution Grapher
A command-line program to graph GitHub Contributions, written in Rust.
![image](https://user-images.githubusercontent.com/37621491/176441393-1c7bc9b5-560b-423e-b397-bf0afa0e3db6.png)

### How to use
#### Setting up
This program may be set up using:
```
cargo build
```
Afterwards, the `target/debug/gh.exe` file can be made globally executable, for example by placing `target/debug` on the PATH on Windows. Then, `gh` will become usable like so:
```
> gh --help
gh 0.1.0
Tom Aarsen
A simple CLI program to graph GitHub Contributions

USAGE:
    gh.exe <USER>

ARGS:
    <USER>    

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```
For example, the program can be executed with:
```
gh tomaarsen
```
Which produces the image shown at the top of this page.

#### Additional requirements
Beyond that, this project uses a `GITHUB_TOKEN` environment variable, which can be set in your [GitHub Settings](https://github.com/settings/tokens) with the `read:user` scope.
