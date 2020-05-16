use crate::machine::Executor;
use crate::machine::Machine;
use crate::machine::Position;
use crate::machine::PreambleData;
use crate::tfm::FontDataHelper;
use dvi::FontDef;
use std::char;
use std::collections::HashMap;

pub struct HTMLMachine {
    content: String,
    color: String,
    color_stack: Vec<String>,
    points_per_dvi_unit: Option<f64>,
    svg_depth: u8,
    paperwidth: Option<f64>,
    paperheight: Option<f64>,

    position: Position,
    position_stack: Vec<Position>,
    font: Option<FontDef>,
    fonts: HashMap<u32, FontDef>,
}

impl HTMLMachine {
    pub fn new() -> HTMLMachine {
        HTMLMachine {
            content: "".to_string(),
            color: "black".to_string(),
            color_stack: Vec::new(),
            points_per_dvi_unit: None,
            svg_depth: 0,
            paperwidth: None,
            paperheight: None,
            position: Position::empty(),
            position_stack: Vec::new(),
            font: None,
            fonts: HashMap::new(),
        }
    }
}

impl Machine for HTMLMachine {
    fn get_content(&self) -> String {
        self.content.clone()
    }
    fn get_position(&mut self) -> &mut Position {
        &mut self.position
    }
    fn put_text(&mut self, buffer: Vec<u32>, font_helper: &FontDataHelper) -> f64 {
        let mut text_width = 0;
        let mut text_height = 0;
        let mut text_depth = 0;

        let mut html_text = "".to_string();

        let font = self.font.as_ref().unwrap(); //TODO

        // TODO: better error handling
        let font_name = std::str::from_utf8(&font.filename).unwrap();
        let mut font_data = font_helper.get(font_name.to_string()).unwrap_or_else(|| {
            eprintln!("Using fallback cmb10 for {}", font_name);
            font_helper.get("cmb10".to_string()).unwrap()
        }); // Fallback if font not found

        for &c in buffer.iter() {
            let mut metrics_option = font_data.characters.get(&c);
            if metrics_option.is_none() {
                //TODO: Handle this better. Error only happens for c === 127
                eprintln!("Could not find font metric for {}", c);
                metrics_option = font_data.characters.get(&126);
            }
            if let Some(metrics) = metrics_option {
                text_width += metrics.width;
                text_height = std::cmp::max(text_height, metrics.height);
                text_depth = std::cmp::max(text_depth, metrics.depth);

                // This is ridiculous.
                if c <= 9 {
                    html_text.push_str(&format!("&#{};", 161 + c));
                } else if c >= 10 && c <= 19 {
                    html_text.push_str(&format!("&#{};", 173 + c - 10));
                } else if c == 20 {
                    html_text.push_str("&#8729;"); // O RLLY?!
                } else if c >= 21 && c <= 32 {
                    html_text.push_str(&format!("&#{};", 184 + c - 21));
                } else if c == 127 {
                    html_text.push_str("&#196;");
                } else {
                    html_text.push_str(&char::from_u32(c).unwrap().to_string());
                    //TODO?
                }
            }
        }

        // tfm is based on 1/2^16 pt units, rather than dviunit which is 10^−7 meters
        let dvi_units_per_font_unit =
            (font_data.design_size as f64) / 1048576.0 * 65536.0 / 1048576.0;
        let points_per_dvi_unit = self.points_per_dvi_unit.unwrap(); //TODO

        // TODO: remove this line?
        let top = ((self.position.v() as f64) - (text_height as f64) * dvi_units_per_font_unit)
            * points_per_dvi_unit;
        let left = (self.position.h() as f64) * points_per_dvi_unit;

        let width = (text_width as f64) * points_per_dvi_unit * dvi_units_per_font_unit;
        let height = (text_height as f64) * points_per_dvi_unit * dvi_units_per_font_unit;
        let depth = (text_depth as f64) * points_per_dvi_unit * dvi_units_per_font_unit;
        let top = (self.position.v() as f64) * points_per_dvi_unit;

        let fontsize = ((font_data.design_size as f64) / 1048576.0) * (font.scale_factor as f64)
            / (font.design_size as f64);

        if self.svg_depth == 0 {
            self.content.push_str(&format!(r#"
<span style="line-height: 0; color: {}; font-family: {}; font-size: {}pt; position: absolute; top: {}pt; left: {}pt; overflow: visible;">
<span style="margin-top: -{}pt; line-height: 0pt; height: {}pt; display: inline-block; vertical-align: baseline; ">
{}
</span>
<span style="display: inline-block; vertical-align: {}pt; height: 0pt; line-height: 0;">
</span>
</span>
"#, self.color, font_name, fontsize, top-height, left, fontsize, fontsize, html_text, height));
        } else {
            let bottom = (self.position.v() as f64) * points_per_dvi_unit;
            // No 'pt' on fontsize since those units are potentially scaled
            self.content.push_str(&format!(
                r#"
<text alignment-baseline="baseline" y="{}" x="{}" style="font-family: {};" font-size="{}">
{}
</text>"#,
                bottom, left, font_name, fontsize, html_text
            ));
        }

        return (text_width as f64) * dvi_units_per_font_unit * (font.scale_factor as f64)
            / (font.design_size as f64);
    }

    fn put_rule(&mut self, ai: i32, bi: i32) {
        let points_per_dvi_unit = self.points_per_dvi_unit.unwrap(); //TODO

        let a = (ai as f64) * points_per_dvi_unit;
        let b = (bi as f64) * points_per_dvi_unit;
        let left = self.position.h() * points_per_dvi_unit;
        let bottom = self.position.v() * points_per_dvi_unit;
        let top = bottom - a;

        self.content.push_str(&format!(r#"
<span style="background: {}; position: absolute; top: {}pt; left: {}pt; width:{}pt; height: {}pt;"></span>
"#, self.color, top, left, b, a));
    }
    fn begin_page(&mut self, _arr: [i32; 10], p: i32) {
        self.position_stack.clear();
        //self.position = Position::empty(); //TODO: Optional
    }
    fn end_page(&mut self) {
        //TODO check if position stack is empty
    }
    fn push_position(&mut self) {
        self.position_stack.push(self.position.clone());
    }
    fn pop_position(&mut self) {
        self.position = self.position_stack.pop().unwrap(); //TODO?
    }
    fn set_font(&mut self, index: u32) {
        self.font = self.fonts.get(&index).map(|f| f.clone());
    }
    fn add_font(&mut self, font: FontDef) {
        self.fonts.insert(font.number, font);
    }
    fn set_preamble_data(&mut self, data: PreambleData) {
        let magnification = data.magnification as f64;
        let numerator = data.numerator as f64;
        let denominator = data.denominator as f64;
        let dvi_unit = (magnification * numerator) / (1000.0 * denominator);

        let resolution = 300.0; // ppi
        let tfm_conv = (25400000.0 / numerator) * (denominator / 473628672.0) / 16.0;
        let conv = (numerator / 254000.0) * (resolution / denominator) * (magnification / 1000.0);

        self.points_per_dvi_unit = Some(dvi_unit * 72.27 / 100000.0 / 2.54);
    }
}

impl Executor for HTMLMachine {}
