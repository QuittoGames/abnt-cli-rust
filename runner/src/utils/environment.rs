pub struct Environment;
use std::path::Path;
use std::process::Command;
use std::{env, os, path, vec};
use std::ptr::{null, read};

impl Environment{
    pub fn get_bibliografy(template_path:&str) -> Vec<String>{        
        let output = Command::new("cmd")
            .args([
                "/C",
                &format!(r#"cd /d "{}" && dir | findstr ".bib""#, template_path),
            ])
            .output()
            .unwrap();

        if (output.stdout.is_empty()) {
            return vec![];
        }

        let bib_name = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

        vec![
            "--metadata".to_string(),
            format!("bibliography={}", bib_name),
        ]
    }

    pub fn get_template(template_path:&str) -> Vec<String>{
        let path = Path::new(template_path).exists();

        if (path){
            return vec![String::from("--template"), template_path.to_string()];
        }
        return vec![];
    }

    pub fn build(filename:&str, path:&str,template_path:&str) -> Vec<String>{
        let mut args:Vec<String> = vec!["render".to_string(),filename.to_string()];

        let mut bib:Vec<String> = Environment::get_bibliografy(path);
        
        let mut template:Vec<String> = Environment::get_template(template_path);

        if !(bib.is_empty()){
            for i in bib{
                args.push(i);
            }
        }

        if !(template.is_empty()){
            for i in template{
                args.push(i);
            }
        }
        return args; 
    }
}