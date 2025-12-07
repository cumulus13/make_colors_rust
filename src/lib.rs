// File: src\lib.rs
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// Date: 2025-12-08
// Description: A simple, powerful and cross-platform Rust library for adding colors to your terminal output.
// License: MIT

//! # make_colors
//!
//! A simple, powerful and cross-platform Rust library for adding colors to your terminal output.
//! 
//! ## Features
//! 
//! - 🎨 Support for standard terminal colors (16 colors)
//! - 🌈 Hex color support (#00FFFF)
//! - 🎯 RGB color support (0-255 for each channel)
//! - 📝 Simple and intuitive API
//! - 🖋 Attributes support (bold, underline, italic, etc.)
//! - 🔧 Flexible formatting with multiple notations
//!
//! ## Quick Start
//!
//! ```rust
//! use make_colors::*;
//!
//! // Simple colored text
//! println!("{}", make_colors("Hello World!", "red", None));
//!
//! // With background
//! println!("{}", make_colors("Important", "white", Some("red")));
//!
//! // Using hex colors
//! println!("{}", make_colors_hex("Cyan text", "#00FFFF", None));
//!
//! // Using RGB
//! println!("{}", make_colors_rgb("Custom color", (255, 100, 50), None));
//! ```

use std::fmt;

/// ANSI color codes for standard colors
pub mod ansi {
    // Foreground colors
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    // Light colors
    pub const LIGHT_BLACK: &str = "\x1b[90m";
    pub const LIGHT_RED: &str = "\x1b[91m";
    pub const LIGHT_GREEN: &str = "\x1b[92m";
    pub const LIGHT_YELLOW: &str = "\x1b[93m";
    pub const LIGHT_BLUE: &str = "\x1b[94m";
    pub const LIGHT_MAGENTA: &str = "\x1b[95m";
    pub const LIGHT_CYAN: &str = "\x1b[96m";
    pub const LIGHT_WHITE: &str = "\x1b[97m";
    
    // Background colors
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
    
    // Light background colors
    pub const BG_LIGHT_BLACK: &str = "\x1b[100m";
    pub const BG_LIGHT_RED: &str = "\x1b[101m";
    pub const BG_LIGHT_GREEN: &str = "\x1b[102m";
    pub const BG_LIGHT_YELLOW: &str = "\x1b[103m";
    pub const BG_LIGHT_BLUE: &str = "\x1b[104m";
    pub const BG_LIGHT_MAGENTA: &str = "\x1b[105m";
    pub const BG_LIGHT_CYAN: &str = "\x1b[106m";
    pub const BG_LIGHT_WHITE: &str = "\x1b[107m";
    
    // Reset and attributes
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    pub const BLINK: &str = "\x1b[5m";
    pub const REVERSE: &str = "\x1b[7m";
    pub const HIDDEN: &str = "\x1b[8m";
    pub const STRIKETHROUGH: &str = "\x1b[9m";
}

/// Errors that can occur when using make_colors
#[derive(Debug, Clone)]
pub enum MakeColorsError {
    InvalidHexColor(String),
    InvalidColorName(String),
}

impl fmt::Display for MakeColorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MakeColorsError::InvalidHexColor(hex) => write!(f, "Invalid hex color: {}", hex),
            MakeColorsError::InvalidColorName(name) => write!(f, "Invalid color name: {}", name),
        }
    }
}

impl std::error::Error for MakeColorsError {}

/// Convert hex color string to RGB tuple
/// 
/// # Examples
/// ```
/// use make_colors::hex_to_rgb;
/// 
/// let (r, g, b) = hex_to_rgb("#00FFFF").unwrap();
/// assert_eq!((r, g, b), (0, 255, 255));
/// ```
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), MakeColorsError> {
    let hex = hex.trim_start_matches('#');
    
    if hex.len() != 6 {
        return Err(MakeColorsError::InvalidHexColor(hex.to_string()));
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| MakeColorsError::InvalidHexColor(hex.to_string()))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| MakeColorsError::InvalidHexColor(hex.to_string()))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| MakeColorsError::InvalidHexColor(hex.to_string()))?;
    
    Ok((r, g, b))
}

