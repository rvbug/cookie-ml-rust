// use clap::{Parser, Command, ArgMatches};
use clap::{Parser};
use yaml_rust2::YamlLoader;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]

#[command(name="ML cookie cutter structure")]
#[command(version="1.0")]
#[command(about="Builds a default ML structure", long_about = None)]
#[command(author="rvbug")]



#[derive(Debug)]
struct Arguments {
    /// name of the ML project - default is ml-cookie-project
    // #[arg(short="--n", long, default_value = "ml-cookie-project")]
    #[arg(long="name", default_value = "ml-cookie-project")]
    name: Option<String>,

    /// path of the ML project
    #[arg(long, default_value = "$HOME")]
    path: Option<String>,

    /// venv flag helps create a defailt virtual env named "venv" 
    #[arg(long)]
    venv: Option<String>,

}

const FILENAME: &str = "config.yaml"; 


fn load_yaml(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(format!("{}", FILENAME))?;
    let mut data = String::new();
    
    match f.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }

    // display the content of the file!()
    println!("{:?}", data);

    Ok(data)

}

// fn load_yaml(filename : &str) {//-> Result<(), String> {
//
//     let file = match File::open(filename) {
//         Ok(file) => println!("{:?}", file),
//         Err(err) => println!("{:?}", err), 
//         // Err(err) => return Err(err.to_string()),
//     };
// }


 fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    println!("\n");

    match args.name {
        Some(name) => {
            println!("Project Name : {:?}", name);
        }
        None => {
            println!("No value provided for project name");
        }
    }

    match args.path {
        Some(path) => {
            println!("Path : {}", path);
        }
        None => { 
            println!("No value provided for path");
        }
    }
    match args.venv {
        Some(venv) => {
            println!("venv : {}", venv);
        }
        None => { 
            println!("virtual env will not be created, create it manually");
        }
    }

    println!("calling load_yaml() function to load the yaml file");

    // return the contents of the YAML 
    let contents = load_yaml(&FILENAME);

    let mut f = File::open(format!("{}", FILENAME))?;
    let mut data = String::new(); 
    match f.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }
    println!("{:?}", data);

    Ok(())
    

}





