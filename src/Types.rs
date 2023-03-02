use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Type {
    name: String,
    framework: String,
    component: String,
    scss_path: String,
    js_path: String,
}

impl Type {
    pub fn new(name: &str, framework: &str, component: &str) -> Self {
        Type {
            name: String::from(name),
            framework: String::from(framework),
            component: String::from(component),
            scss_path: String::from(""),
            js_path: String::from(""),
        }
    }

    pub fn set_path(&mut self) {
        match self.framework.as_str() {
            "react" | "r" | "React" => {
                println!("{}", self.framework);
                let compdir = Path::new("./Components");
                let stylesdir = Path::new("./Styles");
                if !Path::exists(compdir) {
                    match fs::create_dir("./Components") {
                        Err(e) => println!("{}", e),
                        Ok(()) => println!("Created folder succesfully"),
                    }
                }
                if !Path::exists(stylesdir) {
                    fs::create_dir(stylesdir);
                }
                let mut f = File::options()
                    .write(true)
                    .create(true)
                    .open(format!("./Components/{}.js", self.name))
                    .unwrap();

                let react_comp = format!(
                    "import React from 'react';

const {name} = () => {{
  return (
    <div>{name}</div>
  )
}}

export default {name}",
                    name = self.name
                );
                f.write(react_comp.as_bytes());
                let styles = File::options()
                    .create(true)
                    .write(true)
                    .open(format!("./Styles/{}.css", self.name))
                    .unwrap();
            }
            "next" | "n" | "nextjs" => {
                println!("{}", self.framework)
            }
            _ => panic!("Wrong framwork "),
        }
    }
}
