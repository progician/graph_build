fn run_app() -> Result<(), i32> {
    Err(1)
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => err,
    });
}