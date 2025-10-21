use std::io;

fn main() {
    println!("🧮 Calculator Rust ");

    loop {
        let mut input1 = String::new();
        println!("\nMasukkan angka pertama, atau ketik 'exit' untuk keluar:");
        io::stdin().read_line(&mut input1).expect("Gagal membaca input");
        let input1 = input1.trim();
        if input1.eq_ignore_ascii_case("exit") {
            break;
        }
        let num1: f64 = match input1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input tidak valid!");
                continue;
            }
        };

        let mut op = String::new();
        println!("Masukkan operator (+, -, *, /):");
        io::stdin().read_line(&mut op).expect("Gagal membaca input");
        let op = op.trim();

        let mut input2 = String::new();
        println!("Masukkan angka kedua:");
        io::stdin().read_line(&mut input2).expect("Gagal membaca input");
        let num2: f64 = match input2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input tidak valid!");
                continue;
            }
        };

        let result = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Tidak bisa membagi dengan 0!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => {
                println!("Operator tidak valid!");
                continue;
            }
        };

        println!("Hasil: {}", result);
    }

    println!("Terima kasih! 👋");
}
