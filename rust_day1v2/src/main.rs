// // fn halo() {
// //     println!("Halo dari fungsi!");
// // }

// // fn main() {

// //     halo();
// //     halo();

// //     for i in 1..5 {
// //         println!("ke-{}", i);
// //         halo();
// //     }

// //     let mut co = 1;

// //     while co < 8 {
// //         println!("while ke-{}", co);
// //         halo();
// //         co += 1;
// //     }

    
// // }

// // fn sapa(nama: &str) {
// //     println!("Hai, {}!", nama);
// // }

// // fn main() {
// //     sapa("Apridan");
// //     sapa("Rustacean");
// // }



// // fn tambah(a: i32, b: i32) -> i32 {
// //     a + b
// // }

// // fn main() {
// //     let hasil = tambah(3, 5);
// //     println!("Hasil penjumlahan: {}", hasil);
// // }


// fn tampilkan_profil(nama: &str, umur: u8) {
//     println!("Nama: {}", nama);
//     println!("Umur: {} tahun", umur);
// }

// fn hitung_kelipatan(x: u32) {
//     for i in 1..=5 {
//         println!("{} x {} = {}", x, i, x * i);
//     }
// }

// fn cocok() {
//     let aku = 12;
//     match aku {
//         1 => println!("saya"),
//         12 => println!("saya {}", aku),
//         _ => println!("noting!!")
//     } 
// }

// fn apalah() {

//     let mut i = 1;
//     while i <= 99 {
//         println!("ini ke-{}", i);

//         i += 3;
//     }
// }

// fn nilai_match() {
//     let nilai = 90;
    
//     match nilai {
//         1..=5 => println!("bodoh"),
//         6..=7 => println!("lumayan"),
//         8..=10 => println!("amajing"),
//         n if n > 10 => println!("amajing+++"),
//         _ => println!("noting!!")
//     }
// }

// fn ssss() {
//     let angka = 214748364;

//     match angka {
//         kecil @ 0..=7 => println!("kecil ({})", kecil),
//         sedang @ 8..=10 => println!("amajing ({})", sedang),
//         besar @ 11..=i32::MAX => println!("good ({})", besar),
//         _ => println!("nilai tak terduga"),
//     }
// }

// // tuple 

// fn alia() {

//     let profile = ("aligator", 222, 321);

//     println!("nama : {}", profile.0);
//     println!("umur : {}", profile.1);
//     println!("tinggi : {}", profile.2);
// }

// fn koordinat() -> (i32, i32) {
//     (30, 50)          // ← tidak pakai `return`
// }


// //konversi detik
// use std::io;

// // --------- FUNGSI ---------
// fn konversi_detik(total: u32) -> (u32, u32, u32) {
//     let jam    = total / 3600;          // 1 jam = 3600 detik
//     let sisa   = total % 3600;
//     let menit  = sisa / 60;
//     let detik  = sisa % 60;

//     (jam, menit, detik)                 // <- tuple sebagai return value
// }

// fn hitung_nilai_mhs(){
//         // 1. Buat vector tuple siswa
//         let siswa = vec![
//             ("Ali".to_string(), 85),
//             ("Budi".to_string(), 60),
//             ("Citra".to_string(), 90),
//             ("Dina".to_string(), 45),
//             ("gugun".to_string(), 99),
//             ("gugun".to_string(), 44),
//         ];
    
//         // 2. Tampilkan semua data
//         println!("Daftar Siswa:");
//         for (nama, nilai) in &siswa {
//             println!("- {}: {}", nama, nilai);
//         }
    
//         // 3. Tampilkan yang lulus (nilai >= 70)
//         println!("\n Yang Lulus:");
//         for (nama, nilai) in &siswa {
//             if *nilai >= 70 {
//                 println!("- {}: {}", nama, nilai);
//             }
//         }

//         // 4. Tampilkan yang tidak lulus (nilai <= 70)
//         println!("\n Yang Tidak Lulus:");
//         for (nama, nilai) in &siswa {
//             if *nilai <= 70 {
//                 println!("- {}: {}", nama, nilai);
//             }
//         }
    
