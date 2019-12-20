#[macro_use]
extern crate clap;
use clap::{App,};
use std::io::{self, Write};

fn main() { 
    let yaml = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yaml).get_matches(); 
    let output = matches.value_of("out");
    let input = matches.value_of("in");
    let mut out_ : std::process::Command;
    let mut in_: std::process::Command;
    
    let c_arg : &str;
    if cfg!(target_os = "windows"){
        in_= std::process::Command::new("cmd");
        out_ = std::process::Command::new("cmd");
        c_arg = "/C";
    } else{
        in_ = std::process::Command::new("sh");
        out_ = std::process::Command::new("sh");
        c_arg = "-c";
    }
    let res_in = in_
        .arg(c_arg)
        .arg(input.unwrap())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("error");

        let stdin = Some(res_in).map_or(
            std::process::Stdio::inherit(),
            |output: std::process::Child| std::process::Stdio::from(output.stdout.unwrap())
        );
    let out_res = out_
        .arg(c_arg)
        .arg(output.unwrap())
        .stdin(std::process::Stdio::from(stdin))
        .output()
        .expect("error");

    io::stdout().write_all(&out_res.stdout).unwrap();
}
