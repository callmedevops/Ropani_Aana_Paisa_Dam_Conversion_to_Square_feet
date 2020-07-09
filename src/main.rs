use std::io;

pub const PI: f32 = 3.1415926535897932384;
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

fn _extract_values_area(_area: String) -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn get_input(message: &String) -> f32 {
    loop {
        println!("{}", message);

        let mut area = String::new();
        io::stdin()
            .read_line(&mut area)
            .expect("Failed to read the line");
    }
}

fn main() {
    let instruction = String::from("Enter your Land size in formate  Ropani-Aana-Paisa-Dam here");
    let land = Land {
        ropani: get_input(&instruction),
        aana: get_input(&instruction),
        paisa: get_input(&instruction),
        dam: get_input(&instruction),
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
