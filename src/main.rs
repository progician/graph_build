use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod graph;
mod ninja_file;


fn read_build_file(file_path: &Path) -> Result<graph::Graph, String> {
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }

    ninja_file::read(&contents)
}


fn check_graph(build_graph: &graph::Graph) -> Result<(), String> {
    for (_, build_command) in &build_graph.nodes {
        let input_file = Path::new(&build_command.input);
        if !input_file.exists() {
            return Err(format!("'{}', needed by '{}', missing and no known rule to make it",
                &build_command.input,
                &build_command.output,
            ));
        }
    }
    Ok(())
}


fn run_app() -> Result<(), String> {
    let cwd_path = match env::current_dir() {
        Ok(p) => p,
        Err(_) => {
            return Err("couldn't get current working directory".to_owned())
        }
    };

    let mut build_file = cwd_path.clone();
    build_file.push("build.ninja");
    if build_file.is_file() {
        let build_graph = read_build_file(&build_file)?;
        check_graph(&build_graph)?;
        Ok(())
    }
    else {
        Err("no build.ninja file found".to_owned())
    }
}


fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {}", err);
            1
        },
    });
}