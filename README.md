
<h1 align="center">
  <br>
  <a href="https://github.com/GmsGarcia/garry"><img src="https://static.wikia.nocookie.net/herois/images/9/9e/Gary_looking_up_stock_art.png/revision/latest?cb=20221012173246&path-prefix=pt-br" alt="Garry" width="200"></a>
  <br>
  Garry
  <br>
</h1>

<h3 align="center">CLI tool developed with Rust 🦀</h3>

<p align="center">
  <a href="#warning">Warning</a> •
  <a href="#description">Description</a> •
  <a href="#how-to-use">How To Use</a>
</p>

## Warning
This app stores all the data unencryped in a local file.

## Description

Garry is a CLI tool to store key-value pairs. 

## How To Use

### Installation

Build the program with `cargo build` and move the `debug/target` directory to `/usr/local/bin/garry`.
Create a symbolik link with the following command: `sudo ln -s /usr/local/bin/garry/garry /bin/<command-prefix*>`

### Commands

`<command-prefix> get <PAT_NAME>` - copies the PAT key to the clipboard

`<command-prefix> set <PAT_NAME> <PAT_VALUES>` - sets up a new PAT entry

`<command-prefix> delete <PAT_NAME>` - copies the PAT key to the clipboard

`<command-prefix> renew <PAT_NAME> <PAT_VALUE>` - renews a PAT key to the clipboard

---

> GitHub [@GmsGarcia](https://github.com/GmsGarcia)
