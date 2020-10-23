use std::io;

fn main() {

    loop {
        println!("Umrechnung Fahreneinheit in Celsius.");
        println!("Gib eine Zahl ein:");
        let mut zahl = String::new();
        
        
    
        io::stdin()
            .read_line(&mut zahl)
            .expect("Fehler beim Lesen der Zeile");
        
        let zahl: f32 = match zahl.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
       
        let zahl = (zahl - 32.0) * 5.0/9.0;

        println!("Es sind {} Grad Celsius.", zahl);
    }
    
}
