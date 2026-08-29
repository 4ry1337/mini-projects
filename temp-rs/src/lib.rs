use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kelvin(pub f32);

impl Kelvin {
    pub const MIN: Self = Self(0.0);
    pub const MAX: Self = Self(f32::MAX);

    #[must_use]
    pub const fn new(temp: f32) -> Self {
        Self(temp)
    }

    #[must_use]
    pub fn valid(self) -> bool {
        (Self::MIN..Self::MAX).contains(&self)
    }
}

impl Deref for Kelvin {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Kelvin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Celsius> for Kelvin {
    fn from(value: Celsius) -> Self {
        Self(value.0 + 273.15)
    }
}

impl From<Fahrenheit> for Kelvin {
    fn from(value: Fahrenheit) -> Self {
        Self((value.0 + 459.67) * (5.0 / 9.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl Celsius {
    pub const MIN: Self = Self(-273.15);
    pub const MAX: Self = Self(f32::MAX);

    #[must_use]
    pub const fn new(temp: f32) -> Self {
        Self(temp)
    }

    #[must_use]
    pub fn valid(self) -> bool {
        (Self::MIN..Self::MAX).contains(&self)
    }
}

impl Deref for Celsius {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Celsius {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(value: Fahrenheit) -> Self {
        Self((value.0 - 32.0) * (5.0 / 9.0))
    }
}

impl From<Kelvin> for Celsius {
    fn from(value: Kelvin) -> Self {
        Self(value.0 - 273.15)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f32);

impl Fahrenheit {
    pub const MIN: Self = Self(-459.67);
    pub const MAX: Self = Self(f32::MAX);

    #[must_use]
    pub const fn new(temp: f32) -> Self {
        Self(temp)
    }

    #[must_use]
    pub fn valid(self) -> bool {
        (Self::MIN..Self::MAX).contains(&self)
    }
}

impl Deref for Fahrenheit {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fahrenheit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(value: Celsius) -> Self {
        Self(value.0.mul_add(9.0 / 5.0, 32.0))
    }
}

impl From<Kelvin> for Fahrenheit {
    fn from(value: Kelvin) -> Self {
        Self(value.0.mul_add(9.0 / 5.0, -459.67))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Celsius, Fahrenheit, Kelvin};

    #[test]
    fn valid_celsius() {
        assert_eq!(Celsius::new(-273.15).valid(), true);
        assert_eq!(Celsius::new(-460.0).valid(), false);
    }

    #[test]
    fn valid_fahrenheit() {
        assert_eq!(Fahrenheit::new(459.67).valid(), true);
        assert_eq!(Fahrenheit::new(-460.0).valid(), false);
    }

    #[test]
    fn valid_kelvin() {
        assert_eq!(Kelvin::new(0.0).valid(), true);
        assert_eq!(Kelvin::new(-1.0).valid(), false);
    }

    #[test]
    fn fahrenheit_to_celsius() {
        assert_eq!(
            Celsius::from(Fahrenheit::new(0.0)),
            Celsius::new(-17.77777777777778)
        );
    }

    #[test]
    fn kelvin_to_celsius() {
        assert_eq!(Celsius::from(Kelvin::new(0.0)), Celsius::new(-273.15));
    }

    #[test]
    fn fahrenheit_to_kelvin() {
        assert_eq!(Kelvin::from(Fahrenheit::new(32.0)), Kelvin::new(273.15002));
    }

    #[test]
    fn celsius_to_kelvin() {
        assert_eq!(Kelvin::from(Celsius::new(0.0)), Kelvin::new(273.15));
    }

    #[test]
    fn celsius_to_fahrenheit() {
        assert_eq!(Fahrenheit::from(Celsius::new(0.0)), Fahrenheit::new(32.0));
    }

    #[test]
    fn kelvin_to_fahrenheit() {
        assert_eq!(Fahrenheit::from(Kelvin::new(0.0)), Fahrenheit::new(-459.67));
    }
}
