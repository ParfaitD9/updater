use std::fs::{self, File};
use std::io::copy;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use regex::Regex;
use reqwest;
use structopt::StructOpt;
use zip;
fn main() {
    let args = Arguments::from_args();

    let inst = Command::new("pacman")
        .arg("-Q")
        .output()
        .expect("Unable to execute pacman on this system");

    let inst = str::from_utf8(inst.stdout.as_slice()).expect("Unable to parse stdout output");

    let re = Regex::new(r"google-chrome (?P<major>\d{2,3}).\d{1,2}.\d{4}.\d{1,3}").unwrap();
    let version = re.captures(inst).unwrap();
    let major = &version["major"];

    let sent = match &args.version {
        Some(v) => format!(
            r"If you are using Chrome version {}, please download ChromeDriver (?P<driver>\d{{2,3}}.\d{{1,2}}.\d{{4}}.\d{{1,3}})",
            v
        ),
        None => format!(
            r"If you are using Chrome version {}, please download ChromeDriver (?P<driver>\d{{2,3}}.\d{{1,2}}.\d{{4}}.\d{{1,3}})",
            major
        ),
    };

    let res = reqwest::blocking::get("https://chromedriver.chromium.org/downloads").expect("Hello");
    let body = res.text().unwrap();
    let re = Regex::new(sent.as_str()).unwrap();
    let version = re.captures(body.as_str()).unwrap();

    let uri = format!(
        "https://chromedriver.storage.googleapis.com/{}/chromedriver_linux64.zip",
        &version["driver"]
    );

    println!("Downloading chromedriver v{} ...", &version["driver"]);
    let mut res = reqwest::blocking::get(uri.as_str()).expect("Unable to download");

    let mut zipfile = File::create(&args.output.join("tmp.zip")).expect("Unable to create file");
    copy(&mut res, &mut zipfile).expect("Unable to save downloaded file");

    println!("Zip file downloaded !");
    let zipfile: File =
        File::open(&args.output.join("tmp.zip")).expect("Unable to open downloaded zip");
    let mut archive = zip::ZipArchive::new(zipfile).expect("Unable to open zip");

    let mut file = match archive.by_name("chromedriver") {
        Ok(file) => file,
        Err(..) => {
            println!("File chromedriver not found");
            return;
        }
    };

    let mut driver = {
        let fname = match &args.fname {
            Some(s) => s.to_owned(),
            None => String::from(format!("chromedriver-{}", &version["driver"])),
        };
        File::create(&args.output.join(fname)).expect("Hello")
    };
    println!("Extracting driver from .zip file");
    copy(&mut file, &mut driver).expect("Error copying driver from .zip file");
    fs::remove_file(&args.output.join("tmp.zip")).expect("Unable to delete downloaded zip");
    println!("Operation completed :)");
}

#[derive(StructOpt, Debug)]
#[structopt(name = "UPDATER")]
struct Arguments {
    /// Output dir
    #[structopt(short = "o", long, parse(from_os_str))]
    output: PathBuf,

    /// Version to download default Chrome installed version
    #[structopt(short = "v", long)]
    version: Option<u32>,

    #[structopt(short = "f", long)]
    fname: Option<String>,
}
