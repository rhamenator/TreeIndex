use std::{env, fs, path::PathBuf};
use tree_index::{render_html, scan};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 2 {
        eprintln!("usage: tree-index DIRECTORY OUTPUT.html");
        std::process::exit(2);
    }
    let output = PathBuf::from(&args[1]);
    match scan(&args[0]).and_then(|entries| fs::write(&output, render_html(&args[0], &entries))) {
        Ok(()) => println!("wrote {}", output.display()),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
