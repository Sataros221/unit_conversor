const BIT: f32 = 1.0;
const NIBBLE: f32 = BIT * 4.0;
const BYTE: f32 = BIT * 8.0;
const KILOBYTE: f32 = BYTE * 1024.0;
const MEGABYTE: f32 = KILOBYTE * 1024.0;
const TERABYTE: f32 = MEGABYTE * 1024.0;

#[derive(Debug)]
pub enum DataUnit {
    Bit,
    Nibble,
    Byte,
    Kilobyte,
    Megabyte,
    Terabyte,
}

#[derive(Debug)]
pub struct Data {
    pub unit: DataUnit,
    pub value: f32,
}

pub fn translate_unit(unit: &str) -> DataUnit {
    match unit.to_lowercase().as_str() {
        "bit" => DataUnit::Bit,
        "nibble" => DataUnit::Nibble,
        "byte" => DataUnit::Byte,
        "kilobyte" => DataUnit::Kilobyte,
        "megabyte" => DataUnit::Megabyte,
        "terabyte" => DataUnit::Terabyte,
        _ => panic!("Invalid data unit"),
    }
}

impl Data {
    pub fn convert_to(&self, target_unit: DataUnit) -> Data {
        let data_in_bits: f32 = match self.unit {
            DataUnit::Bit => self.value,
            DataUnit::Nibble => self.value * NIBBLE,
            DataUnit::Byte => self.value * BYTE,
            DataUnit::Kilobyte => self.value * KILOBYTE,
            DataUnit::Megabyte => self.value * MEGABYTE,
            DataUnit::Terabyte => self.value * TERABYTE,
        };

        let res: f32 = match target_unit {
            DataUnit::Bit => data_in_bits,
            DataUnit::Nibble => data_in_bits / NIBBLE,
            DataUnit::Byte => data_in_bits / BYTE,
            DataUnit::Kilobyte => data_in_bits / KILOBYTE,
            DataUnit::Megabyte => data_in_bits / MEGABYTE,
            DataUnit::Terabyte => data_in_bits / TERABYTE,
        };

        return Data {
            unit: target_unit,
            value: res,
        };
    }
}
