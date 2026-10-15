// Belajar Rust Dasar: Variabel, Tipe Data, Operator, Fungsi, If-Else & Loop

fn main() {
    println!("=== BELAJAR RUST DASAR ===\n");

    // 1. VARIABEL
    println!("1. VARIABEL");
    variabel();
    println!();

    // 2. TIPE DATA
    println!("2. TIPE DATA");
    tipe_data();
    println!();

    // 3. OPERATOR
    println!("3. OPERATOR");
    operator();
    println!();

    // 4. FUNGSI
    println!("4. FUNGSI");
    fungsi();
    println!();

    // 5. IF-ELSE CONDITION
    println!("5. IF-ELSE CONDITION");
    if_else();
    println!();

    // 6. LOOP
    println!("6. LOOP");
    loop_contoh();
    println!();

    // 7. CONTOH GABUNGAN - CEK BILANGAN PRIMA
    println!("7. CONTOH GABUNGAN (BILANGAN PRIMA)");
    cek_prima_contoh();
    println!();

    println!("=== SELESAI ===");
}

// 1. Variabel
fn variabel() {
    // Variabel immutable (tidak bisa diubah)
    let x = 5;
    println!("   x = {}", x);

    // Variabel mutable (bisa diubah)
    let mut y = 10;
    println!("   y sebelum diubah = {}", y);
    y = 20;
    println!("   y setelah diubah = {}", y);

    // Shadowing: mendeklarasikan ulang dengan nama yang sama
    let y = "sekarang jadi string";
    println!("   y setelah shadowing = {}", y);

    // Konstanta (wajib menyebutkan tipe data)
    const PI: f64 = 3.14159;
    println!("   PI = {}", PI);
}

// 2. Tipe Data
fn tipe_data() {
    // Integer (bilangan bulat)
    let angka_i32: i32 = 100; // signed 32-bit
    let angka_u64: u64 = 200; // unsigned 64-bit
    println!("   i32: {}, u64: {}", angka_i32, angka_u64);

    // Float (bilangan desimal)
    let desimal_f32: f32 = 3.14;
    let desimal_f64: f64 = 6.28;
    println!("   f32: {}, f64: {}", desimal_f32, desimal_f64);

    // Boolean
    let is_true: bool = true;
    let is_false: bool = false;
    println!("   true: {}, false: {}", is_true, is_false);

    // Karakter
    let huruf: char = 'R';
    println!("   char: {}", huruf);

    // String slice (&str) dan String
    let teks_str: &str = "Halo Rust";
    let teks_string: String = String::from("Belajar Rust");
    println!("   &str: {}", teks_str);
    println!("   String: {}", teks_string);

    // Tuple (kumpulan nilai dengan tipe berbeda)
    let biodata: (String, i32, bool) = (String::from("Lukman"), 20, true);
    let (nama, umur, aktif) = biodata; // destructuring
    println!("   Tuple -> Nama: {}, Umur: {}, Aktif: {}", nama, umur, aktif);

    // Array (jumlah elemen tetap, tipe data sama)
    let angka_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("   Array: {:?}", angka_arr);
    println!("   Elemen ke-0: {}", angka_arr[0]);
    println!("   Panjang array: {}", angka_arr.len());
}

// 3. Operator
fn operator() {
    let a: i32 = 10;
    let b: i32 = 3;

    // Operator Aritmatika
    println!("   Aritmatika:");
    println!("     a + b = {}", a + b);
    println!("     a - b = {}", a - b);
    println!("     a * b = {}", a * b);
    println!("     a / b = {} (pembagian integer)", a / b);
    println!("     a % b = {}", a % b);

    // Operator Perbandingan
    println!("   Perbandingan:");
    println!("     a == b ? {}", a == b);
    println!("     a != b ? {}", a != b);
    println!("     a > b  ? {}", a > b);
    println!("     a < b  ? {}", a < b);
    println!("     a >= b ? {}", a >= b);
    println!("     a <= b ? {}", a <= b);

    // Operator Logika
    let x: bool = true;
    let y: bool = false;
    println!("   Logika:");
    println!("     x && y = {}", x && y); // AND
    println!("     x || y = {}", x || y); // OR
    println!("     !x     = {}", !x);     // NOT

    // Operator Penugasan
    let mut c: i32 = 5;
    println!("   Penugasan:");
    println!("     c awal = {}", c);
    c += 2; // c = c + 2
    println!("     c += 2 => {}", c);
    c *= 3; // c = c * 3
    println!("     c *= 3 => {}", c);
}

