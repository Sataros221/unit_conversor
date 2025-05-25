use units::data::{self, Data, DataUnit};

mod units {
    pub mod data;
}

fn print_conversion<T>(data: &T, converted: &T)
where
    T: std::fmt::Debug,
{
    let message = format!("{:?} -> {:?}", data, converted);
    let fill = "*".repeat(message.len() + 4);
    println!("{}", fill);
    println!("* {} *", message);
    println!("{}", fill);
}

fn read_line(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", prompt);

    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().to_string();
}

fn main() {
    println!("-----* Welcome to the Data Unit Converter! *-----");

    loop {
        println!("Please enter the value to convert (or type 'exit' to quit):");
        println!("Select the type of unit to convert:");
        println!("1. Data 💽");
        println!("2. Temperature 🌡️");
        println!("3. Distance 📏");
        println!("Enter the number of your choice:");

        // TYPE CHOICE DEFINITION
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();

        // CHOICE HANDLING
        match choice {
            "1" => {
                println!(
                    "Available units:\n* bit,\n* nibble,\n* byte,\n* kilobyte,\n* megabyte,\n* terabyte"
                );

                let from_unit: String = read_line("From unit: ");
                let from_unit: DataUnit = data::translate_unit(&from_unit);

                let to_unit: String = read_line("To unit: ");
                let to_unit: DataUnit = data::translate_unit(&to_unit);

                let value: String = read_line("Enter the value to convert: ");
                let value: f32 = match value.parse::<f32>() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Invalid value. Should be a number (f32).");
                        continue;
                    }
                };

                let data = Data {
                    value,
                    unit: to_unit,
                };

                let converted: Data = data.convert_to(from_unit);

                print_conversion(&data.value, &converted.value);
            }
            "2" => {
                // Temperature conversion logic (not implemented yet)
                println!("Temperature conversion is not implemented yet :(");
            }
            "3" => {
                // Distance conversion logic (not implemented yet)
                println!("Distance conversion is not implemented yet :(");
            }
            "exit" => {
                println!("Exiting the converter. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}
