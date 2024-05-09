use crate::display::utils::{display_brand, display_processed_confirm};

pub fn description_menu() {
    
    loop {
        display_brand();
        println!(
            r"Découvrez Sora : votre solution d'espaces de travail flexibles.

Idéal pour les professionnels et les entreprises, nous offrons des bureaux équipés,
des salles de réunion technologiques et des services de qualité.
Nos formules adaptatives, de la location ponctuelle à l'abonnement mensuel,
répondent à tous vos besoins. Visitez Sora.space pour plus d'informations.
"
        );
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