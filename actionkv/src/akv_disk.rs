/// actionkv with index on disk
use libactionkv::ActionKV;
use std::collections::HashMap;
use std::path::PathBuf;

use structopt::StructOpt;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    action: String,
    key: String,
    value: Option<String>,
}

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    println!("hi");

    const INDEX_KEY: &ByteStr = b"+index";

    let opt = Opt::from_args();
    let path = std::path::Path::new(&opt.path);
    let mut akv = ActionKV::open(path).expect("uh oh!");
    akv.load().expect("ahhhh!!!");

    let key = opt.key.as_bytes();

    match opt.action.as_str() {
        "get" => {
            let index_as_bytes = akv.get(&INDEX_KEY).unwrap().unwrap();
            let index: HashMap<ByteString, u64> = bincode::deserialize(&index_as_bytes).unwrap();
            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = akv.get_at(i).unwrap();
                    println!("{:?}", kv.value);
                }
            }
        }
        "delete" => akv.delete(key).unwrap(),
        "insert" => {
            let value = opt.value.expect(&USAGE);
            akv.insert(key, value.as_ref()).unwrap();
            // reuse our key-value store to keep a copy of our index
            store_index_on_disk(&mut akv, &INDEX_KEY);
        }
        "update" => {
            let value = opt.value.expect(&USAGE);
            akv.update(key, value.as_ref()).unwrap();
            store_index_on_disk(&mut akv, &INDEX_KEY);
        }
        _ => panic!("omgomgomg"),
    }
}
