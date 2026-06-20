mod utils;

use std::process::Command;
use std::{env, os, path, vec};
use std::ptr::{null, read};

use utils::environment;

use crate::utils::environment::Environment;

// D:\Research\ABNT\templetes


fn create_pdf(path:&str , filename:&str){
    let template_path = r"D:\Research\ABNT\templetes\template.tex";

    let is_istall = Command::new("quarto")

    .arg("--version")
    .env("PATH",path)
    .status();

    if (is_istall.is_err()){
        print!("[ERROR] Quarto was not found. Please install it first.");
    }

    let status = Command::new("quarto")
    .args(["render", "--clear"])
    .env("PATH", path)
    .status();

    if status.is_err() {
        println!("[ERROR] failed to clear render");
        return;
    }

    let args:Vec<String> = Environment::build(filename, path, template_path);

    let render_status = Command::new("quarto").args(args)
    .env("PATH", path)
    .status();

    if render_status.is_err() {
        println!("[ERROR] failed to clear render");
        return;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).map(|s| s.as_str());
    let filename = args.get(2).map(|s| s.as_str());

    let path = match path {
        Some(p) => p,
        None => {
            println!("[ERROR] missing path");
            return;
        }
    };

    let filename = match filename {
        Some(f) => f,
        None => {
            println!("[ERROR] missing filename");
            return;
        }
    };

    create_pdf(path, filename);
}