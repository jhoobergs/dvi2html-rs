pub fn tex_color_to_hex(color: &str) -> String {
    match color {
        "gray 0" => "black".to_string(),
        "gray 1" => "white".to_string(),
        _ if color.starts_with("rgb ") => format!(
            "#{}",
            color
                .split_whitespace()
                .filter(|&i| i != "rgb")
                .map(|v| format!("{:0>2x}", (v.parse::<f64>().unwrap() * 255.0) as u8)) //TODO? unwrap?
                .collect::<Vec<String>>()
                .join("")
        ),
        _ if color.starts_with("gray ") => {
            let v = color.split_ascii_whitespace().collect::<Vec<&str>>();
            let r = (v.get(1).unwrap().parse::<f64>().unwrap() * 255.0) as u8;
            format!("#{:0>2x}{:0>2x}{:0>2x}", r, r, r)
        }
        _ => "black".to_string(),
    }
}
