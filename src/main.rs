extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Deviner le nombre!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Le nombre secret est : {}", secret_number);

    println!("Merci d'insérer le nombre.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Impossible de lire la ligne");

    let guess: u32 = guess.trim().parse()
        .ok()
        .expect("Merci de rentrer un numéro");

    println!("Vous avez trouvé: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Trop petit!"),
        Ordering::Greater => println!("Trop grand!"),
        Ordering::Equal   => println!("BIMMMM"),
    }
}
