use std::io::{stdout, Write};

use clearscreen::clear;
use regex::Regex;

pub fn start(data: String) {
    clear().unwrap();
    let cds = data.split("\n");
    let mut curfn = String::new();
    let mut fns: Vec<Func> = Vec::new();

    for cd in cds {
        //println!("cd : {}", cd);
        if cd.starts_with("ON") && !cd.ends_with("}") {
            let frg = Regex::new(r#"ON\s+(\w+)\(\)"#).unwrap();
            if let Some(cap) = frg.captures(cd) {
                let fnname = cap.get(1).unwrap().as_str().to_string();
                let fnc = Func {
                    name: fnname.clone(),
                    code: Vec::new(),
                };
                fns.push(fnc);
                curfn = fnname;
            }
        } else if cd == "}" {
            curfn = "".to_string();
        } else if cd.starts_with("ON") && cd.ends_with("}") {
            let frg = Regex::new(r#"ON\s+(\w+)\(\)"#).unwrap();
            if let Some(cap) = frg.captures(cd) {
                let fnname = cap.get(1).unwrap().as_str().to_string();
                let fnc = Func::new(fnname, Vec::new());
                fns.push(fnc);
                curfn = "".to_string();
            }
        } else {
            if !curfn.is_empty() {
                for f in &mut fns {
                    if curfn == f.name {
                        f.code.push(cd.to_string());
                    }
                }
            }
        }
    }

    for f in fns.clone() {
        if f.name == "main"{
            f.clone().run(f.code, &mut fns);
        }
    }
}
#[allow(dead_code)]

#[derive(Debug, Clone)]
struct Func {
    name: String,
    code: Vec<String>,
}
#[allow(dead_code)]

impl Func {
    fn new(name: String, code: Vec<String>) -> Self {
        Self { name, code }
    }

    fn dis(&self) {
        println!("name : {} , code : {:?}", self.name, self.code);
    }
}

trait Run {
    fn run(&self , code: Vec<String>, fns: &mut Vec<Func>);
}
#[allow(unused_variables)]

impl Run for Func {
    fn run(&self , code: Vec<String>, fns: &mut Vec<Func>) {
        for cd in code{
            let cd = cd.trim();
            if cd.starts_with("echoln"){
                let elnrg = Regex::new(r#"echoln\((.*?)\)\;"#).unwrap();
                if let Some(cap) = elnrg.captures(cd){
                    let txts = cap.get(1).unwrap().as_str();
                    let txts = txts.split(",");
                    let mut top = String::new();
                    for txt in txts{
                        if txt.starts_with("\""){
                            top.push_str(txt.trim_start_matches("\"").trim_end_matches("\""));
                        }
                        else{
                            top.push_str("VAR_DEBUG -  ");
                            top.push_str(txt);
                            top.push_str(" -");

                        }
                    }
                    println!("{top}");
                }
            }
            else if cd.starts_with("echo"){
                let elnrg = Regex::new(r#"echo\((.*?)\)\;"#).unwrap();
                if let Some(cap) = elnrg.captures(cd){
                    let txts = cap.get(1).unwrap().as_str();
                    let txts = txts.split(",");
                    let mut top = String::new();
                    for txt in txts{
                        if txt.starts_with("\""){
                            top.push_str(txt.trim_start_matches("\"").trim_end_matches("\""));
                        }
                        else{
                            top.push_str("VAR_DEBUG -  ");
                            top.push_str(txt);
                            top.push_str(" -");

                        }
                    }
                    print!("{top}");
                }
            }
            else if cd == "out.flush();"{
                stdout().flush().unwrap();
            }
            else{
                for fs in fns.clone(){
                    let tocll = format!("{}();",fs.name);
                    if cd == tocll{
                        fs.clone().run(fs.code, fns);
                    }
                }
            }
        }
    }
}

struct Var {
    name: String,
    val: String,
    vtype: VType,
}

enum VType {
    I,
    F,
    S,
}
