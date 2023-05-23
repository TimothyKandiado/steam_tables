use std::println;

use steam_tables::{data, saturated_steam::SteamTable};

fn main() {
    temperature_table();
    pressure_table();
}

fn pressure_table() {
    let steam_table = SteamTable::new(data::SATURATED_BY_PRESSURE_TABLE.to_string()).unwrap();

    let values = steam_table.get_values_at_point(1.0).unwrap();

    print_data(values);

    let values = steam_table.get_values_at_point(2.0).unwrap();

    print_data(values);

    let values = steam_table.get_values_at_point(5.0).unwrap();

    print_data(values);
}

fn temperature_table() {
    let steam_table = SteamTable::new(data::SATURATED_BY_TEMPERATURE_TABLE.to_string()).unwrap();

    let values = steam_table.get_values_at_point(0.01).unwrap();

    print_data(values);

    let values = steam_table.get_values_at_point(100.0).unwrap();

    print_data(values);

    let values = steam_table.get_values_at_point(373.946).unwrap();

    print_data(values);
}

fn print_data(data: Vec<(String, f32)>) {
    println!();
    for value in data {
        println!("{}: {}", value.0, value.1);
    }
    println!();
}
