pub enum DataUnit {
    Bit,
    Byte,
    Kilobyte,
    Megabyte,
}

pub struct DataValue {
    pub value: i32,
    pub unit: DataUnit,
}

impl DataValue {
    const BYTE: i32 = 8;

    pub fn convert_to(&self, unit: DataUnit) -> i32 {
        match unit {
            DataUnit::Bit => self.value,
            DataUnit::Byte => self.value * Self::BYTE,
            DataUnit::Kilobyte => self.value * Self::BYTE * 1024,
            DataUnit::Megabyte => self.value * Self::BYTE * 1024 * 1024,
        }
    }
}
