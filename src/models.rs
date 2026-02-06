use ansi_term::Style;

pub struct Card {
    pub representation: String,
    pub output_style: Style,
    pub value: u32,
}