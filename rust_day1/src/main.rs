/*fn main() {
    let nama = "apridan";
    let mut umur = 12;

    println!("halo nama saya {}", nama);
    println!("umur saya {}", umur);

    umur = umur + 5;

    println!("halo nama saya {}", nama);
    println!("umur saya revisi {}", umur);
}*/


fn main() {
    let suhu = 320.0;

    if suhu > 35.0 {
        println!("🔥 Panas banget!");
    } else if suhu > 25.0 {
        println!("🌤️ Hangat");
    } else {
        println!("❄️ Dingin");
    }

    let hari = 4;
    match hari {
        1 => println!("Senin"),
        2 => println!("Selasa"),
        3 => println!("Rabu"),
        4 => println!("Kamis"),
        5 => println!("Jumat"),
        6 | 7 => println!("Libur!"), // 6 dan 7
        _ => println!("Hari tidak valid"),
    }


    // perulangan for
    /*for i in 1..=5 {
        println!("ini ke{}", i);
    }

    // perulangan while
    let mut count = 0;

    while count <= 3 {
        println!("count = {}", count);
        count += 1;
    }

    // perulangan loop pakai break
    let mut x = 0;

    loop {
        println!("x = {}", x);
        x += 1;

        if x >= 3 {
            break;
        }
    }*/


    println!("For Loop:");
    for i in 1..=5 {
        println!("-> i = {}", i);
    }

    println!("\nWhile Loop:");
    let mut n = 0;
    while n < 3 {
        println!("-> n = {}", n);
        n += 1;
    }

    println!("\nLoop Tak Terbatas:");
    let mut x = 0;
    loop {
        if x > 2 {
            break;
        }
        println!("-> x = {}", x);
    }
}

