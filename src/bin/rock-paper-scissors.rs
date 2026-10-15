use rand::rng;
use rand::seq::IndexedRandom;
use std::io;

fn main() {
    let mut input_user = String::new();
    let mut rng = rng();
    let array: [&str; 3] = ["batu", "gunting", "kertas"];
    let bot = array.choose(&mut rng).unwrap();

    println!("Masukkan nilai");

    io::stdin()
        .read_line(&mut input_user)
        .expect("Gagal membaca nilai input");

    if input_user.trim() == "batu" && bot.trim() == "kertas" {
        println!("kalah, bot memilih {} dan kamu {}", bot, input_user.trim());
    } else if input_user.trim() == "gunting" && bot.trim() == "batu" {
        println!("kalah, bot memilih {} dan kamu {}", bot, input_user.trim());
    } else if input_user.trim() == "kertas" && bot.trim() == "gunting" {
        println!("kalah, bot memilih {} dan kamu {}", bot, input_user.trim());
    } else if input_user.trim() == bot.trim() {
        println!("seri, bot memilih {} dan kamu {}", bot, input_user.trim());
    } else {
        println!(
            "kamu menang. bot: {} | kamu: {}",
            bot.trim(),
            input_user.trim()
        )
    }
}