/// Map color name or abbreviation to ANSI code
fn get_color_code(color: &str, background: bool) -> Option<String> {
    let color = color.to_lowercase();
    let color = color.trim();
    
    // Handle "light" prefix
    let is_light = color.starts_with("light");
    let base_color = if is_light {
        &color[5..]
    } else {
        &color
    };
    
    // Map abbreviations and full names
    let normalized = match base_color {
        "b" | "bk" | "black" => "black",
        "r" | "rd" | "re" | "red" => "red",
        "g" | "gr" | "ge" | "green" => "green",
        "y" | "ye" | "yl" | "yellow" => "yellow",
        "bl" | "blue" => "blue",
        "m" | "mg" | "ma" | "magenta" => "magenta",
        "c" | "cy" | "cn" | "cyan" => "cyan",
        "w" | "wh" | "wi" | "wt" | "white" => "white",
        _ => return None,
    };
    
    // Get the appropriate ANSI code
    let code = match (normalized, is_light, background) {
        ("black", false, false) => ansi::BLACK,
        ("red", false, false) => ansi::RED,
        ("green", false, false) => ansi::GREEN,
        ("yellow", false, false) => ansi::YELLOW,
        ("blue", false, false) => ansi::BLUE,
        ("magenta", false, false) => ansi::MAGENTA,
        ("cyan", false, false) => ansi::CYAN,
        ("white", false, false) => ansi::WHITE,
        
        ("black", true, false) => ansi::LIGHT_BLACK,
        ("red", true, false) => ansi::LIGHT_RED,
        ("green", true, false) => ansi::LIGHT_GREEN,
        ("yellow", true, false) => ansi::LIGHT_YELLOW,
        ("blue", true, false) => ansi::LIGHT_BLUE,
        ("magenta", true, false) => ansi::LIGHT_MAGENTA,
        ("cyan", true, false) => ansi::LIGHT_CYAN,
        ("white", true, false) => ansi::LIGHT_WHITE,
        
        ("black", false, true) => ansi::BG_BLACK,
        ("red", false, true) => ansi::BG_RED,
        ("green", false, true) => ansi::BG_GREEN,
        ("yellow", false, true) => ansi::BG_YELLOW,
        ("blue", false, true) => ansi::BG_BLUE,
        ("magenta", false, true) => ansi::BG_MAGENTA,
        ("cyan", false, true) => ansi::BG_CYAN,
        ("white", false, true) => ansi::BG_WHITE,
        
        ("black", true, true) => ansi::BG_LIGHT_BLACK,
        ("red", true, true) => ansi::BG_LIGHT_RED,
        ("green", true, true) => ansi::BG_LIGHT_GREEN,
        ("yellow", true, true) => ansi::BG_LIGHT_YELLOW,
        ("blue", true, true) => ansi::BG_LIGHT_BLUE,
        ("magenta", true, true) => ansi::BG_LIGHT_MAGENTA,
        ("cyan", true, true) => ansi::BG_LIGHT_CYAN,
        ("white", true, true) => ansi::BG_LIGHT_WHITE,
        
        // Catch-all for unknown colors
        _ => return None,
    };
    
    Some(code.to_string())
}

/// Get attribute ANSI code
fn get_attribute_code(attr: &str) -> Option<&'static str> {
    match attr.to_lowercase().as_str() {
        "bold" => Some(ansi::BOLD),
        "dim" => Some(ansi::DIM),
        "italic" => Some(ansi::ITALIC),
        "underline" => Some(ansi::UNDERLINE),
        "blink" => Some(ansi::BLINK),
        "reverse" => Some(ansi::REVERSE),
        "hidden" => Some(ansi::HIDDEN),
        "strikethrough" => Some(ansi::STRIKETHROUGH),
        _ => None,
    }
}

/// Main function to colorize text with named colors
/// 
/// # Examples
/// ```
/// use make_colors::make_colors;
/// 
/// // Simple usage
/// let colored = make_colors("Hello", "red", None);
/// println!("{}", colored);
/// 
/// // With background
/// let colored = make_colors("Error", "white", Some("red"));
/// println!("{}", colored);
/// 
/// // With attributes
/// let colored = make_colors_with_attrs("Bold text", "green", None, &["bold"]);
/// println!("{}", colored);
/// ```
pub fn make_colors(text: &str, fg: &str, bg: Option<&str>) -> String {
    make_colors_with_attrs(text, fg, bg, &[])
}

