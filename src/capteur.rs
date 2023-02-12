use crate::commande::{AdresseCapteur, Commande};
use crate::donnees::{Donnees, DonneesCalibration};
use rppal::i2c::I2c;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

pub struct Capteur {
    i2c: I2c,
    big_endian: bool,
    donnees_calibration: Option<DonneesCalibration>,
}

impl Capteur {
    pub fn new() -> Result<Self, rppal::i2c::Error> {
        let big_endian;
        if cfg!(target_endian = "big") {
            big_endian = true;
        } else {
            big_endian = false;
        }

        Ok(Self {
            i2c: I2c::new()?,
            big_endian,
            donnees_calibration: None,
        })
    }

    pub fn demarrer(&mut self) -> Result<(), rppal::i2c::Error> {
        self.i2c
            .set_slave_address(AdresseCapteur::I2cAddressA.adresse())?;

        let mut chip_id_btyes = [0u8; 2];
        self.i2c
            .block_write(Commande::RegisterChipid.adresse() as u8, &[])?;
        self.i2c
            .block_read(Commande::RegisterChipid.adresse() as u8, &mut chip_id_btyes)?;

        let chip_id;
        match self.big_endian {
            true => chip_id = u16::from_be_bytes(chip_id_btyes),
            false => chip_id = u16::from_le_bytes(chip_id_btyes),
        }

        if Commande::ChipIdBme280.adresse() == chip_id {
            log::info!(
                "Sonde trouvée : bme280. Identifiant puce trouvé: 0x{:x}",
                chip_id,
            );
        } else if Commande::ChipId1Bmp280.adresse() == chip_id {
            log::info!(
                "Sonde trouvée : bmp280 (1). Identifiant puce trouvé: 0x{:x}",
                chip_id,
            );
        } else if Commande::ChipId2Bmp280.adresse() == chip_id {
            log::info!(
                "Sonde trouvée : bmp280 (2). Identifiant puce trouvé: 0x{:x}",
                chip_id,
            );
        } else if Commande::ChipId3Bmp280.adresse() == chip_id {
            log::info!(
                "Sonde trouvée : bme280 (3). Identifiant puce trouvé: 0x{:x}",
                chip_id,
            );
        } else {
            log::info!(
                "Sonde non trouvée. Identifiant puce trouvé: 0x{:x}",
                chip_id,
            );
        }

        self.charger_donnees_calibration()?;

        // Humidity 16x oversampling
        self.i2c
            .block_write(Commande::RegisterControlHum.adresse() as u8, &[0x5])?;

        // Temperature/pressure 16x oversampling, normal mode
        self.i2c
            .block_write(Commande::RegisterControl.adresse() as u8, &[0xb7])?;

        Ok(())
    }

    pub fn charger_donnees_calibration(&mut self) -> Result<(), rppal::i2c::Error> {
        let mut donnees_calibration = DonneesCalibration {
            dig_t1: 0,
            dig_t2: 0,
            dig_t3: 0,
            dig_p1: 0,
            dig_p2: 0,
            dig_p3: 0,
            dig_p4: 0,
            dig_p5: 0,
            dig_p6: 0,
            dig_p7: 0,
            dig_p8: 0,
            dig_p9: 0,
            dig_h1: 0,
            dig_h2: 0,
            dig_h3: 0,
            dig_h4: 0,
            dig_h5: 0,
            dig_h6: 0,
        };

        let mut donnees_capteur = [0u8; 24];
        self.i2c.block_read(
            Commande::RegisterDigT1.adresse() as u8,
            &mut donnees_capteur,
        )?;

        let mut h1 = [0u8; 1];
        let mut h2 = [0u8; 2];
        let mut h3 = [0u8; 1];
        let mut h4 = [0u8; 1];
        let mut h5 = [0u8; 1];
        let mut h5_1 = [0u8; 1];
        let mut h6 = [0u8; 1];

        self.i2c
            .block_read(Commande::RegisterDigH1.adresse() as u8, &mut h1)?;

        self.i2c
            .block_read(Commande::RegisterDigH2.adresse() as u8, &mut h2)?;

        self.i2c
            .block_read(Commande::RegisterDigH3.adresse() as u8, &mut h3)?;

        self.i2c
            .block_read(Commande::RegisterDigH4.adresse() as u8, &mut h4)?;

        self.i2c
            .block_read(Commande::RegisterDigH5.adresse() as u8, &mut h5)?;

        self.i2c
            .block_read(Commande::RegisterDigH5_1.adresse() as u8, &mut h5_1)?;

        self.i2c
            .block_read(Commande::RegisterDigH6.adresse() as u8, &mut h6)?;

        match self.big_endian {
            true => {}
            false => {
                donnees_calibration.dig_t1 =
                    u16::from_le_bytes([donnees_capteur[0], donnees_capteur[1]]);
                donnees_calibration.dig_t2 =
                    i16::from_le_bytes([donnees_capteur[2], donnees_capteur[3]]);
                donnees_calibration.dig_t3 =
                    i16::from_le_bytes([donnees_capteur[4], donnees_capteur[5]]);

                donnees_calibration.dig_p1 =
                    u16::from_le_bytes([donnees_capteur[6], donnees_capteur[7]]);
                donnees_calibration.dig_p2 =
                    i16::from_le_bytes([donnees_capteur[8], donnees_capteur[9]]);
                donnees_calibration.dig_p3 =
                    i16::from_le_bytes([donnees_capteur[10], donnees_capteur[11]]);
                donnees_calibration.dig_p4 =
                    i16::from_le_bytes([donnees_capteur[12], donnees_capteur[13]]);
                donnees_calibration.dig_p5 =
                    i16::from_le_bytes([donnees_capteur[14], donnees_capteur[15]]);
                donnees_calibration.dig_p6 =
                    i16::from_le_bytes([donnees_capteur[16], donnees_capteur[17]]);
                donnees_calibration.dig_p7 =
                    i16::from_le_bytes([donnees_capteur[18], donnees_capteur[19]]);
                donnees_calibration.dig_p8 =
                    i16::from_le_bytes([donnees_capteur[20], donnees_capteur[21]]);
                donnees_calibration.dig_p9 =
                    i16::from_le_bytes([donnees_capteur[22], donnees_capteur[23]]);

                donnees_calibration.dig_h1 = h1[0];
                donnees_calibration.dig_h2 = i16::from_le_bytes(h2);
                donnees_calibration.dig_h3 = h3[0] as i8;
                donnees_calibration.dig_h4 = (h4[0] as u16) << 4 | (h5[0] as u16) & 0xF;
                donnees_calibration.dig_h5 = (h5_1[0] as u16) << 4 | (h5[0] as u16) >> 4;
                donnees_calibration.dig_h6 = h6[0] as i8;
            }
        }
        self.donnees_calibration = Some(donnees_calibration);
        Ok(())
    }

