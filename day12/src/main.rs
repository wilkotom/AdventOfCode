use std::fs::read_to_string;
use serde_json::Value;


fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let parsed: Value = serde_json::from_str(&data).unwrap();
    println!("{:?}", parse(&parsed));
}

fn parse(value: &Value) -> i64 {

    match value {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => {n.as_i64().unwrap()},
        Value::String(_) => 0,
        Value::Array(a) => {a.iter().map(|x| parse(x)).sum()}
        Value::Object(o) => {
            if o.values().filter(|v| **v == Value::String("red".to_owned())).count()  == 0 {
                o.values().map(|x| parse(x)).sum()
            } else {
                0
            }
        },
    }

}