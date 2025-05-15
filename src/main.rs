use units::data::{DataUnit, DataValue};

mod units {
    pub mod data;
}

fn main() {
    println!("Hello, world!");

    let data_ejemplardo = DataValue {
        value: 10,
        unit: DataUnit::Byte,
    };
    let converted = data_ejemplardo.convert_to(DataUnit::Bit);

    println!(
        "El valor de {} bytes es igual a {} bits",
        data_ejemplardo.value, converted
    );
}