//         // nilai tertinggi
//         if let Some((nama_top, nilai_top)) = siswa.iter().max_by_key(|(_, n)| n) {
//             println!("\n🏅 Nilai tertinggi: {} ({})", nama_top, nilai_top);
//         }

//         // 4. Hitung rata-rata nilai
//         let total: u32 = siswa.iter().map(|(_, n)| n).sum();
//         let rata2 = total as f32 / siswa.len() as f32;
//         println!("\n Rata-rata nilai: {:.2}", rata2);
// }




// fn main() {
//     //tampilkan_profil("Rustacean", 28);
//     //println!();
//     //hitung_kelipatan(7);
//     //cocok();
//     //apalah();
//     //nilai_match();
//     //ssss();
//     ////////////////////////// konversi detik ////////////////////////////////////////////////
//     // println!("Masukkan total detik:");

//     // // baca input user
//     // let mut input = String::new();
//     // io::stdin().read_line(&mut input).unwrap();
//     // let total: u32 = input.trim().parse().expect("Harus angka positif!");

//     // // panggil fungsi dan 'bongkar' tuple
//     // let (jam, menit, detik) = konversi_detik(total);

//     // println!("{total} detik = {jam} jam {menit} menit {detik} detik");
// ////////////////////////////////////////////////////////////////////////
//     // alia();
//     // let (x, y) = koordinat();
//     // println!("x = {x}, y = {y}");

//     hitung_nilai_mhs()

// }


use std::io;

// ---------- DATA MODEL ----------
struct Siswa {
    nama:  String,
    nilai: u32,
}

enum Menu {
    Daftar,
    Tambah,
    Lulus,
    TidakLulus,
    Tertinggi,
    Keluar,
}

impl Menu {
    fn from_u8(i: u8) -> Option<Self> {
        match i {
            1 => Some(Menu::Daftar),
            2 => Some(Menu::Tambah),
            3 => Some(Menu::Lulus),
            4 => Some(Menu::TidakLulus),
            5 => Some(Menu::Tertinggi),
            0 => Some(Menu::Keluar),
            _ => None,
        }
    }
}

// ---------- UTIL INPUT ----------
fn input(prompt: &str) -> String {
    let mut buf = String::new();
    print!("{prompt}");
    let _ = std::io::Write::flush(&mut std::io::stdout()); // tampilkan prompt segera
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

// ---------- PROGRAM UTAMA ----------
fn main() {
    let mut db: Vec<Siswa> = vec![
        Siswa { nama: "Ali".into(),   nilai: 85 },
        Siswa { nama: "Budi".into(),  nilai: 60 },
        Siswa { nama: "Citra".into(), nilai: 90 },
    ];

    loop {
        println!("\n=== MENU ===
1) Daftar siswa
2) Tambah siswa
3) Siswa lulus
4) Siswa tidak lulus
5) Nilai tertinggi
0) Keluar");

        let pilih: u8 = input("Pilih: ").parse().unwrap_or(99);
        match Menu::from_u8(pilih) {
            Some(Menu::Daftar) => {
                println!("📋 Daftar Siswa:");
                for s in &db {
                    println!("- {}: {}", s.nama, s.nilai);
                }
            }
            Some(Menu::Tambah) => {
                let nama  = input("Nama  : ");
                let nilai = input("Nilai : ").parse::<u32>().expect("angka!");
                db.push(Siswa { nama, nilai });
                println!("✅ Data ditambahkan!");
            }
            Some(Menu::Lulus) => {
                for s in &db {
                    if s.nilai >= 70 {
                        println!("🎉 {} ({})", s.nama, s.nilai);
                    }
                }
            }
            Some(Menu::TidakLulus) => {
                for s in &db {
                    if s.nilai < 70 {
                        println!("😢 {} ({})", s.nama, s.nilai);
                    }
                }
            }
            Some(Menu::Tertinggi) => {
                if let Some(top) = db.iter().max_by_key(|s| s.nilai) {
                    println!("🏅 {} mendapat nilai tertinggi ({})", top.nama, top.nilai);
                }
            }
            Some(Menu::Keluar) => break,
            None => println!("Menu tidak valid!"),
        }
    }

    println!("Bye!");
}

