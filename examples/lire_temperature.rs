use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use bme_280::capteur::Capteur;

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    let arret_demande = Arc::new(AtomicBool::new(false));
    env_logger::init();

    let arret_demande_clone = arret_demande.clone();

    ctrlc::set_handler(move || {
        log::info!("Lecture des températures : arrêt demandé");
        arret_demande_clone.store(true, Ordering::SeqCst);
    })
    .unwrap();

    let mut capteur = Capteur::new().unwrap();
    capteur.demarrer().unwrap();

    // Première lecture après initialisation ignorée
    capteur.lire_donnees().unwrap();
    thread::sleep(Duration::from_secs(1));

    while !arret_demande.load(Ordering::SeqCst) {
        let donnees = capteur.lire_donnees().unwrap();

        log::info!("Données capteur {:?}", donnees);

        thread::sleep(Duration::from_secs(60));
    }
}
