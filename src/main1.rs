fun main1 {
    println!("C'est ta dernière chance, après ça, il n'y a pas de retour en arrière,\
    Prends la pilule bleue: l'histoire c'est finie, tu te réveilles dans ton lit, crois-y, ou tu viux coire,\
    Prends la pilule rouge  et tu reste dans au pays des merveilles et je te montrerai à quelle profondeur se trouve la maison des lapins,\
    rappelle-toi, tout le que je t'offre est la verité, rien de plus. ");

    let mut pilule :  String = String::new();
    std::io::stdin().read_line(&mut pilule).unwrap();

    //nettoyer l'entrée de la pilule 
    pilule = pilule.trim().to_string();

    // if, la pilule est rouge, on reste dans la matrix
    // if est bleue, on quitte la matrix
    
    // dans tous les case, la matrix explose et tout le monde meurt.__rust_force_expr! 
}