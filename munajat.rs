use std::{thread, time::Duration};
use std::io::{stdout, Write};

fn main() {
    let lyrics = [
        "Tuhan, kirimkanlah aku",
        "Kekasih yang baik hati",
        "Yang mencintai aku",
        "Apa adanya",
    ];

    println!("🎵 Munajat Cinta\n");

    for line in lyrics.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();
        // Delay per kata supaya total ~5 detik per baris
        let delay_per_word = Duration::from_millis(5_000 / words.len() as u64);

        for word in words {
            print!("{} ", word);
            stdout().flush().unwrap();
            thread::sleep(delay_per_word);
        }
        println!(); // pindah baris
        thread::sleep(Duration::from_millis(300)); // delay antar baris
    }

    println!("\n--- Selesai ---");
}
