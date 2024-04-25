use clap::{Parser};

/// clap has default help feature



#[derive(Parser)]
// #[command(author, version, about, long_about = None)]

struct Args {
    /// name of the ML project - default is ml-cookie-project
    #[arg(short, long, default_value = "ml-cookie-project")]
    name: Option<String>,

    /// path of the ML project
    #[arg(short, long, default_value = "$HOME")]
    path: Option<String>,

    /// venv flag helps create a defailt virtual env named "venv" 
    #[arg(short, long)]
    venv: Option<String>,

}



fn main() -> Result<(), String> {
    let args = Args::parse();

    println!("Hello, {:?} {:?}!", args.name, args.path);
    if let Some(name) = args.name.as_deref() {
        println!("Hello, {:?}!", name);
    } else {
        println!("no name is given");
    }

    Ok(())
}




