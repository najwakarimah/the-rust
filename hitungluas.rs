use std::io;

// Fungsi untuk menghitung luas persegi panjang
fn luas_persegi_panjang(panjang: f64, lebar: f64) -> f64 {
    panjang * lebar
}

// Fungsi untuk menghitung luas segitiga
fn luas_segitiga(alas: f64, tinggi: f64) -> f64 {
    0.5 * alas * tinggi
}

// Fungsi untuk menghitung luas persegi
fn luas_persegi(sisi: f64) -> f64 {
    sisi * sisi
}

// Fungsi untuk menghitung luas jajar genjang
fn luas_jajar_genjang(alas: f64, tinggi: f64) -> f64 {
    alas * tinggi
}

fn main() {
    println!("Program Hitung Luas Bangun Datar");

    // Persegi Panjang
    let mut input = String::new();
    println!("Masukkan panjang persegi panjang:");
    io::stdin().read_line(&mut input).unwrap();
    let panjang: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Masukkan lebar persegi panjang:");
    io::stdin().read_line(&mut input).unwrap();
    let lebar: f64 = input.trim().parse().unwrap();
    input.clear();

    println!(
        "Luas persegi panjang = {}",
        luas_persegi_panjang(panjang, lebar)
    );

    // Segitiga
    println!("\nMasukkan alas segitiga:");
    io::stdin().read_line(&mut input).unwrap();
    let alas: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Masukkan tinggi segitiga:");
    io::stdin().read_line(&mut input).unwrap();
    let tinggi: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Luas segitiga = {}", luas_segitiga(alas, tinggi));

    // Persegi
    println!("\nMasukkan sisi persegi:");
    io::stdin().read_line(&mut input).unwrap();
    let sisi: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Luas persegi = {}", luas_persegi(sisi));

    // Jajar Genjang
    println!("\nMasukkan alas jajar genjang:");
    io::stdin().read_line(&mut input).unwrap();
    let alas_jg: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Masukkan tinggi jajar genjang:");
    io::stdin().read_line(&mut input).unwrap();
    let tinggi_jg: f64 = input.trim().parse().unwrap();

    println!("Luas jajar genjang = {}", luas_jajar_genjang(alas_jg, tinggi_jg));
}
