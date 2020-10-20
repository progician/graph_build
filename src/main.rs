use std::env;

fn run_app() -> Result<(), i32> {
    let cwd_path = match env::current_dir() {
        Ok(p) => p,
        Err(_) => {
            eprint!("error: couldn't get current working directory");
            return Err(1)
        }
    };

    let mut build_file = cwd_path.clone();
    build_file.push("build.ninja");
    if build_file.is_file() {
        Ok(())
    }
    else {
        eprint!("error: no build.ninja file found!");
        Err(1)
    }
}


fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => err,
    });
}