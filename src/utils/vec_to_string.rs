use serde_json::Value;

pub fn convert_to_vec_string(vec: &Vec<Value>) -> Vec<String> {
    let mut str_vec = Vec::new();
    for value in vec {
        str_vec.push(value.as_str().unwrap().to_string());
    }
    str_vec
}
