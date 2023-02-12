use rust_decimal::Decimal;

#[derive(Debug)]
pub(crate) struct DonneesCalibration {
    pub dig_t1: u16,
    pub dig_t2: i16,
    pub dig_t3: i16,

    pub dig_p1: u16,
    pub dig_p2: i16,
    pub dig_p3: i16,
    pub dig_p4: i16,
    pub dig_p5: i16,
    pub dig_p6: i16,
    pub dig_p7: i16,
    pub dig_p8: i16,
    pub dig_p9: i16,

    pub dig_h1: u8,
    pub dig_h2: i16,
    pub dig_h3: i8,
    pub dig_h4: u16,
    pub dig_h5: u16,
    pub dig_h6: i8,
}

#[derive(Debug)]
#[derive(Default)]
pub struct Donnees {
    pub temperature_degre_celsius: Option<Decimal>,
    pub humidite_pourcent: Option<Decimal>,
    pub pression_hpa: Option<Decimal>,
}
