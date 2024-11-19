---
<p align="center"> 
<img height="50" widhth="50" src="https://github.com/rvbug/cookie-ml/assets/10928536/4f22cc2e-309f-4650-9695-0e855a2dd638" >
</p>

---

<p align="center"> 
  <img height=22 src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"> <img src="https://img.shields.io/badge/linting-rust-yellowgreen">
</p>



# Introduction

All ML projects start with basic understanding of data and experiement with Jupyter notebook (mostly) and then gradually moving to a proper IDE, writing/reusing helper functions, seperating data_pipline code with data processing, preparing for MLOps etc. It is chaotic for beginners but that is the beauty of it.


# About this project (Work in progress)

Using a standard structure helps you focusing on getting started immediately on any machine learning project.   

I tend to start using jupyter notebook to get an understanding of the data, the story it tries to tell, capturing all the details including references, links, tips and tricks, latest researech. These are important information to store during the initial days of your research on the project.

This repo helps you to setup your project and folder structure via a script.


# Python Version: 
If you want to use completed python version of this project, use this repo [Cookie ML Python version](https://github.com/rvbug/cookie-ml), 

# IDE
My prefered IDE is [Neovim](https://neovim.io/), if you like to configure for Rust developement, take a look at my [Repo](https://github.com/rvbug/neovim)

![image](https://github.com/rvbug/cookie-ml-rust/assets/10928536/97631e0a-66a1-4414-badb-a6cdb94f62a5)


# Rust Installation

To install rust
```bash
$> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Project Structure

The project structure is quite simple actually
![image](https://github.com/rvbug/cookie-ml/assets/10928536/e0785d48-c21b-42c6-84a7-de211e6687ca)


# Usage

It contains a rust script and a configurable folder structure in YAML format.
Extemely simple to use command line script.


```rust
$> cargo run -- --help
Builds a default ML structure

Usage: cookie-ml [OPTIONS]

Options:
  -n, --name <NAME>  name of the ML project - default is ml-cookie-project [default: ml-cookie-project]
  -p, --path <PATH>  path of the ML project [default: $HOME]
  -v, --venv <VENV>  venv flag helps create a defailt virtual env named "venv"
  -h, --help         Print help
  -V, --version      Print version
```

## Arguments 
| short format | long format | description |
| --- | --- | --- |
| --h | --help | displays help |
| --n | --name | (optional) provide project name, defaults to "ml-cookie-cutter"  |
| --p | --path | (optional) provide project location, defaults to $HOME directory |
| --v | --venv | (optional) create virtual env. <br>Activate using `source {venv_path}/bin/activate` |


## Run
```bash
cargo run -- --name my-rust-ml-project
```

## Documentation   

```rust
cargo doc --no-deps --open
```  

## Test

#### Run all tests
`cargo test`

#### Run with output
`cargo test -- --nocapture`

#### Run a specific test
`cargo test test_complete_project_creation`

#### Run tests with coverage (requires cargo-tarpaulin)
`cargo tarpaulin`  


# Reference 
- [Rust](https://www.rust-lang.org/)
- [YAML](https://yaml.org/)  
- [Data Version Control](https://dvc.org/)    
- [Dags Hub](https://dagshub.com/)  
- [Jupyter Notebook](https://jupyter.org/)  
- [Docker](https://www.docker.com/)  
  
  
