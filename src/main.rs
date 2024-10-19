use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::fs::File;
use std::env;

fn main() {

    let home_dir = env::home_dir().expect("err: <$HOME_DIR> undefined").display().to_string();
    let mut git_password = String::new();

    println!("{home_dir}");

    let file = File::open([home_dir.as_str(), "/.fhub"].join("")).expect("err: cant find file [.fhub] with PAT key");
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_line(&mut git_password).expect("err: cant read PAT key");

    let binding = env::current_dir().unwrap();
    let curr_dir = binding.to_str().unwrap();


    let git_pass_file_path = [home_dir.as_str(), "/.git_password.sh"].join("");

    if !Path::new(&git_pass_file_path).exists() {
        let mut pass_file = File::options().mode(0o777).create_new(true).write(true).open(&git_pass_file_path).expect(["err: cant create file in ", &git_pass_file_path].join("").as_str());
        pass_file.write_all("#!/bin/sh\n exec echo \"$GIT_PASSWORD\"".as_bytes()).expect("err: cant read PAT key");
    }

    Command::new("git").current_dir(curr_dir)
        .arg("add").arg(".")
        .spawn().expect("err: cant spawn [$ git add . ]")
        .wait().expect("err: while running [$ git add . ]");

    Command::new("git").current_dir(curr_dir)
        .arg("commit").arg("-m").arg("\"fhub commit\"")
        .spawn().expect("err: cant spawn [$ commit -m \"fasthub commit\"]")
        .wait().expect("err: while running [$ commit -m \"fasthub commit\"]");

    Command::new("git").current_dir(curr_dir).stdin(Stdio::piped())
        .arg("push").arg("origin").arg("main")
        .env("GIT_PASSWORD", git_password)
        .env("GIT_ASKPASS", git_pass_file_path)
        .spawn().expect("err: cant spawn [$ git push origin main")
        .wait().expect("err: cant spawn [$ git push origin main]");

    println!("push success");
}
