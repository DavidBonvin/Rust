fn main() {
    
 
    println!("Entrez votre prenom, sil vous plait: ");
    let mut nombre: String = String::new();
    std::io::stdin().read_line(&mut nombre).unwrap();
    nombre = nombre.trim().to_string(); 

    println!("Entrez votre nationalité, sil vous plait: "); 
    let mut pais: String = String::new();
    std::io::stdin().read_line(&mut pais).unwrap(); 
    
    pais = pais.trim().to_string(); 

    println!("hola, bienvenido o bienvenida {} de {} " , nombre, pais);
    println!("entre votre age, sil vous plait: ");
    let mut age : String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
   
    // convertir ça age en numero
    let age_int : u8 = age.trim().parse().unwrap();
    if age_int >= 18 {
        println!("vous pouvez entre dans la disco")
    }else {
        println!("vous etez un(e) adolecente encore"); 
    }
    println!(" vous avez {} ans ", age_int);

}

