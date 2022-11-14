use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[derive(Debug, PartialEq, Copy, Clone, Sequence, IntEnum)]
#[repr(usize)]
pub enum ResistorColor {
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

pub fn color_to_value(color: ResistorColor) -> u32 {
    ResistorColor::int_value(color) as u32
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}", resistor),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn color_to_value_seven() {
        let color = ResistorColor::Blue;
        assert_eq!(color_to_value(color), 6);
    }
    
    #[test]
    pub fn value_to_color_string_yellow() {
        assert_eq!(value_to_color_string(4), "Yellow");
    }
}
