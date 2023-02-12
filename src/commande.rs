pub enum AdresseCapteur {
    I2cAddressB,
    I2cAddressA,
    Bme280DefaultI2cAddress,
}

pub(crate) enum Commande {
    RegisterDigT1,
    #[allow(dead_code)]
    RegisterDigT2,
    #[allow(dead_code)]
    RegisterDigT3,

    #[allow(dead_code)]
    RegisterDigP1,
    #[allow(dead_code)]
    RegisterDigP2,
    #[allow(dead_code)]
    RegisterDigP3,
    #[allow(dead_code)]
    RegisterDigP4,
    #[allow(dead_code)]
    RegisterDigP5,
    #[allow(dead_code)]
    RegisterDigP6,
    #[allow(dead_code)]
    RegisterDigP7,
    #[allow(dead_code)]
    RegisterDigP8,
    #[allow(dead_code)]
    RegisterDigP9,

    RegisterDigH1,
    RegisterDigH2,
    RegisterDigH3,
    RegisterDigH4,
    RegisterDigH5,
    RegisterDigH5_1,
    RegisterDigH6,

    RegisterChipid,
    #[allow(dead_code)]
    RegisterReset,

    RegisterControlHum,
    RegisterControl,
    RegisterPressureData,
    #[allow(dead_code)]
    RegisterTempData,
    #[allow(dead_code)]
    RegisterHumidityData,
    ChipId1Bmp280,
    ChipId2Bmp280,
    ChipId3Bmp280,
    ChipIdBme280,
}

impl AdresseCapteur {
    pub fn adresse(self) -> u16 {
        match self {
            AdresseCapteur::I2cAddressB => 0x76,
            AdresseCapteur::I2cAddressA => 0x77,
            AdresseCapteur::Bme280DefaultI2cAddress => 0x77,
        }
    }
}

impl Commande {
    pub(crate) fn adresse(self) -> u16 {
        let adresse = match self {
            Commande::ChipId1Bmp280 => 0x56,
            Commande::ChipId2Bmp280 => 0x57,
            Commande::ChipId3Bmp280 => 0x58,
            Commande::ChipIdBme280 => 0x60,
            Commande::RegisterDigT1 => 0x88,
            Commande::RegisterDigT2 => 0x8A,
            Commande::RegisterDigT3 => 0x8C,
            Commande::RegisterDigP1 => 0x8E,
            Commande::RegisterDigP2 => 0x90,
            Commande::RegisterDigP3 => 0x92,
            Commande::RegisterDigP4 => 0x94,
            Commande::RegisterDigP5 => 0x96,
            Commande::RegisterDigP6 => 0x98,
            Commande::RegisterDigP7 => 0x9A,
            Commande::RegisterDigP8 => 0x9C,
            Commande::RegisterDigP9 => 0x9E,
            Commande::RegisterDigH1 => 0xA1,
            Commande::RegisterDigH2 => 0xE1,
            Commande::RegisterDigH3 => 0xE3,
            Commande::RegisterDigH4 => 0xE4,
            Commande::RegisterDigH5 => 0xE5,
            Commande::RegisterDigH5_1 => 0xE6,
            Commande::RegisterDigH6 => 0xE7,
            Commande::RegisterChipid => 0xD0,
            Commande::RegisterReset => 0xE0,
            Commande::RegisterControlHum => 0xF2,
            Commande::RegisterControl => 0xF4,
            Commande::RegisterPressureData => 0xF7,
            Commande::RegisterTempData => 0xFA,
            Commande::RegisterHumidityData => 0xFD,
        };
        adresse
    }
}
