use std::env;

mod graph;

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