use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Errore: Devi fornire il percorso di una cartella.");
        eprintln!("Uso corretto: cargo run -- <percorso_cartella>");
        return;
    }

    let target_dir = &args[1];
    println!("Analizzo la cartella: {}", target_dir);

    // Legge il contenuto della directory passata
    let paths = fs::read_dir(target_dir).expect("Errore: Impossibile leggere la cartella");

    // Cicla attraverso tutti i file trovati
    for path in paths {
        let entry = path.expect("Errore nel leggere il file");
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(estensione_grezza) = file_path.extension() {
                let estensione_testo = estensione_grezza.to_str().unwrap();
                let mut nuovo_percorso = PathBuf::from(target_dir);
                nuovo_percorso.push(estensione_testo);
                
                // Crea la sottocartella se non esiste
                fs::create_dir_all(&nuovo_percorso).expect("Impossibile creare la cartella");
                
                let nome_file = file_path.file_name().unwrap();
                nuovo_percorso.push(nome_file);
                
                // Sposta il file
                fs::rename(&file_path, &nuovo_percorso).expect("Impossibile spostare il file");
                println!("Spostato {:?} nella cartella {}", nome_file, estensione_testo);
            }
        }
    }
}