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

fn koordinat() -> (i32, i32) {
    (30, 50)          // ← tidak pakai `return`
}


//konversi detik
use std::io;

// --------- FUNGSI ---------
fn konversi_detik(total: u32) -> (u32, u32, u32) {
    let jam    = total / 3600;          // 1 jam = 3600 detik
    let sisa   = total % 3600;
    let menit  = sisa / 60;
    let detik  = sisa % 60;

    (jam, menit, detik)                 // <- tuple sebagai return value
}

fn hitung_nilai_mhs(){
        // 1. Buat vector tuple siswa
        let siswa = vec![
            ("Ali".to_string(), 85),
            ("Budi".to_string(), 60),
            ("Citra".to_string(), 90),
            ("Dina".to_string(), 45),
            ("gugun".to_string(), 99),
            ("gugun".to_string(), 44),
        ];
    
        // 2. Tampilkan semua data
        println!("Daftar Siswa:");
        for (nama, nilai) in &siswa {
            println!("- {}: {}", nama, nilai);
        }
    
        // 3. Tampilkan yang lulus (nilai >= 70)
        println!("\n Yang Lulus:");
        for (nama, nilai) in &siswa {
            if *nilai >= 70 {
                println!("- {}: {}", nama, nilai);
            }
        }

        // 4. Tampilkan yang tidak lulus (nilai <= 70)
        println!("\n Yang Tidak Lulus:");
        for (nama, nilai) in &siswa {
            if *nilai <= 70 {
                println!("- {}: {}", nama, nilai);
            }
        }
    
        // nilai tertinggi
        if let Some((nama_top, nilai_top)) = siswa.iter().max_by_key(|(_, n)| n) {
            println!("\n🏅 Nilai tertinggi: {} ({})", nama_top, nilai_top);
        }

        // 4. Hitung rata-rata nilai
        let total: u32 = siswa.iter().map(|(_, n)| n).sum();
        let rata2 = total as f32 / siswa.len() as f32;
        println!("\n Rata-rata nilai: {:.2}", rata2);
}




fn main() {
    //tampilkan_profil("Rustacean", 28);
    //println!();
    //hitung_kelipatan(7);
    //cocok();
    //apalah();
    //nilai_match();
    //ssss();
    ////////////////////////// konversi detik ////////////////////////////////////////////////
    // println!("Masukkan total detik:");

    // // baca input user
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).unwrap();
    // let total: u32 = input.trim().parse().expect("Harus angka positif!");

    // // panggil fungsi dan 'bongkar' tuple
    // let (jam, menit, detik) = konversi_detik(total);

    // println!("{total} detik = {jam} jam {menit} menit {detik} detik");
////////////////////////////////////////////////////////////////////////
    // alia();
    // let (x, y) = koordinat();
    // println!("x = {x}, y = {y}");

    hitung_nilai_mhs()

}



