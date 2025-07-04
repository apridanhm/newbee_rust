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

fn main() {
    tampilkan_profil("Rustacean", 28);
    println!();
    hitung_kelipatan(7);
}