/// Colorize text with attributes
pub fn make_colors_with_attrs(text: &str, fg: &str, bg: Option<&str>, attrs: &[&str]) -> String {
    let mut result = String::new();
    
    // Add attributes
    for attr in attrs {
        if let Some(code) = get_attribute_code(attr) {
            result.push_str(code);
        }
    }
    
    // Add foreground color
    if let Some(fg_code) = get_color_code(fg, false) {
        result.push_str(&fg_code);
    }
    
    // Add background color
    if let Some(bg_color) = bg {
        if let Some(bg_code) = get_color_code(bg_color, true) {
            result.push_str(&bg_code);
        }
    }
    
    // Add text and reset
    result.push_str(text);
    result.push_str(ansi::RESET);
    
    result
}

/// Colorize text using hex color codes
/// 
/// # Examples
/// ```
/// use make_colors::make_colors_hex;
/// 
/// // Cyan text
/// let colored = make_colors_hex("Cyan text", "#00FFFF", None);
/// println!("{}", colored);
/// 
/// // With background
/// let colored = make_colors_hex("Custom", "#FF5500", Some("#001122"));
/// println!("{}", colored);
/// ```
pub fn make_colors_hex(text: &str, fg_hex: &str, bg_hex: Option<&str>) -> Result<String, MakeColorsError> {
    make_colors_hex_with_attrs(text, fg_hex, bg_hex, &[])
}

/// Colorize text using hex colors with attributes
pub fn make_colors_hex_with_attrs(
    text: &str, 
    fg_hex: &str, 
    bg_hex: Option<&str>,
    attrs: &[&str]
) -> Result<String, MakeColorsError> {
    let (r, g, b) = hex_to_rgb(fg_hex)?;
    
    let bg_rgb = if let Some(bg) = bg_hex {
        Some(hex_to_rgb(bg)?)
    } else {
        None
    };
    
    Ok(make_colors_rgb_with_attrs(text, (r, g, b), bg_rgb, attrs))
}

/// Colorize text using RGB values
/// 
/// # Examples
/// ```
/// use make_colors::make_colors_rgb;
/// 
/// // Custom RGB color
/// let colored = make_colors_rgb("Orange text", (255, 165, 0), None);
/// println!("{}", colored);
/// ```
pub fn make_colors_rgb(text: &str, fg_rgb: (u8, u8, u8), bg_rgb: Option<(u8, u8, u8)>) -> String {
    make_colors_rgb_with_attrs(text, fg_rgb, bg_rgb, &[])
}

/// Colorize text using RGB values with attributes
pub fn make_colors_rgb_with_attrs(
    text: &str,
    fg_rgb: (u8, u8, u8),
    bg_rgb: Option<(u8, u8, u8)>,
    attrs: &[&str]
) -> String {
    let mut result = String::new();
    
    // Add attributes
    for attr in attrs {
        if let Some(code) = get_attribute_code(attr) {
            result.push_str(code);
        }
    }
    
    // Add foreground RGB
    result.push_str(&format!("\x1b[38;2;{};{};{}m", fg_rgb.0, fg_rgb.1, fg_rgb.2));
    
    // Add background RGB if provided
    if let Some((r, g, b)) = bg_rgb {
        result.push_str(&format!("\x1b[48;2;{};{};{}m", r, g, b));
    }
    
    // Add text and reset
    result.push_str(text);
    result.push_str(ansi::RESET);
    
    result
}

/// Builder pattern for creating colored text
/// 
/// # Examples
/// ```
/// use make_colors::ColorBuilder;
/// 
/// let colored = ColorBuilder::new("Hello World")
///     .fg("red")
///     .bg("white")
///     .bold()
///     .build();
/// println!("{}", colored);
/// 
/// let hex_colored = ColorBuilder::new("Hex Colors")
///     .fg_hex("#00FFFF").unwrap()
///     .underline()
///     .build();
/// println!("{}", hex_colored);
/// ```
pub struct ColorBuilder {
    text: String,
    fg: Option<String>,
    bg: Option<String>,
    fg_rgb: Option<(u8, u8, u8)>,
    bg_rgb: Option<(u8, u8, u8)>,
    attrs: Vec<String>,
}

