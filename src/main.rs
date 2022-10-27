use std::fs::{self, File};
use std::io::{copy, Read, Write};
use std::process::Command;
use std::str;

use regex::Regex;
use reqwest;
use zip;
fn main() {
    let inst = Command::new("pacman")
        .arg("-Q")
        .output()
        .expect("Unable to execute pacman on this system");

    let inst = str::from_utf8(inst.stdout.as_slice()).expect("Unable to parse stdout output");

    let re = Regex::new(r"google-chrome (?P<major>\d{2,3}).\d{1,2}.\d{4}.\d{1,3}").unwrap();
    let version = re.captures(inst).unwrap();
    let major = &version["major"];

    let sent = format!(
        r"If you are using Chrome version {}, please download ChromeDriver (?P<driver>\d{{2,3}}.\d{{1,2}}.\d{{4}}.\d{{1,3}})",
        major
    );

    let res = reqwest::blocking::get("https://chromedriver.chromium.org/downloads").expect("Hello");
    let body = res.text().unwrap();
    let re = Regex::new(sent.as_str()).unwrap();
    let version = re.captures(body.as_str()).unwrap();
    let uri = format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        &version["driver"]
    );

    println!("Downloading chromedriver ...");
    let mut res = reqwest::blocking::get(uri.as_str()).expect("Unable to download");
    let mut zipfile = File::create(
        format!(
            "/home/parfaitd/Logs/chromedriver_{}.zip",
            &version["driver"]
        )
        .as_str(),
    )
    .expect("Unable to create file");
    copy(&mut res, &mut zipfile);
    let mut archive = zip::ZipArchive::new(zipfile).unwrap();

    let mut file = match archive.by_name("chromedriver") {
        Ok(file) => file,
        Err(..) => {
            println!("File test/lorem_ipsum.txt not found");
            return;
        }
    };

    let mut driver =
        File::create(format!("/home/parfaitd/Logs/chromedriver_{}", &version["driver"]).as_str())
            .expect("Hello");

    copy(&mut file, &mut driver).expect("Error copying driver from .zip file");

    println!("File downloaded");
}
