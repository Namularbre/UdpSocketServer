use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Forwarding {
    from: String,
    to: String,
    #[serde(rename = "doReverse")]
    do_reverse: bool
}

impl Forwarding {
    pub fn new(from: String, to: String, do_reverse: bool) -> Forwarding {
        return Forwarding{
            from,
            to,
            do_reverse
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    blacklist: Vec<String>,
    forwarding: Vec<Forwarding>
}

impl Config {
    pub fn new(blacklist: Vec<String>, forwarding: Vec<Forwarding>) -> Config {
        return Config {
            blacklist,
            forwarding
        }
    }

    pub fn from_json(file_name: String) -> Config {
        let mut file = File::open(file_name).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Error reading config file content.");
        let json: Value = serde_json::from_str(&buf).expect("Error parsing config file");
        let object_data = json.as_object().unwrap();
        let blacklist = object_data.get("blacklist").unwrap().as_array().unwrap();
        let forwarding = object_data.get("forwarding").unwrap().as_array().unwrap();
        let str_blacklist = Self::convert_to_vec_string(blacklist);
        let mut vec_forwarding = Vec::new();
        for json_forwarding in forwarding {
            let object_forwarding = json_forwarding.as_object().unwrap();
            let from = object_forwarding.get("from").unwrap().as_str().unwrap();
            let to = object_forwarding.get("to").unwrap().as_str().unwrap();
            let do_reverse = object_forwarding.get("doReverse").unwrap().as_bool().unwrap();
            let forwarding_struct = Forwarding::new(from.to_string(), to.to_string(), do_reverse);
            vec_forwarding.push(forwarding_struct);
        }
        Config::new(str_blacklist, vec_forwarding)
    }

    fn convert_to_vec_string(vec: &Vec<Value>) -> Vec<String> {
        let mut str_vec = Vec::new();
        for value in vec {
            str_vec.push(value.as_str().unwrap().to_string());
        }
        str_vec
    }

    pub fn addr_is_blacklisted(&self, addr: String) -> bool {
        self.blacklist.contains(&addr)
    }

    pub fn is_forwarded(&self, addr: String) -> Option<String> {
        for elem_forwarding in &self.forwarding {
            if elem_forwarding.from == addr {
                return Some(elem_forwarding.to.clone())
            } else if elem_forwarding.do_reverse && elem_forwarding.to == addr {
                return Some(elem_forwarding.from.clone())
            }
        }
        None
    }
}
