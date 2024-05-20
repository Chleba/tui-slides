use crate::enums::{ReturnSlideWidget, SlideContentType, SlideJson};
use color_eyre::owo_colors::colors::css::Beige;
use ratatui::{
    text::Line,
    widgets::{Block, Paragraph, WidgetRef},
};
use tui_big_text::BigText;

fn make_slide_paragraph<'a>(slide: SlideJson) -> ReturnSlideWidget<'a> {
    ReturnSlideWidget::Paragraph(Paragraph::new(slide.content.content))
}

fn make_slide_bigtext<'a>(slide: SlideJson) -> ReturnSlideWidget<'a> {
    let lines: Vec<Line> = slide
        .content
        .content
        .split('\n')
        .map(|s| Line::from(s.to_string()))
        .collect();
    ReturnSlideWidget::BigText(
        BigText::builder()
            .pixel_size(tui_big_text::PixelSize::Sextant)
            .lines(lines)
            .alignment(ratatui::layout::Alignment::Center)
            .build()
            .unwrap(),
    )
}

pub fn make_slide_content<'a>(slide: SlideJson) -> ReturnSlideWidget<'a> {
    match slide.content.type_ {
        SlideContentType::Paragraph => make_slide_paragraph(slide),
        SlideContentType::BigText => make_slide_bigtext(slide),
        // SlideContentType::Image => {}
        // _ => ReturnSlideWidget::Paragraph(Paragraph::new("__text__")),
    }
}
