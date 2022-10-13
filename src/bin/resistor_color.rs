use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[derive(Debug, PartialEq, Copy, Clone, Sequence, IntEnum)]
#[repr(usize)]
enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

fn color_to_value(color: ResistorColor) -> u32 {
    ResistorColor::int_value(color) as u32
}

fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string(),
    }
}

fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}

fn main() {
    let color = ResistorColor::Blue;
    let value = 7;

    println!("Value for color {:?} is {}", color, color_to_value(color));
    println!("Color for value {} is {}", value, value_to_color_string(value));
    println!("List of colors available are {:?}", colors());
}
