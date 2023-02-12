# bme280-rust

![Création de l'exemple lire_temperature](https://github.com/ftmazzone/bme280-rust/actions/workflows/construction/badge.svg)

Compilation croisée ARMv6 pour Raspberry Pi Zero

# Exemple 

- Mesure de l'humidité, de la température et la pression atmosphérique avec le capteur bme280

```bash
cargo run --example lire_temperature
```

# Credits

* [bme280 datasheet](https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme280-ds002.pdf)
* [pimoroni BME280](https://github.com/pimoroni/bme280-python)
* [abhiTronix's Raspberry Pi ToolChains V3](https://github.com/abhiTronix/raspberry-pi-cross-compilers)
