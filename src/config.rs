use std::io::prelude::*;

#[derive(Debug, Deserialize)]
enum Action {
    Clear,
    Drop,
    Replace
}

#[derive(Debug, Deserialize)]
enum Method {}

#[derive(Debug, Deserialize)]
enum Operator {
    GreaterThan,
    GreaterThanEqualTo,
    Equal,
    LessThan,
    LessThanEqualTo,
}

#[derive(Debug, Deserialize)]
enum Operand {
    Date,
    Number(u32)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    databases: Option<Vec<Database>>
}

#[derive(Debug, Deserialize)]
struct Database {
    name: String,
    comments: Option<bool>,
    action: Option<String>,
    tables: Option<Vec<Table>>
}

#[derive(Debug, Deserialize)]
struct Table {
    name: String,
    action: Option<String>,
    columns: Option<Vec<Column>>,
    rows: Option<Vec<Row>>
}

#[derive(Debug, Deserialize)]
struct Column {
    name: String,
    action: String,
    operator: Option<String>,
    operand: Option<String>,
    method: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Row {
    action: String,
    operator: Option<String>,
    operand: Option<String>,
}

pub fn open(path: String) -> Result<Config, &'static str> {
    let mut f = match ::File::open(path) {
        Ok(file) => file,
        Err(_) => return Err("Error opening config file")
    };

    let mut toml_str = String::new();

    if f.read_to_string(&mut toml_str).is_err() {
        return Err("Error reading config file");
    }

    let c = ::toml::from_str(toml_str.as_str());

    match c {
        Ok(v) => {
            validate(&v);
            Ok(v)
        },
        Err(_) => Err("Error reading config")
    }
}

fn validate(conf: &Config) {
}
