// fn halo() {
//     println!("Halo dari fungsi!");
// }

// fn main() {

//     halo();
//     halo();

//     for i in 1..5 {
//         println!("ke-{}", i);
//         halo();
//     }

//     let mut co = 1;

//     while co < 8 {
//         println!("while ke-{}", co);
//         halo();
//         co += 1;
//     }

    
// }

// fn sapa(nama: &str) {
//     println!("Hai, {}!", nama);
// }

// fn main() {
//     sapa("Apridan");
//     sapa("Rustacean");
// }



// fn tambah(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn main() {
//     let hasil = tambah(3, 5);
//     println!("Hasil penjumlahan: {}", hasil);
// }


fn tampilkan_profil(nama: &str, umur: u8) {
    println!("Nama: {}", nama);
    println!("Umur: {} tahun", umur);
}

fn hitung_kelipatan(x: u32) {
    for i in 1..=5 {
        println!("{} x {} = {}", x, i, x * i);
    }
}

fn cocok() {
    let aku = 12;
    match aku {
        1 => println!("saya"),
        12 => println!("saya {}", aku),
        _ => println!("noting!!")
    } 
}

fn apalah() {

    let mut i = 1;
    while i <= 99 {
        println!("ini ke-{}", i);

        i += 3;
    }
}

fn nilai_match() {
    let nilai = 90;
    
    match nilai {
        1..=5 => println!("bodoh"),
        6..=7 => println!("lumayan"),
        8..=10 => println!("amajing"),
        n if n > 10 => println!("amajing+++"),
        _ => println!("noting!!")
    }
}

fn ssss() {
    let angka = 214748364;

    match angka {
        kecil @ 0..=7 => println!("kecil ({})", kecil),
        sedang @ 8..=10 => println!("amajing ({})", sedang),
        besar @ 11..=i32::MAX => println!("good ({})", besar),
        _ => println!("nilai tak terduga"),
    }
}

// tuple 

fn alia() {

    let profile = ("aligator", 222, 321);

    println!("nama : {}", profile.0);
    println!("umur : {}", profile.1);
    println!("tinggi : {}", profile.2);
}



fn main() {
    tampilkan_profil("Rustacean", 28);
    println!();
    hitung_kelipatan(7);
    cocok();
    apalah();
    nilai_match();
    ssss();
    alia();
}



