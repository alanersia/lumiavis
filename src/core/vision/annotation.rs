use crate::{DrawRect, Image, LumiavisError, RgbColor};

#[derive(Debug, Clone)]
pub struct Annotation {
    pub rect: DrawRect,
    pub label: String,
    pub score: Option<f32>,
    pub color: RgbColor,
    pub thickness: u32,
    pub text_color: RgbColor,
    pub label_background: Option<RgbColor>,
}

impl Annotation {
    pub fn new(
        rect: DrawRect,
        label: impl Into<String>,
        score: Option<f32>,
        color: RgbColor,
    ) -> Self {
        Self {
            rect,
            label: label.into(),
            score,
            color,
            thickness: 2,
            text_color: RgbColor::WHITE,
            label_background: Some(color),
        }
    }

    pub fn with_thickness(mut self, thickness: u32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_text_color(mut self, color: RgbColor) -> Self {
        self.text_color = color;
        self
    }

    pub fn with_label_background(mut self, color: Option<RgbColor>) -> Self {
        self.label_background = color;
        self
    }

    pub fn display_text(&self) -> String {
        match self.score {
            Some(score) => format!("{} {:.2}", self.label, score),
            None => self.label.clone(),
        }
    }
}

pub fn draw_label_box(image: &mut Image, ann: &Annotation) -> Result<(), LumiavisError> {
    image.draw_rect(ann.rect, ann.color, ann.thickness)?;

    let text = ann.display_text();

    let text_width = text.chars().count() as u32 * 6;
    let text_height = 9;

    let bg_x = ann.rect.x;
    let bg_y = ann.rect.y.saturating_sub(text_height + 2);
    let bg_w = text_width + 4;
    let bg_h = text_height + 2;

    if let Some(bg) = ann.label_background {
        image.fill_rect(DrawRect::new(bg_x, bg_y, bg_w, bg_h), bg)?;
    }

    image.draw_text(bg_x + 2, bg_y + 2, &text, ann.text_color)?;

    Ok(())
}

pub fn draw_annotations(
    image: &mut Image,
    annotations: &[Annotation],
) -> Result<(), LumiavisError> {
    for ann in annotations {
        draw_label_box(image, ann)?;
    }

    Ok(())
}

pub fn draw_fps_overlay(
    image: &mut Image,
    fps: f64,
    x: u32,
    y: u32,
    text_color: RgbColor,
    background: Option<RgbColor>,
) -> Result<(), LumiavisError> {
    let text = format!("FPS: {:.2}", fps);

    let text_width = text.chars().count() as u32 * 6;
    let text_height = 9;

    if let Some(bg) = background {
        image.fill_rect(DrawRect::new(x, y, text_width + 4, text_height + 2), bg)?;
    }

    image.draw_text(x + 2, y + 2, &text, text_color)?;

    Ok(())
}
