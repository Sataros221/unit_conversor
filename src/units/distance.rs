const NANOMETER: f64 = METER / 1_000_000_000.0;
const MICROMETER: f64 = METER / 1_000_000.0;
const MILLIMETER: f64 = METER / 1000.0;
const CENTIMETER: f64 = METER / 100.0;
const METER: f64 = 1.0;
const KILOMETER: f64 = METER * 1000.0;

#[derive(Debug)]
pub enum DistanceUnit {
    Nanometer,
    Micrometer,
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

#[derive(Debug)]
pub struct Distance {
    pub unit: DistanceUnit,
    pub value: f64,
}

impl Distance {
    pub fn convert_to(&self, target_unit: DistanceUnit) -> Distance {
        let distance_in_meters: f64 = match self.unit {
            DistanceUnit::Nanometer => self.value * NANOMETER,
            DistanceUnit::Micrometer => self.value * MICROMETER,
            DistanceUnit::Millimeter => self.value * MILLIMETER,
            DistanceUnit::Centimeter => self.value * CENTIMETER,
            DistanceUnit::Meter => self.value * METER,
            DistanceUnit::Kilometer => self.value * KILOMETER,
        };

        let res: f64 = match target_unit {
            DistanceUnit::Nanometer => distance_in_meters / NANOMETER,
            DistanceUnit::Micrometer => distance_in_meters / MICROMETER,
            DistanceUnit::Millimeter => distance_in_meters / MILLIMETER,
            DistanceUnit::Centimeter => distance_in_meters / CENTIMETER,
            DistanceUnit::Meter => distance_in_meters / METER,
            DistanceUnit::Kilometer => distance_in_meters / KILOMETER,
        };

        return Distance {
            unit: target_unit,
            value: res,
        };
    }
}
