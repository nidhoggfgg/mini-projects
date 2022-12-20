use print_tables_rust::tables::Table;

fn main() {
    let header: Vec<String> = vec!["name", "age", "salary"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    let mut data = Vec::new();
    let line1: Vec<String> = vec!["nidhoggfgg", "22", "100000"]
        .iter()
        .map(ToString::to_string)
        .collect();
    let line2: Vec<String> = vec!["Some", "999", "9999"]
        .iter()
        .map(ToString::to_string)
        .collect();
    data.push(line1);
    data.push(line2);

    let mut some = Table::new(true);
    some.set_header(header);
    if let Err(err) = some.set_rows(data) {
        println!("{}", err);
    }
    let result = some.make_table();

    for i in result {
        println!("{}", i);
    }

    let col: Vec<String> = vec!["Chinese", "Fucking"]
        .iter()
        .map(ToString::to_string)
        .collect();

    if let Err(err) = some.push_col(col, "country".to_string()) {
        println!("{}", err);
    };

    let result = some.make_table();

    for i in result {
        println!("{}", i);
    }
}
