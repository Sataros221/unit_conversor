const BIT: f32 = 1.0;
const BYTE: f32 = BIT * 8.0;
const KILOBYTE: f32 = BYTE * 1024.0;
const MEGABYTE: f32 = KILOBYTE * 1024.0;

pub enum DataUnit {
    Bit,
    Byte,
    Kilobyte,
    Megabyte,
}

pub struct Data {
    pub unit: DataUnit,
    pub value: f32,
}

impl Data {
    pub fn convert_to(&self, target_unit: DataUnit) -> Data {
        let data_in_bits: f32 = match self.unit {
            DataUnit::Bit => self.value,
            DataUnit::Byte => self.value * BYTE,
            DataUnit::Kilobyte => self.value * KILOBYTE,
            DataUnit::Megabyte => self.value * MEGABYTE,
        };

        let res: f32 = match target_unit {
            DataUnit::Bit => data_in_bits,
            DataUnit::Byte => data_in_bits / BYTE,
            DataUnit::Kilobyte => data_in_bits / KILOBYTE,
            DataUnit::Megabyte => data_in_bits / MEGABYTE,
        };

        return Data {
            unit: target_unit,
            value: res,
        };
    }
}
