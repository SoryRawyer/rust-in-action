// example API usage for libactionkv

use std::path::PathBuf;

use libactionkv::ActionKV;
use structopt::StructOpt;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

#[derive(Debug, StructOpt)]
struct Opt {
    /// input database file
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// get, delete, insert, or update
    action: String,

    key: String,

    value: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("hi: {:?}", opt);

    let path = std::path::Path::new(&opt.path);
    let mut store = ActionKV::open(&path).expect("unable to open file");
    store.load().expect("unable to load data");

    let key = opt.key.as_bytes();

    match opt.action.as_str() {
        "get" => match store.get(key).unwrap() {
            None => eprintln!("{:?} not found", &opt.key),
            Some(value) => println!("{:?}", value),
        },
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = opt.value.expect(&USAGE);
            store.insert(key, value.as_ref()).unwrap()
        }
        "update" => {
            let value = opt.value.expect(&USAGE);
            store.update(key, value.as_ref()).unwrap()
        }
        _ => panic!("omgomgomgomg"),
    }
}
