use serde::Deserialize;

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

