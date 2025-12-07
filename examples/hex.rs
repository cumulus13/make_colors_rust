use make_colors::*;

fn main() {
    println!("=== Testing Full Hex Color Support (24-bit True Color) ===\n");

    // Test berbagai hex colors seperti rich di Python
    let test_colors = vec![
        ("#FF0000", "Pure Red"),
        ("#00FF00", "Pure Green"),
        ("#0000FF", "Pure Blue"),
        ("#FFFF00", "Yellow"),
        ("#FF00FF", "Magenta"),
        ("#00FFFF", "Cyan"),
        ("#FFA500", "Orange"),
        ("#FF1493", "Deep Pink"),
        ("#9932CC", "Dark Orchid"),
        ("#00CED1", "Dark Turquoise"),
        ("#FF4500", "Orange Red"),
        ("#2E8B57", "Sea Green"),
        ("#4169E1", "Royal Blue"),
        ("#DC143C", "Crimson"),
        ("#FFD700", "Gold"),
        ("#ADFF2F", "Green Yellow"),
        ("#FF69B4", "Hot Pink"),
        ("#BA55D3", "Medium Orchid"),
        ("#20B2AA", "Light Sea Green"),
        ("#778899", "Light Slate Gray"),
        ("#B0C4DE", "Light Steel Blue"),
        ("#FFDAB9", "Peach Puff"),
        ("#EE82EE", "Violet"),
        ("#F0E68C", "Khaki"),
        ("#E6E6FA", "Lavender"),
        ("#FFF0F5", "Lavender Blush"),
        ("#7CFC00", "Lawn Green"),
        ("#FFFACD", "Lemon Chiffon"),
        ("#ADD8E6", "Light Blue"),
        ("#F08080", "Light Coral"),
    ];

    println!("🎨 Standard Web Colors:");
    for (hex, name) in test_colors.iter() {
        let colored = make_colors_hex(&format!("  ████ {} ({})", name, hex), hex, None)
            .unwrap_or_else(|_| format!("Error: {}", hex));
        println!("{}", colored);
    }

    println!("\n🌈 Gradient Test (Red to Blue):");
    for i in 0..=20 {
        let r = 255 - (i * 12);
        let g = 0;
        let b = i * 12;
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let colored = make_colors_hex("████", &hex, None)
            .unwrap_or_else(|_| "█".to_string());
        print!("{}", colored);
    }
    println!();

    println!("\n🎨 Rainbow Spectrum:");
    let rainbow = vec![
        ("#FF0000", "Red"),
        ("#FF7F00", "Orange"),
        ("#FFFF00", "Yellow"),
        ("#00FF00", "Green"),
        ("#0000FF", "Blue"),
        ("#4B0082", "Indigo"),
        ("#9400D3", "Violet"),
    ];
    for (hex, name) in rainbow {
        let colored = make_colors_hex(&format!("  ████████ {}", name), hex, None)
            .unwrap();
        println!("{}", colored);
    }

    println!("\n🔥 Gradient Background Test:");
    for i in 0..=10 {
        let intensity = i * 25;
        let hex_fg = "#FFFFFF";
        let hex_bg = format!("#{:02X}0000", intensity);
        let text = format!(" Step {} ", i);
        let colored = make_colors_hex(&text, hex_fg, Some(&hex_bg))
            .unwrap();
        print!("{}", colored);
    }
    println!();

    println!("\n✨ Material Design Colors:");
    let material_colors = vec![
        ("#F44336", "Red 500"),
        ("#E91E63", "Pink 500"),
        ("#9C27B0", "Purple 500"),
        ("#673AB7", "Deep Purple 500"),
        ("#3F51B5", "Indigo 500"),
        ("#2196F3", "Blue 500"),
        ("#03A9F4", "Light Blue 500"),
        ("#00BCD4", "Cyan 500"),
        ("#009688", "Teal 500"),
        ("#4CAF50", "Green 500"),
        ("#8BC34A", "Light Green 500"),
        ("#CDDC39", "Lime 500"),
        ("#FFEB3B", "Yellow 500"),
        ("#FFC107", "Amber 500"),
        ("#FF9800", "Orange 500"),
        ("#FF5722", "Deep Orange 500"),
    ];

    for (hex, name) in material_colors {
        let colored = make_colors_hex(&format!("  ████ {}", name), hex, None)
            .unwrap();
        println!("{}", colored);
    }

    println!("\n🎯 Custom Hex Colors with Background:");
    println!("{}", make_colors_hex("  White on Dark Red  ", "#FFFFFF", Some("#8B0000")).unwrap());
    println!("{}", make_colors_hex("  Black on Gold  ", "#000000", Some("#FFD700")).unwrap());
    println!("{}", make_colors_hex("  Lime on Navy  ", "#00FF00", Some("#000080")).unwrap());
    println!("{}", make_colors_hex("  Hot Pink on Black  ", "#FF69B4", Some("#000000")).unwrap());

    println!("\n💎 Grayscale Gradient:");
    for i in 0..=16 {
        let gray = i * 15;
        let hex = format!("#{:02X}{:02X}{:02X}", gray, gray, gray);
        let colored = make_colors_hex("███", &hex, None).unwrap();
        print!("{}", colored);
    }
    println!();

    println!("\n🌟 RGB Direct Test (bypassing hex):");
    println!("{}", make_colors_rgb("  RGB(255, 100, 50) - Coral", (255, 100, 50), None));
    println!("{}", make_colors_rgb("  RGB(100, 200, 255) - Sky Blue", (100, 200, 255), None));
    println!("{}", make_colors_rgb("  RGB(255, 192, 203) - Pink", (255, 192, 203), None));
    println!("{}", make_colors_rgb("  RGB(255, 215, 0) - Gold", (255, 215, 0), None));

    println!("\n✅ Conclusion: Library supports ALL 16,777,216 colors (24-bit True Color)!");
}