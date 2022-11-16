// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

const EARTH_SECONDS: f64 = 31557600.0 ;

pub trait Planet {
    const RELATIVE_TO_EARTH: f64 = 1.0;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (EARTH_SECONDS * Self::RELATIVE_TO_EARTH)
    }
}

macro_rules! planet {
    ($struct_name:ident, $relative_year:expr) => {
        pub struct $struct_name;
        impl Planet for $struct_name {
            const RELATIVE_TO_EARTH: f64 = $relative_year;
        }
    };
}

const MERCURY_DAYS: f64 = 0.2408467_f64;
const VENUS_DAYS: f64 = 0.61519726_f64;
const MARS_DAYS: f64 = 1.8808158_f64;
const JUPITER_DAYS: f64 = 11.862615_f64;
const SATURN_DAYS: f64 = 29.447498_f64;
const URANUS_DAYS: f64 = 84.016846_f64;
const NEPTUNE_DAYS: f64 = 164.79132_f64;

planet!(Mercury, MERCURY_DAYS);
planet!(Venus, VENUS_DAYS);
planet!(Earth, 1.0);
planet!(Mars, MARS_DAYS);
planet!(Jupiter, JUPITER_DAYS);
planet!(Saturn, SATURN_DAYS);
planet!(Uranus, URANUS_DAYS);
planet!(Neptune, NEPTUNE_DAYS);