impl ColorBuilder {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            fg: None,
            bg: None,
            fg_rgb: None,
            bg_rgb: None,
            attrs: Vec::new(),
        }
    }
    
    pub fn fg(mut self, color: &str) -> Self {
        self.fg = Some(color.to_string());
        self
    }
    
    pub fn bg(mut self, color: &str) -> Self {
        self.bg = Some(color.to_string());
        self
    }
    
    pub fn fg_hex(mut self, hex: &str) -> Result<Self, MakeColorsError> {
        self.fg_rgb = Some(hex_to_rgb(hex)?);
        Ok(self)
    }
    
    pub fn bg_hex(mut self, hex: &str) -> Result<Self, MakeColorsError> {
        self.bg_rgb = Some(hex_to_rgb(hex)?);
        Ok(self)
    }
    
    pub fn fg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.fg_rgb = Some((r, g, b));
        self
    }
    
    pub fn bg_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.bg_rgb = Some((r, g, b));
        self
    }
    
    pub fn bold(mut self) -> Self {
        self.attrs.push("bold".to_string());
        self
    }
    
    pub fn italic(mut self) -> Self {
        self.attrs.push("italic".to_string());
        self
    }
    
    pub fn underline(mut self) -> Self {
        self.attrs.push("underline".to_string());
        self
    }
    
    pub fn dim(mut self) -> Self {
        self.attrs.push("dim".to_string());
        self
    }
    
    pub fn blink(mut self) -> Self {
        self.attrs.push("blink".to_string());
        self
    }
    
    pub fn reverse(mut self) -> Self {
        self.attrs.push("reverse".to_string());
        self
    }
    
    pub fn attr(mut self, attr: &str) -> Self {
        self.attrs.push(attr.to_string());
        self
    }
    
    pub fn build(self) -> String {
        // Prioritize RGB colors over named colors
        if let Some(fg_rgb) = self.fg_rgb {
            let attrs_refs: Vec<&str> = self.attrs.iter().map(|s| s.as_str()).collect();
            make_colors_rgb_with_attrs(&self.text, fg_rgb, self.bg_rgb, &attrs_refs)
        } else if let Some(fg) = self.fg {
            let bg_ref = self.bg.as_deref();
            let attrs_refs: Vec<&str> = self.attrs.iter().map(|s| s.as_str()).collect();
            make_colors_with_attrs(&self.text, &fg, bg_ref, &attrs_refs)
        } else {
            // No colors specified, just return text with attributes
            let mut result = String::new();
            for attr in &self.attrs {
                if let Some(code) = get_attribute_code(attr) {
                    result.push_str(code);
                }
            }
            result.push_str(&self.text);
            result.push_str(ansi::RESET);
            result
        }
    }
}

// Convenience macros similar to Python's make_colors shortcuts
/// Macro to create colored text easily
/// 
/// # Examples
/// ```
/// use make_colors::color;
/// 
/// println!("{}", color!("red", "Error message"));
/// println!("{}", color!("white", "red", "Important"));
/// ```
#[macro_export]
macro_rules! color {
    ($fg:expr, $text:expr) => {
        $crate::make_colors($text, $fg, None)
    };
    ($fg:expr, $bg:expr, $text:expr) => {
        $crate::make_colors($text, $fg, Some($bg))
    };
}

/// Macro for hex colors
#[macro_export]
macro_rules! color_hex {
    ($fg:expr, $text:expr) => {
        $crate::make_colors_hex($text, $fg, None).unwrap_or_else(|_| $text.to_string())
    };
    ($fg:expr, $bg:expr, $text:expr) => {
        $crate::make_colors_hex($text, $fg, Some($bg)).unwrap_or_else(|_| $text.to_string())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#00FFFF").unwrap(), (0, 255, 255));
        assert_eq!(hex_to_rgb("#FF0000").unwrap(), (255, 0, 0));
        assert_eq!(hex_to_rgb("00FF00").unwrap(), (0, 255, 0));
    }

    #[test]
    fn test_make_colors() {
        let result = make_colors("Test", "red", None);
        assert!(result.contains("Test"));
        assert!(result.contains("\x1b["));
    }

    #[test]
    fn test_make_colors_hex() {
        let result = make_colors_hex("Test", "#00FFFF", None).unwrap();
        assert!(result.contains("Test"));
        assert!(result.contains("\x1b[38;2;"));
    }

    #[test]
    fn test_color_builder() {
        let result = ColorBuilder::new("Test")
            .fg("red")
            .bold()
            .build();
        assert!(result.contains("Test"));
    }
}