// https://doc.rust-lang.ru/book/ch03-05-control-flow.html
// конвертер температур из единиц Фаренгейта в единицы Цельсия
use std::io;


fn converter(f: f32) -> f32 {
    let x = (5.0 / 9.0) as f32;
    ((f - 32.0) * x) as f32
}


fn main() {
    loop {
        println!("\nВведите температуру в единицах Фаренгейта.");

        let mut frgt = String::new();
        io::stdin()
            .read_line(&mut frgt)
            .expect("Failed to read line");

        let frgt: f32 = match frgt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let cl: f32 = converter(frgt);
        println!("Температура в по единицах Цельсия: {}.", cl)
    }
}