// 4. Fungsi
fn fungsi() {
    sapa("Rust");

    let hasil_tambah = tambah(5, 7);
    println!("   Hasil penjumlahan 5 + 7 = {}", hasil_tambah);

    let hasil_kali = kali(4, 6);
    println!("   Hasil perkalian 4 * 6   = {}", hasil_kali);

    println!("   Luas persegi sisi 10   = {}", luas_persegi(10));
}

// Fungsi tanpa return
fn sapa(nama: &str) {
    println!("   Halo, {}!", nama);
}

// Fungsi dengan return (tanpa titik koma)
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

// Fungsi dengan return eksplisit (pakai keyword return)
fn kali(a: i32, b: i32) -> i32 {
    return a * b;
}

// Fungsi menghitung luas persegi
fn luas_persegi(sisi: i32) -> i32 {
    sisi * sisi
}

// 5. If-Else Condition
fn if_else() {
    let nilai: i32 = 85;

    // If-Else If-Else biasa
    println!("   Nilai = {}", nilai);
    if nilai >= 90 {
        println!("   Grade A");
    } else if nilai >= 80 {
        println!("   Grade B");
    } else if nilai >= 70 {
        println!("   Grade C");
    } else if nilai >= 60 {
        println!("   Grade D");
    } else {
        println!("   Grade E");
    }

    // If sebagai expression (bisa disimpan ke variabel)
    let status = if nilai >= 75 { "Lulus" } else { "Tidak Lulus" };
    println!("   Status: {}", status);

    // Cek genap/ganjil
    if nilai % 2 == 0 {
        println!("   {} adalah bilangan genap", nilai);
    } else {
        println!("   {} adalah bilangan ganjil", nilai);
    }
}

// 6. Loop
fn loop_contoh() {
    // a) loop - berhenti pakai break
    println!("   a) loop:");
    let mut counter: i32 = 1;
    loop {
        println!("      Loop ke-{}", counter);
        counter += 1;
        if counter > 5 {
            break;
        }
    }

    // b) while
    println!("   b) while:");
    let mut i: i32 = 1;
    while i <= 5 {
        println!("      While: {}", i);
        i += 1;
    }

    // c) for - range
    println!("   c) for dengan range 1..=5:");
    for angka in 1..=5 {
        println!("      For range: {}", angka);
    }

    // d) for - iterasi array
    println!("   d) for dengan iterasi array:");
    let buah = ["Apel", "Jeruk", "Mangga", "Pisang"];
    for b in buah.iter() {
        println!("      Buah: {}", b);
    }

    // e) for - dengan index pakai enumerate
    println!("   e) for dengan index (enumerate):");
    for (index, b) in buah.iter().enumerate() {
        println!("      {}: {}", index + 1, b);
    }
}

// 7. Contoh gabungan: Cek bilangan prima
fn cek_prima_contoh() {
    let angka: i32 = 7;
    if is_prima(angka) {
        println!("   {} adalah bilangan prima", angka);
    } else {
        println!("   {} bukan bilangan prima", angka);
    }

    let angka2: i32 = 4;
    if is_prima(angka2) {
        println!("   {} adalah bilangan prima", angka2);
    } else {
        println!("   {} bukan bilangan prima", angka2);
    }
}

fn is_prima(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i: i32 = 2;
    while i < n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}