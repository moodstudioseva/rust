use std::io;

fn main() {
    loop {
        println!("Umrechnung Fahrenheit in Celsius.");
        println!("Fahrenheit zu Celsius [fc], Celsius zu Fahreneinheit [cf]");
        let auswahl = input("Auswahl: ");
        if auswahl != "fc" && auswahl != "cf" {
            continue;
        }
        let eingabe = input("Gib eine Zahl ein: ");
        let mut zahl: f32 = match eingabe.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if auswahl == "fc" {
            zahl = (zahl - 32.0) * 5.0 / 9.0;
            println!("Es sind {} Grad Celsius.", zahl);
        } else if auswahl == "cf" {
            zahl = zahl * 9.0 / 5.0 + 32.0;
            println!("Es sind {} Fahrenheit.", zahl);
        }
    }
}

fn input(x: &str) -> String {
    println!("{}", x);
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Fehler beim Lesen der Zeile");
    text.trim().to_string()
}
