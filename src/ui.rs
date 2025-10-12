pub trait UIElement {}
impl UIElement for Button {}
impl UIElement for TextField {}

pub struct Button {}
impl Button {
    pub fn press() {}
}

pub struct TextField {
    is_selected: bool,
    buffer: Vec<char>,
}
impl TextField {
    pub fn select() {}
    pub fn deselect() {}
    pub fn type_in() {}
}
