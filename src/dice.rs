use std::fmt::Display;

use rand::Rng;
#[derive(Clone, Copy)]
pub struct Dice {
    value: u8
}
impl Dice {
    pub fn roll(&mut self) {
        self.value = rand::thread_rng().gen_range(1..=6);
    }
    pub fn get_value(self) -> u8 {
        self.value
    }
    pub fn set_value(mut self, val: u8) -> bool {
        if (1..=6).contains(&val) {
            self.value = val;
            true
        } else {
            false
        }
    }
}
impl Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", match self.value {
            1 => '\u{2680}',
            2 => '\u{2681}',
            3 => '\u{2682}',
            4 => '\u{2683}',
            5 => '\u{2684}',
            6 => '\u{2685}',
            _ => panic!(),
        })
    }
}
impl Default for Dice {
    fn default() -> Self {
        let mut new_val = Self { value: 1 };
        new_val.roll();
        new_val

    }
}
impl From<Dice> for u8 {
    fn from(value: Dice) -> Self {
        value.value
    }
}