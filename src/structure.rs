use serde::Deserialize;

const FILENAME: &str = "config.yaml"; 


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


#[derive(Debug, Deserialize)]
enum Entry {
    Docker(Vec<String>),
    Src(Vec<String>),
    Test(Vec<String>),
    Data(Vec<String>),
    Plot(Vec<String>),
    Logs(Vec<String>),
    Docs(Vec<String>),
    Plot(Vec<String>),
    Files(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct Data {
    Raw: String,
    Processed: String,
    Train: String,
    Test: String,
    Validate: String,
}

#[derive(Debug, Deserialize)]
struct Notebook {
    nb_data: String,
    nb_report: String,
    nb_model: String,
    nb_documentation: String,
    notebook_files: Vec<String>,
}

