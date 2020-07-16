use std::io;

pub const ROPANI: f32 = 5476.0;
pub const AANA: f32 = 342.25;
pub const PAISA: f32 = 85.560360991;
pub const DAM: f32 = 21.3899565639;

struct Land {
    ropani: f32,
    aana: f32,
    paisa: f32,
    dam: f32,
}

trait Convertible {
    fn area(&self) -> f32;
}

impl Convertible for Land {
    fn area(&self) -> f32 {
        (self.ropani * (ROPANI))
            + (self.aana * (AANA))
            + (self.paisa * (PAISA))
            + (self.dam * (DAM))
    }
}

fn get_input(arrays: &mut [f32]) {
    let mut area = String::new();
    io::stdin()
        .read_line(&mut area)
        .expect("Failed to read the line");
    let mut i: usize = 0;

    for token in area.split("/") {
        let result = token.trim().parse();
        if result.is_ok() {
            arrays[i] = result.unwrap();
        }
        i += 1;
    }
    println!(
        "{}(Ropani) {}(Aana) {}(Paisa) {}(Dam)",
        &arrays[0], &arrays[1], &arrays[2], &arrays[3]
    );
}

fn main() {
    println!("Enter your Land size in formate  Ropani-Aana-Paisa-Dam here");
    let mut arrays: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
    get_input(&mut arrays);
    let land = Land {
        ropani: arrays[0],
        aana: arrays[1],
        paisa: arrays[2],
        dam: arrays[3],
    };
    println!(
        "Your Area Conversion is: {}(Ropani) {}(Aana) {}(Paisa) {}(Dam) ===> {}(Square Feet)",
        land.ropani,
        land.aana,
        land.paisa,
        land.dam,
        land.area()
    );
}
