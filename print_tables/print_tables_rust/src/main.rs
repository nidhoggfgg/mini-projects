use print_tables_rust::tables::Table;

fn main() {
    let header: Vec<String> = vec!["name", "age", "salary"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    let mut data = Vec::new();
    let line1: Vec<String> = vec!["nidhoggfgg", "22", "100000"]
        .iter()
        .map(|a| a.to_string())
        .collect();
    data.push(line1);

    let mut some = Table::new(true);
    some.set_header(header);
    some.set_rows(data);
    let result = some.make_table();

    for i in result {
        println!("{}", i);
    }
}
