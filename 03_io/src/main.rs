use std::io;

fn main() {
    println!("Dimmi qualcosa: ");

    // le variabili sono normalmente immutabili
    // e si usa mut per renderle mutabili.
    // Tutte le variabili devono essere inizializzate!
    let mut line = String::new();

    // lettura di una linea
    io::stdin()
        .read_line(&mut line)               // read_line() richiede un riferimento a mutabile
        .expect("error reading line");      // gestione errore (per evitare warning 'unused Result')

    // stampa variabile
    println!("Hai scritto {}", line);


    // lettura di un intero senza segno a 32 bit: u32
    println!("Inserisci un numero: ");
    let mut n = String::new();  // inizialmente e' una stringa
    io::stdin().read_line(&mut n).expect("Failed to read line");    // legge la linea
    // converte la stringa in u32, modificando il tipo di 'n'
    let n: u32 = n.trim().parse().expect("Please insert a number!");
    println!("Hai inserito {}", n);
}
