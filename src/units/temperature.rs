const FAHRENHEIT_TO_CELSIUS: f64 = 5.0 / 9.0;
const CELSIUS_TO_FAHRENHEIT: f64 = 9.0 / 5.0;
const KELVIN_TO_CELSIUS: f64 = 273.15;

pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub struct Temperature {
    pub unit: TemperatureUnit,
    pub value: f64,
}

impl Temperature {
    pub fn convert_to(&self, target_unit: TemperatureUnit) -> Temperature {
        let value_in_celsius: f64 = match self.unit {
            TemperatureUnit::Celsius => self.value,
            TemperatureUnit::Fahrenheit => (self.value - 32.0) * FAHRENHEIT_TO_CELSIUS,
            TemperatureUnit::Kelvin => self.value - KELVIN_TO_CELSIUS,
        };

        let res: f64 = match target_unit {
            TemperatureUnit::Celsius => value_in_celsius,
            TemperatureUnit::Fahrenheit => value_in_celsius * CELSIUS_TO_FAHRENHEIT + 32.0,
            TemperatureUnit::Kelvin => value_in_celsius + KELVIN_TO_CELSIUS,
        };

        return Temperature {
            unit: target_unit,
            value: res,
        };
    }
}
