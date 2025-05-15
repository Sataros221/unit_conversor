use units::data::{
    Data,
    DataUnit::{Bit, Byte, Kilobyte},
};

mod units {
    pub mod data;
}

fn main() {
    println!("Hello, world!");

    let data_ejemplardo = Data {
        value: 10.0,
        unit: Kilobyte,
    };
    let converted = data_ejemplardo.convert_to(Byte);

    println!(
        "El valor de {} bits es igual a {} kilobytes",
        data_ejemplardo.value, converted
    );
}
