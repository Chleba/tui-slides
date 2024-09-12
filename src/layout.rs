use ratatui::{prelude::*, widgets::*};

const TITLE_HEIGHT: u16 = 2;
const MIN_CONTENT_HEIGHT: u16 = 20;

pub const CONTENT_WIDTH: u16 = 50;
pub const CONTENT_HEIGHT: u16 = 30;

const VERTICAL_CONSTRAINS: [Constraint; 2] = [
    Constraint::Length(TITLE_HEIGHT),
    Constraint::Min(MIN_CONTENT_HEIGHT),
];

pub struct SlidesLayout {
    pub title: Rect,
    pub slides: Rect,
    pub content: Rect,
}

fn get_centered_rect_percent(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

fn get_centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x_axis =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let y_axis =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    Rect {
        x: x_axis[1].x - (width / 2),
        y: y_axis[1].y - ((height / 2) + 1),
        width,
        height,
    }
}

pub fn get_title_layout(area: Rect) -> Rect {
    let layout = Layout::vertical(VERTICAL_CONSTRAINS).split(area);
    layout[0]
}

pub fn get_slides_layout(area: Rect, box_width: u16, box_height: u16) -> SlidesLayout {
    let layout = Layout::vertical(VERTICAL_CONSTRAINS).split(area);
    // let center_rect = get_centered_rect_percent(CONTENT_PERCENT_WIDTH, CONTENT_PERCENT_HEIGHT, layout[1]);
    let center_rect = get_centered_rect(box_width, box_height, layout[1]);

    SlidesLayout {
        title: layout[0],
        slides: layout[1],
        content: center_rect,
    }
}
