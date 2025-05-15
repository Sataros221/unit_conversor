const BIT: i32 = 1;
const BYTE: i32 = BIT * 8;
const KILOBYTE: i32 = BYTE * 1024;
const MEGABYTE: i32 = KILOBYTE * 1024;

pub enum ResultType {
    Integer(i32),
    Float(f32),
}

pub enum DataUnit {
    Bit,
    Byte,
    Kilobyte,
    Megabyte,
}

pub struct Data {
    pub unit: DataUnit,
    pub value: ResultType,
}

impl Data {
    pub fn convert_to(&self, target_unit: DataUnit) -> Data {
        let data_in_bytes = match self.unit {
            DataUnit::Bit => self.value,
            DataUnit::Byte => self.value * BYTE,
            DataUnit::Kilobyte => self.value * KILOBYTE,
            DataUnit::Megabyte => self.value * MEGABYTE,
        };

        return match target_unit {
            DataUnit::Bit => data_in_bytes,
            DataUnit::Byte => data_in_bytes / BYTE,
            DataUnit::Kilobyte => data_in_bytes / KILOBYTE,
            DataUnit::Megabyte => data_in_bytes / MEGABYTE,
        };
    }
}