    pub fn lire_donnees(&mut self) -> Result<Donnees, rppal::i2c::Error> {
        let mut donnees = Donnees::default();
        let mut donnees_capteur = [0u8; 8];
        self.i2c.block_read(
            Commande::RegisterPressureData.adresse() as u8,
            &mut donnees_capteur,
        )?;

        let adc_t: u32;
        match self.big_endian {
            true => {
                adc_t = 0;
            }
            false => {
                adc_t = u32::from_le_bytes([
                    0,
                    donnees_capteur[5],
                    donnees_capteur[4],
                    donnees_capteur[3],
                ]) >> 12;
            }
        };
        let donnees_calibration = self.donnees_calibration.as_ref().unwrap();

        // Calcul de la température en dégrés celsius
        let tvar1 = (((adc_t >> 3) - ((donnees_calibration.dig_t1 as u32) << 1))
            * (donnees_calibration.dig_t2 as u32))
            >> 11;
        let tvar2 = (((((adc_t >> 4) - (donnees_calibration.dig_t1 as u32))
            * ((adc_t >> 4) - (donnees_calibration.dig_t1 as u32)))
            >> 12)
            * (donnees_calibration.dig_t3 as u32))
            >> 14;
        let t_fine = (tvar1 + tvar2) as i64;
        let temperature_c: f64 = ((t_fine * 5 + 128) >> 8) as f64 / 100.;

        donnees.temperature_degre_celsius = Decimal::from_f64(temperature_c);

        // Calcul de la pression en hectopascals

        let adc_p: i64;
        match self.big_endian {
            true => {
                adc_p = 0;
            }
            false => {
                adc_p = (u32::from_le_bytes([
                    0,
                    donnees_capteur[2],
                    donnees_capteur[1],
                    donnees_capteur[0],
                ]) >> 12) as i64;
            }
        }
        let mut pvar1 = t_fine as f64 / 2. - 64000.;
        let mut pvar2 = pvar1 * pvar1 * donnees_calibration.dig_p6 as f64 / 32768.;
        pvar2 = pvar2 + pvar1 * donnees_calibration.dig_p5 as f64 * 2.;
        pvar2 = pvar2 / 4. + donnees_calibration.dig_p4 as f64 * 65536.;
        pvar1 = (donnees_calibration.dig_p3 as f64 * pvar1 * pvar1 / 524288.
            + donnees_calibration.dig_p2 as f64 * pvar1)
            / 524288.;
        pvar1 = (1. + pvar1 / 32768.) * donnees_calibration.dig_p1 as f64;

        let pression_hectopascals: f64;

        if pvar1 != 0. {
            let mut p = (1048576 - adc_p) as f64;
            p = ((p - pvar2 / 4096.) * 6250.) / pvar1;
            pvar1 = donnees_calibration.dig_p9 as f64 * p * p / 2147483648.;
            pvar2 = p * donnees_calibration.dig_p8 as f64 / 32768.;
            p = p + (pvar1 + pvar2 + donnees_calibration.dig_p7 as f64) / 16.;

            pression_hectopascals = p / 100.;
        } else {
            pression_hectopascals = 0.;
        }

        donnees.pression_hpa = Decimal::from_f64(pression_hectopascals);

        // Calcul de l'humidité en pourcents (bme280)

        let adc_h;
        match self.big_endian {
            true => {
                adc_h = 0;
            }
            false => {
                adc_h = (u16::from_le_bytes([donnees_capteur[7], donnees_capteur[6]])) as i64;
            }
        }

        let mut h = t_fine as f64 - 76800.;
        h = (adc_h as f64
            - (donnees_calibration.dig_h4 as f64 * 64.
                + donnees_calibration.dig_h5 as f64 / 16384. * h))
            * (donnees_calibration.dig_h2 as f64 / 65536.
                * (1.
                    + donnees_calibration.dig_h6 as f64 / 67108864.
                        * h
                        * (1. + donnees_calibration.dig_h3 as f64 / 67108864. * h)));
        h = h * (1. - donnees_calibration.dig_h1 as f64 * h / 5242883.);

        let humidite;
        if 100. < h {
            humidite = 100.;
        } else if h < 0. {
            humidite = 0.;
        } else {
            humidite = h;
        }

        donnees.humidite_pourcent = Decimal::from_f64(humidite);

        Ok(donnees)
    }
}
