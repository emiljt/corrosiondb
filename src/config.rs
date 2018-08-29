enum Action {
    Clear,
    Drop,
    Replace
}

enum Method {}

enum Operator {}

enum Operand {}

enum Type {}

#[derive(Debug, Deserialize)]
struct Config {
    databases: Option<Vec<Database>>
}

#[derive(Debug, Deserialize)]
struct Database {
    name: Option<String>,
    comments: Option<bool>,
    tables: Option<Vec<Table>>
}

#[derive(Debug, Deserialize)]
struct Table {
    name: Option<String>,
    columns: Option<Vec<Column>>,
    rows: Option<Vec<Row>>
}

#[derive(Debug, Deserialize)]
struct Column {
    name: Option<String>
}

#[derive(Debug, Deserialize)]
struct Row {
}

pub fn print_config() {
    let toml_str = r#"
    [[databases]]
        name = "simplenexus"
        comments = false

        [[databases.tables]]
            name = "recent_jobs"
            action = "clear"

        [[databases.tables]]
            name = "users"

            [[databases.tables.columns]]
                name = "past_passwords"
                action = "clear"

        [[databases.tables]]
            name = "stats"

            [[simplenexus.tables.rows]]
                action = "drop"
                operator = ">"
                operand = "created_at"
                type = "date"
    "#;

    let decoded: Config = ::toml::from_str(toml_str).unwrap();
    println!("{:#?}", decoded);
}