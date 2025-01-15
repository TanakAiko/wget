pub mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    // Récupérer l'URL
    let _url = matches.args_present();
    println!("matches.args_present() : {}", matches.args_present());

    let a = matches.contains_id("url");
    println!("matches.contains_id('url') : {}", a);

    let b = matches.value_source("url").unwrap();
    println!("matches.value_source('url') : {:?}", b);

    let url: &String = matches.get_one::<String>("url").expect("URL is required");
    println!("Téléchargement depuis l'URL : {}", url);

    // Récupérer le nom de sortie
    /* if let Some(output) = matches.value_of("output") {
        println!("Le fichier sera enregistré sous : {}", output);
    }

    // Récupérer le répertoire de sortie
    if let Some(path) = matches.value_of("path") {
        println!("Le fichier sera enregistré dans : {}", path);
    } */
}
