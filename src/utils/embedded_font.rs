use rusttype::Font;


pub fn get_embedded_font() -> Font<'static> {
    const FONT_DATA: &[u8] = include_bytes!("../fonts/SpaceMono-Regular.ttf");
    Font::try_from_bytes(FONT_DATA).expect("Failed to load embedded font")
}
