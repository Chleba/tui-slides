use crate::enums::{ContentJson, ReturnSlideWidget, SlideContentType, SlideJson};
use color_eyre::owo_colors::colors::css::Beige;
use crossterm::terminal::size;
use ratatui::{
    layout::Rect, text::Line, widgets::{Block, Paragraph, WidgetRef}
};
use ratatui_image::{picker:: Picker, Resize, Image, StatefulImage};
use tui_big_text::BigText;

pub fn get_slide_content_string(slide: ContentJson) -> String {
    let mut content_str = String::from("");
    if let Some(cv) = slide.content {
        content_str = cv;
    }
    content_str
}

fn make_slide_paragraph<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide);
    ReturnSlideWidget::Paragraph(Paragraph::new(content))
}

fn make_slide_line<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide);
    ReturnSlideWidget::Line(Line::from(content))
}

fn make_slide_bigtext<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    let content = get_slide_content_string(slide);
    let lines: Vec<Line> = content
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

pub fn make_slide_image<'a>(slide: ContentJson) -> ReturnSlideWidget<'a> {
    // let mut rect = Rect::new(0, 0, 30, 20);
    // if let Some(r) = slide.rect {
    //     rect = r;
    // }
    let content = get_slide_content_string(slide);

    ReturnSlideWidget::Image(content)

    // let dyn_img = image::io::Reader::open(content).unwrap().decode().unwrap();
    // let mut picker = Picker::from_termios().unwrap();
    // picker.guess_protocol();
    // let img_static = picker.new_protocol(dyn_img.clone(), Rect::new(0, 0, rect.width, rect.height), Resize::Fit(None)).unwrap();
    // let img_static = picker.new_protocol(dyn_img.clone(), Rect::new(0, 0, rect.width, rect.height), Resize::Fit(None)).unwrap();
    // let img_static = picker.new_resize_protocol(dyn_img);
    // ReturnSlideWidget::Image(img_static)
    // ReturnSlideWidget::Image(dyn_img)
}

pub fn make_slide_content<'a>(slide_content: ContentJson) -> ReturnSlideWidget<'a> {
    match slide_content.type_ {
        SlideContentType::Paragraph => make_slide_paragraph(slide_content),
        SlideContentType::BigText => make_slide_bigtext(slide_content),
        SlideContentType::Line => make_slide_line(slide_content),
        SlideContentType::Image => make_slide_image(slide_content),
    }
}
