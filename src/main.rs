use std::{fmt::Display, println};

use steam_tables::{data, saturated_steam::SteamTable, water};

fn main() {
    temperature_table();
    pressure_table();
    water_table()
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

fn water_table() {
    println!("Properties of Water");
    let water_table =
        water::WaterTable::new(data::COMPRESSED_LIQUID_SUPERHEATED_STEAM.to_string()).unwrap();

    let properties = water_table.get_values_at_point(0.1, 50.0).unwrap();
    print_data(properties);

    let properties = water_table.get_values_at_point(1.1, 500.0).unwrap();
    print_data(properties);

    let properties = water_table.get_values_at_point(2.0, 50.0).unwrap();
    print_data(properties);

    let properties = water_table.get_values_at_point(0.02, 500.0).unwrap();
    print_data(properties);
}
fn print_data<T, K>(data: Vec<(T, K)>)
where
    T: Display,
    K: Display,
{
    println!();
    for value in data {
        println!("{}: {}", value.0, value.1);
    }
    println!();
}
