use crate::utils::vec_to_string::convert_to_vec_string;
use serde::Deserialize;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Forwarding {
    from: String,
    to: String,
    #[serde(rename = "doReverse")]
    do_reverse: bool
}

impl Forwarding {
    pub fn new(from: String, to: String, do_reverse: bool) -> Forwarding {
        Forwarding{
            from,
            to,
            do_reverse
        }
    }

    pub fn from_json(json: &Value) -> Forwarding {
        let object_forwarding = json.as_object()
            .expect("Error parsing forwarding");

        let from = object_forwarding.get("from")
            .unwrap()
            .as_str()
            .expect("Error parsing from field");

        let to = object_forwarding.get("to")
            .unwrap()
            .as_str()
            .expect("Error parsing to field");

        let do_reverse = object_forwarding.get("doReverse")
            .unwrap()
            .as_bool()
            .expect("Error parsing doReverse field");

        Forwarding::new(from.to_string(), to.to_string(), do_reverse)
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(rename = "serverAddr")]
    server_addr: String,
    blacklist: Vec<String>,
    forwarding: Vec<Forwarding>
}

impl Config {
    pub fn new(server_addr: String, blacklist: Vec<String>, forwarding: Vec<Forwarding>) -> Config {
        Config {
            server_addr,
            blacklist,
            forwarding
        }
    }

    fn read_config_file(file_name: String) -> String {
        let mut file = File::open(file_name).expect("Error opening config.json file. Go to the readme file and paste the example.");
        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Error reading config file content.");
        buf
    }

    fn parse_buffer(buf: &String) -> serde_json::Map<String, Value> {
        let json: Value = serde_json::from_str(&buf)
            .expect("Error parsing config file");

        let object_data = json.as_object()
            .expect("Error parsing config file.");

        object_data.clone()
    }

    fn parse_forwarding(str_forwarding: &Vec<Value>) -> Vec<Forwarding> {
        let mut vec_forwarding = Vec::new();

        for json_forwarding in str_forwarding {
            vec_forwarding.push(Forwarding::from_json(json_forwarding));
        }

        vec_forwarding
    }

    pub fn from_json(file_name: String) -> Config {
        let buf = Config::read_config_file(file_name);
        let object_data = Self::parse_buffer(&buf);


        let server_addr = object_data.get("serverAddr")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();

        let blacklist = object_data.get("blacklist")
            .unwrap()
            .as_array()
            .unwrap();
        let str_blacklist = convert_to_vec_string(blacklist);

        let raw_forwarding = object_data.get("forwarding")
            .unwrap()
            .as_array()
            .expect("Error parsing forwarding.");
        let forwardings = Self::parse_forwarding(raw_forwarding);

        Config::new(server_addr, str_blacklist, forwardings)
    }

    pub fn addr_is_blacklisted(&self, addr: &String) -> bool {
        self.blacklist.contains(addr)
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

    pub fn get_server_addr(&self) -> String {
        self.server_addr.to_string()
    }
}
