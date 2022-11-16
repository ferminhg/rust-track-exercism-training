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

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}



pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;


const EARTH_DAYS: f64 = 365.25_f64;
const MERCURY_DAYS: f64 = 0.2408467_f64;
const VENUS_DAYS: f64 = 0.61519726_f64;
const MARS_DAYS: f64 = 1.8808158_f64;
const JUPITER_DAYS: f64 = 11.862615_f64;
const SATURN_DAYS: f64 = 29.447498_f64;
const URANUS_DAYS: f64 = 84.016846_f64;
const NEPTUNE_DAYS: f64 = 164.79132_f64;
const EARTH_SECONDS: f64 = EARTH_DAYS * 24_f64 * 60_f64 * 60_f64 ;

fn years_calculator(seconds: f64, planet_days: f64) -> f64 {
    (seconds / (planet_days * EARTH_SECONDS)) as f64
}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, MERCURY_DAYS)
    }
}
impl Planet for Venus {fn years_during(d: &Duration) -> f64 {
    years_calculator(d.seconds, VENUS_DAYS)
}}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, 1.0)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, MARS_DAYS)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, JUPITER_DAYS)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, SATURN_DAYS)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, URANUS_DAYS)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        years_calculator(d.seconds, NEPTUNE_DAYS)
    }
}
