use std::io::{stdin, stdout, Write};

use clearscreen::clear;
use regex::Regex;

pub fn start(data: String) {
    clear().unwrap();
    let cds = data.split("\n");
    let mut curfn = String::new();
    let mut fns: Vec<Func> = Vec::new();
    let mut vrs: Vec<Var> = Vec::new();
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
        if f.name == "main" {
            for i in vrs.clone() {
                println!("line : 49");
                i.display();
            }
            f.clone().run(f.code, &mut fns, &mut vrs);
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
    fn run(&self, code: Vec<String>, fns: &mut Vec<Func>, vrs: &mut Vec<Var>);
}
#[allow(unused_variables)]
#[allow(unused_assignments)]

impl Run for Func {
    fn run(&self, code: Vec<String>, fns: &mut Vec<Func>, vrs: &mut Vec<Var>) {
        for cd in code {
            let cd = cd.trim();
            if cd.starts_with("echoln") {
                let elnrg = Regex::new(r#"echoln\((.*?)\)\;"#).unwrap();
                if let Some(cap) = elnrg.captures(cd) {
                    let txts = cap.get(1).unwrap().as_str();
                    let txts = txts.split(",");
                    let mut top = String::new();
                    for txt in txts {
                        if txt.starts_with("\"") {
                            top.push_str(txt.trim_start_matches("\"").trim_end_matches("\""));
                        } else {
                            //println!("checking for vars near 94");
                            for vr in vrs.clone() {
                                //println!("cur var : {:?}",vr.clone());
                                if txt == vr.name {
                                    top.push_str(
                                        &vr.val.trim_start_matches("\"").trim_end_matches("\""),
                                    );
                                }
                            }
                        }
                    }
                    println!("{top}");
                }
            } else if cd.starts_with("echo") {
                let elnrg = Regex::new(r#"echo\((.*?)\)\;"#).unwrap();
                if let Some(cap) = elnrg.captures(cd) {
                    let txts = cap.get(1).unwrap().as_str();
                    let txts = txts.split(",");
                    let mut top = String::new();
                    for txt in txts {
                        if txt.starts_with("\"") {
                            top.push_str(txt.trim_start_matches("\"").trim_end_matches("\""));
                        } else {
                            for vr in vrs.clone() {
                                if txt == vr.name {
                                    top.push_str(
                                        &vr.val.trim_start_matches("\"").trim_end_matches("\""),
                                    );
                                }
                            }
                        }
                    }
                    print!("{top}");
                }
            } else if cd == "out.flush();" {
                stdout().flush().unwrap();
            } else if cd.starts_with("may") {
                let varrg = Regex::new(r#"may\s+(\w+)\s*=\s*(.+)\s*;"#).unwrap();
                if let Some(cap) = varrg.captures(cd) {
                    let vrnm = cap.get(1).unwrap().as_str();
                    let vrval = cap.get(2).unwrap().as_str();
                    let mut vrt = VType::F;
                    if vrval.parse::<i128>().is_ok() {
                        vrt = VType::I;
                    } else {
                        if vrval.parse::<f64>().is_ok() {
                            vrt = VType::F;
                        } else {
                            vrt = VType::S;
                        }
                    }
                    let vr = Var {
                        name: vrnm.to_string(),
                        val: vrval.to_string(),
                        vtype: vrt,
                    };
                    vrs.push(vr);
                    // for i in vrs.clone() {
                    //     //println!("line : 147");
                    //     i.display();
                    // }
                }
            } else if cd.starts_with("takein") {
                let tkinrg = Regex::new(r#"takein\((.*?)\);"#).unwrap();
                if let Some(cap) = tkinrg.captures(cd) {
                    let vrnm = cap.get(1).unwrap().as_str();
                    for i in vrs.iter_mut() {
                        if vrnm == i.name {
                            let mut vl = String::new();
                            stdin().read_line(&mut vl).unwrap();
                            i.val = vl.trim().to_string();
                        }
                    }
                }
            } else if cd.starts_with("add") {
                let adrg = Regex::new(r#"add\((.*?)\);"#).unwrap();
                if let Some(cap) = adrg.captures(cd) {
                    let expr = cap.get(1).unwrap().as_str();
                    let mut varndexpr = expr.split(":");
                    let mut ix = 0;
                    let mut fv = "0".to_string();
                    let mut tsvr = Var::new("name".to_string(), "".to_string(), VType::S);
                    while let Some(i) = varndexpr.next() {
                        if ix == 0 {
                            let tgvrnm = i;
                            for vr in vrs.clone() {
                                if tgvrnm == vr.name {
                                    tsvr = vr;
                                }
                            }
                            ix += 1;
                        } else {
                            let vls = i.split(",");
                            for vl in vls {
                                if vl.parse::<i128>().is_ok() {
                                    let vl_i128 = vl.parse::<i128>().unwrap_or(0);
                                    if !fv.is_empty() {
                                        if fv.parse::<i128>().is_ok() {
                                            fv = (fv.parse::<i128>().unwrap_or(0) + vl_i128)
                                                .to_string();
                                        } else if fv.parse::<f64>().is_ok() {
                                            fv = (fv.parse::<f64>().unwrap_or(0.0)
                                                + vl_i128 as f64)
                                                .to_string();
                                        } else {
                                            fv = format!("{}{}", fv, vl_i128);
                                        }
                                    } else {
                                        fv = vl_i128.to_string();
                                    }
                                } else if vl.parse::<f64>().is_ok() {
                                    let vl_f64 = vl.parse::<f64>().unwrap_or(0.0);
                                    if !fv.is_empty() {
                                        if fv.parse::<f64>().is_ok() {
                                            fv = (fv.parse::<f64>().unwrap_or(0.0) + vl_f64)
                                                .to_string();
                                        } else if fv.parse::<i128>().is_ok() {
                                            fv = (fv.parse::<i128>().unwrap_or(0) as f64 + vl_f64)
                                                .to_string();
                                        } else {
                                            fv = format!("{}{}", fv, vl_f64);
                                        }
                                    } else {
                                        fv = vl_f64.to_string();
                                    }
                                } else {
                                    for i in vrs.clone() {
                                        if i.name == vl {
                                            let getval = i.val;
                                            if getval.parse::<i128>().is_ok() {
                                                fv = (fv.parse::<i128>().unwrap_or(0)
                                                    + getval.parse::<i128>().unwrap())
                                                .to_string();
                                            } else if getval.parse::<f64>().is_ok() {
                                                fv = (fv.parse::<f64>().unwrap_or(0.0)
                                                    + getval.parse::<i128>().unwrap() as f64)
                                                    .to_string();
                                            } else {
                                                fv = format!("{}{}", fv, getval);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tsvr.val = fv.clone();
                    for vr in vrs.iter_mut() {
                        if vr.name == tsvr.name {
                            vr.val = fv.clone();
                        }
                    }
                }
            } else if cd.starts_with("sub") {
                let sbrg = Regex::new(r#"sub\((.*?)\);"#).unwrap();
                if let Some(cap) = sbrg.captures(cd) {
                    let expr = cap.get(1).unwrap().as_str();
                    let mut varndexpr = expr.split(":");
                    let mut ix = 0;
                    let mut fv = "0".to_string();
                    let mut tsvr = Var::new("name".to_string(), "".to_string(), VType::S);
                    while let Some(i) = varndexpr.next() {
                        if ix == 0 {
                            let tgvrnm = i;
                            for vr in vrs.clone() {
                                if tgvrnm == vr.name {
                                    tsvr = vr;
                                }
                            }
                            ix += 1;
                        } else {
                            let vls = i.split(",");
                            for vl in vls {
                                if vl.parse::<i128>().is_ok() {
                                    let vl_i128 = vl.parse::<i128>().unwrap_or(0);
                                    if !fv.is_empty() {
                                        if fv.parse::<i128>().is_ok() {
                                            fv = (fv.parse::<i128>().unwrap_or(0) - vl_i128)
                                                .to_string();
                                            //println!("fv at 252 - {}", fv);
                                        } else if fv.parse::<f64>().is_ok() {
                                            fv = (fv.parse::<f64>().unwrap_or(0.0)
                                                - vl_i128 as f64)
                                                .to_string();
                                            //println!("fv at 256 - {}", fv);
                                        } else {
                                            fv = format!("{}{}", fv, vl_i128);
                                            //println!("fv at 260 - {}", fv);
                                        }
                                    } else {
                                        fv = vl_i128.to_string();
                                        //println!("fv at 265 - {}", fv);
                                    }
                                } else if vl.parse::<f64>().is_ok() {
                                    let vl_f64 = vl.parse::<f64>().unwrap_or(0.0);
                                    if !fv.is_empty() {
                                        if fv.parse::<f64>().is_ok() {
                                            fv = (fv.parse::<f64>().unwrap_or(0.0) - vl_f64)
                                                .to_string();
                                            //println!("fv at 265 - {}", fv);
                                        } else if fv.parse::<i128>().is_ok() {
                                            fv = (fv.parse::<i128>().unwrap_or(0) as f64 - vl_f64)
                                                .to_string();
                                            // println!("fv at 268 - {}", fv);
                                        } else {
                                            fv = format!("{}{}", fv, vl_f64);
                                            //println!("fv at 272 - {}", fv);
                                        }
                                    } else {
                                        fv = vl_f64.to_string();
                                    }
                                } else {
                                    for i in vrs.clone() {
                                        if i.name == vl {
                                            let getval = i.val;
                                            if getval.parse::<i128>().is_ok() {
                                                fv = (fv.parse::<i128>().unwrap_or(0)
                                                    + getval.parse::<i128>().unwrap())
                                                .to_string();
                                            } else if getval.parse::<f64>().is_ok() {
                                                fv = (fv.parse::<f64>().unwrap_or(0.0)
                                                    + getval.parse::<i128>().unwrap() as f64)
                                                    .to_string();
                                            } else {
                                                fv = format!("{}{}", fv, getval);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tsvr.val = fv.clone();
                    for vr in vrs.iter_mut() {
                        if vr.name == tsvr.name {
                            vr.val = fv.clone();
                        }
                    }
                }
            } else {
                for fs in fns.clone() {
                    let tocll = format!("{}();", fs.name);
                    if cd == tocll {
                        // for i in vrs.clone() {
                        //     //println!("line : 150");
                        //     i.display();
                        // }
                        fs.clone().run(fs.code, fns, vrs);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Var {
    name: String,
    val: String,
    vtype: VType,
}

impl Var {
    fn new(name: String, val: String, vtype: VType) -> Self {
        Self { name, val, vtype }
    }
}

trait VarT {
    fn display(&self);
}

impl VarT for Var {
    fn display(&self) {
        println!(
            "name - {} , val - {} , vtype - {:?}",
            self.name, self.val, self.vtype
        );
    }
}
#[derive(Debug, Clone)]
enum VType {
    I,
    F,
    S,
}
