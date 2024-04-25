// use clap::{Parser, Command, ArgMatches};
 use clap::{Parser};

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


    // #[arg(long)]
    // author: Option<String>,
    //
    //
    // #[arg[long]]
    // about: Option<String>,
    //
}



fn main() {
    // println!("Hello, {:?} {:?}!", args.name, args.path);
    let args = Arguments::parse();
    println!("\n");

    // let matches = Command::new("ML cookie cutter structure")
    //     .version("1.0")
    //     .author("rvbug")
    //     .about("Builds a default ML structure").get_matches();



    match args.name {
        Some(name) => {
            println!("Project Name : {:?}", name);
        }
        None => {
            println!("No value providedi for project name");
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
            println!("venv provided is {}", venv);
        }
        None => { 
            println!("virtual env will not be created, create it manually");
        }
    }

    // match args.author {
    //     Some(author) => {
    //         println!("Author : {}", author);
    //     }
    //     None => { 
    //     }
    // }


}
