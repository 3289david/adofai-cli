use anyhow::Result;
use crate::effects::themes::Theme;

pub fn run(name: String) -> Result<()> {
    if let Some(theme) = Theme::get(&name) {
        println!("Theme: {}", theme.name);
        println!("  Scanlines: {}", if theme.scanlines { "ON" } else { "OFF" });
        println!("  Bloom: {}", if theme.bloom { "ON" } else { "OFF" });
        println!("  CRT: {}", if theme.crt { "ON" } else { "OFF" });
        println!("  Vignette: {}", if theme.vignette { "ON" } else { "OFF" });
        println!();
        println!("  Use with: adofai play level.adofai --theme {}", name);
    } else {
        println!("Unknown theme: {}", name);
        println!();
        println!("Available themes:");
        for t in Theme::list() {
            println!("  - {}", t);
        }
    }

    Ok(())
}
