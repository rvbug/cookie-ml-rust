---
<p align="center"> 
<img height="50" widhth="50" src="https://github.com/rvbug/cookie-ml/assets/10928536/4f22cc2e-309f-4650-9695-0e855a2dd638" >
</p>

---

<p align="center"> 

  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"> 
  <img src="https://img.shields.io/badge/linting-pylint-yellowgreen">
</p>



# Introduction

All ML projects start with basic understanding of data and experiement with Jupyter notebook (mostly) and then gradually moving to a proper IDE, writing/reusing helper functions, seperating data_pipline code with data processing etc. It is chaotic for beginners but that is the beauty of it.


# About this project

Using a standard structure helps you focus on getting started immediately on any machine learning project.   

I tend to start using jupyter notebook to get an understanding of the data, the story it tries to tell, capturing all the details including references, links, tips and tricks, latest researech. These are important information to store during the initial days of your research on the project.


# Rust Installation


# Dependencies
- clap with Derive Feature
- serde
- serde_yaml


# Project Structure

The project structure is quite simple actually
![image](https://github.com/rvbug/cookie-ml/assets/10928536/e0785d48-c21b-42c6-84a7-de211e6687ca)


# Usage

It contains a rust script and a configurable folder structure in YAML format.
Extemely simple to use command line script.


```rust
$> cookie-ml-rust --h

// Usage: cookie-ml-rust [OPTIONS]

// Options:
//   -n, --name <NAME>  name of the ML project - default is ml-cookie-project [default: ml-cookie-project]
//   -p, --path <PATH>  path of the ML project [default: $HOME directory]
//   -v, --venv <VENV>  creates a virtual env if this flag is provided

```

## Arguments 
| short format | long format | description |
| --- | --- | --- |
| --h | --help | displays help |
| --n | --name | (optional) provide project name, defaults to "ml-cookie-cutter"  |
| --p | --path | (optional) provide project location, defaults to $HOME directory |
| --v | --venv | (optional) create virtual env. <br>Activate using `source {venv_path}/bin/activate` |




# Reference 
- [Rust](https://www.rust-lang.org/)
- [YAML](https://yaml.org/)  
- [Data Version Control](https://dvc.org/)    
- [Dags Hub](https://dagshub.com/)  
- [Jupyter Notebook](https://jupyter.org/)  
- [Docker](https://www.docker.com/)  
  
  
