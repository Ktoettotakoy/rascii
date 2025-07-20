use rusttype::Font;
use figlet_rs::FIGfont;

const LARRY_FONT_DATA: &str = include_str!("../fonts/larry3d.flf");
const SPACE_MONO_FONT_DATA: &[u8] = include_bytes!("../fonts/SpaceMono-Regular.ttf");


pub fn get_embedded_font() -> Font<'static> {
    Font::try_from_bytes(SPACE_MONO_FONT_DATA).expect("Failed to load embedded font")
}

pub fn get_larry3d_font() -> FIGfont {
    FIGfont::from_content(LARRY_FONT_DATA).expect("Failed to load embedded larry3d font")
}
