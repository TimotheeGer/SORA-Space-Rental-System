use crate::display::utils::{display_database, display_processed_confirm};

pub fn indice_menu() {
    
    loop {
        display_database();
        println!(
            r"Je suis le principal processeur d'informations de la biologie, utilisant des synapses au lieu de circuits. 
On me compare souvent à une machine très avancée, mais aucune technologie actuelle ne peut égaler ma capacité de traitement en parallèle. 
Qui suis-je?"
        );
        println!();
        match display_processed_confirm("quitter ?".to_owned(), "".to_owned()) {
            Ok(comfirm) => {
                if comfirm {
                    break;
                }
            },
            Err(_) => break
        };
    }
}