use std::io;
fn main() {
    loop {

        println!("Berechnung einer n-ten Fibonacci Zahl.");
        let eingabe = input("Wähle eine Zahl: ");
        let mut zahl: f32 = match eingabe.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        zahl = 1.0/5_f32.sqrt()*(((1.0+5_f32.sqrt())/2.0).powf(zahl)-((1.0-5_f32.sqrt())/2.0).powf(zahl));
        println!("Die Fibonacci Zahl lautet: {}", zahl);

    }
    
}

fn input(x: &str) -> String {
    println!("{}", x);
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Fehler beim Lesen der Zeile.");
        text.trim().to_string()
        

}
