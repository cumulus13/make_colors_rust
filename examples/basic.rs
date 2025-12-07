use make_colors::*;

fn main() {
    println!("=== Basic Color Examples ===\n");

    // Simple colored text
    println!("{}", make_colors("Simple red text", "red", None));
    println!("{}", make_colors("Green text", "green", None));
    println!("{}", make_colors("Blue text", "blue", None));
    
    println!("\n=== With Background ===\n");
    
    // Text with background
    println!("{}", make_colors("White on red", "white", Some("red")));
    println!("{}", make_colors("Black on yellow", "black", Some("yellow")));
    println!("{}", make_colors("White on blue", "white", Some("blue")));
    
    println!("\n=== Using Shortcuts ===\n");
    
    // Using shortcuts
    println!("{}", make_colors("r = red", "r", None));
    println!("{}", make_colors("g = green", "g", None));
    println!("{}", make_colors("bl = blue", "bl", None));
    println!("{}", make_colors("w on r", "w", Some("r")));
    
    println!("\n=== Light Colors ===\n");
    
    // Light colors
    println!("{}", make_colors("Light red", "lightred", None));
    println!("{}", make_colors("Light green", "lightgreen", None));
    println!("{}", make_colors("Light blue", "lightblue", None));
    println!("{}", make_colors("Light yellow", "lightyellow", None));
    
    println!("\n=== Hex Colors ===\n");
    
    // Hex colors
    println!("{}", make_colors_hex("Cyan (#00FFFF)", "#00FFFF", None).unwrap());
    println!("{}", make_colors_hex("Orange (#FF8800)", "#FF8800", None).unwrap());
    println!("{}", make_colors_hex("Pink (#FF1493)", "#FF1493", None).unwrap());
    println!("{}", make_colors_hex("Purple (#9932CC)", "#9932CC", None).unwrap());
    
    // Hex with background
    println!("{}", make_colors_hex("Custom colors", "#FFFFFF", Some("#FF0000")).unwrap());
    
    println!("\n=== RGB Colors ===\n");
    
    // RGB colors
    println!("{}", make_colors_rgb("RGB Orange", (255, 165, 0), None));
    println!("{}", make_colors_rgb("RGB Teal", (0, 128, 128), None));
    println!("{}", make_colors_rgb("RGB Gold", (255, 215, 0), None));
    
    println!("\n=== With Attributes ===\n");
    
    // With attributes
    println!("{}", make_colors_with_attrs("Bold red", "red", None, &["bold"]));
    println!("{}", make_colors_with_attrs("Underlined blue", "blue", None, &["underline"]));
    println!("{}", make_colors_with_attrs("Italic green", "green", None, &["italic"]));
    println!("{}", make_colors_with_attrs("Bold + Underline", "yellow", None, &["bold", "underline"]));
    
    println!("\n=== Using ColorBuilder ===\n");
    
    // ColorBuilder examples
    println!("{}", ColorBuilder::new("Builder: Red + Bold")
        .fg("red")
        .bold()
        .build());
    
    println!("{}", ColorBuilder::new("Builder: Hex color")
        .fg_hex("#00FFFF").unwrap()
        .underline()
        .build());
    
    println!("{}", ColorBuilder::new("Builder: RGB with background")
        .fg_rgb(255, 100, 50)
        .bg_rgb(0, 0, 0)
        .bold()
        .build());
    
    println!("\n=== Using Macros ===\n");
    
    // Using macros
    println!("{}", color!("red", "Macro: Red text"));
    println!("{}", color!("white", "red", "Macro: White on red"));
    println!("{}", color_hex!("#00FFFF", "Macro: Hex cyan"));
    
    println!("\n=== Practical Examples ===\n");
    
    // Status messages
    println!("{}", make_colors("[✓] Success", "lightgreen", None));
    println!("{}", make_colors("[✗] Error", "lightred", None));
    println!("{}", make_colors("[!] Warning", "lightyellow", None));
    println!("{}", make_colors("[i] Info", "lightblue", None));
    
    // Log levels
    println!("{}", make_colors(" ERROR ", "white", Some("red")));
    println!("{}", make_colors(" WARN  ", "black", Some("yellow")));
    println!("{}", make_colors(" INFO  ", "white", Some("blue")));
    println!("{}", make_colors(" DEBUG ", "white", Some("black")));
}