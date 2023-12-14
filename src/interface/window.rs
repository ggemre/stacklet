extern crate pancurses;

use pancurses::*;
use crate::external::widget::{Filter, Widget};
use crate::utils::fuzzy::fuzzy_match;
use crate::utils::helpers::{find_widget_by_y, find_widget_by_y_mut};

#[derive(Debug, PartialEq)]
pub enum BreakCondition {
    QUIT,
    SELECTION,
    INPUT,
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}

static mut WINDOW: Option<Window> = None;

fn get_window() -> &'static mut Window {
    unsafe {
        WINDOW.get_or_insert_with(|| initscr())
    }
}

static mut SCROLL_OFFSET: i32 = 0;

fn get_scroll_offset() -> &'static i32 {
    unsafe {
        &SCROLL_OFFSET
    }
}

fn reset_scroll_offset() {
    unsafe {
        SCROLL_OFFSET = 0;
    }
}

fn scroll(inc: i32) {
    unsafe {
        SCROLL_OFFSET += inc;
    }
}

fn draw(window: &Window, model: &mut Vec<Widget>) {
    let left_margin: i32 = 2;
    let mut current_level: i32 = *get_scroll_offset();

    window.clear();
    for widget in model {
        match widget {
            Widget::Input { y, content, .. } => {
                window.mvprintw(current_level, left_margin as i32, &content);
                *y = current_level;
            }
            Widget::Text { y, content, show, .. } => {
                if *show {
                    window.mvprintw(current_level, left_margin as i32, &content);
                    *y = current_level;
                } else {
                    *y = -1;
                    current_level -= 1; // TODO: temp fix for now
                }
            }
        }
        current_level += 1;
    }
}

fn filter_widgets(model: &mut Vec<Widget>, filter: Filter, content: &str) {
    for widget in model {
        match widget {
            Widget::Input { .. } => {}
            Widget::Text { content: widget_content, show, .. } => {
                let content_lower = content.to_lowercase();
                let widget_content_lower = widget_content.to_lowercase();

                if filter == Filter::Off || content_lower.is_empty() {
                    *show = true;
                    continue;
                } else if filter == Filter::Exact {
                    if *show && !widget_content_lower.contains(&content_lower) {
                        *show = false;
                    } else if !*show && widget_content_lower.contains(&content_lower) {
                        *show = true;
                    }
                    continue;
                } else if filter == Filter::Fuzzy {
                    if *show && !fuzzy_match(&content_lower, &widget_content_lower) && !widget_content_lower.contains(&content_lower) {
                        *show = false;
                    } else if !*show && (fuzzy_match(&content_lower, &widget_content_lower) || widget_content_lower.contains(&content_lower)) {
                        *show = true;
                    }
                    continue;
                }
            }
        }
    }
}

fn wait_for_input(window: &Window, model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let mut cursor = Cursor { x: 0, y: 0 };
    reset_scroll_offset();
    let mut break_condition: BreakCondition;
    let mut limit = model.len() - 1;
    let left_margin: usize = 2;
    let mut current_widget: usize = 0;
    let height = window.get_max_y() as usize;

    if let Some(Widget::Input { content, .. }) = find_widget_by_y(model, cursor.y as i32) {
        cursor.x = content.len();
    }

    window.mv(cursor.y as i32, cursor.x as i32);

    draw(window, model);

    loop {
        let ch = window.getch();
        match ch {
            Some(Input::Character('\u{1b}')) => {
                break_condition = BreakCondition::QUIT;
                break;
            }
            Some(Input::KeyEnter) |
            Some(Input::Character('\n')) => {
                // set current_widget to id of widget at cursor.y
                // if current_widget is input, break_condition = BreakCondition::INPUT
                // else break_condition = BreakCondition::SELECTION

                break_condition = BreakCondition::QUIT;
                for widget in model {
                    match widget {
                        Widget::Input { y, id, .. } if *y == (cursor.y as i32) => {
                            current_widget = *id;
                            break_condition = BreakCondition::INPUT;
                            break;
                        }
                        Widget::Text { y, id, .. } if *y == (cursor.y as i32) => {
                            current_widget = *id;
                            break_condition = BreakCondition::SELECTION;
                            break;
                        }
                        _ => {
                            break_condition = BreakCondition::QUIT;
                        },
                    }
                }

                break;
            }
            Some(Input::KeyUp) => {
                if *get_scroll_offset() < 0 && cursor.y == 0 {
                    scroll(1);
                    draw(window, model);
                    cursor.y = 1;
                    limit += 1;
                }

                if cursor.y > 0 {
                    window.mvprintw(cursor.y as i32, 0, " ");
                    cursor.y -= 1;

                    if let Some(Widget::Input { content, .. }) = find_widget_by_y(model, cursor.y as i32) {
                        curs_set(1);
                        cursor.x = content.len();
                    } else {
                        curs_set(0);
                        window.mvprintw(cursor.y as i32, 0, ">");
                    }
                }
            }
            Some(Input::KeyDown) => {
                if cursor.y < limit {
                    window.mvprintw(cursor.y as i32, 0, " ");
                    cursor.y += 1;

                    if cursor.y > height - 1 {
                        scroll(-1);
                        draw(window, model);
                        cursor.y -= 1;
                        limit -= 1;
                    }

                    if let Some(Widget::Input { content, .. }) = find_widget_by_y(model, cursor.y as i32) {
                        curs_set(1);
                        cursor.x = content.len();
                    } else {
                        curs_set(0);
                        window.mvprintw(cursor.y as i32, 0, ">");
                    }
                }
            }
            Some(Input::KeyLeft) => {
                if cursor.x > 0 && matches!(find_widget_by_y(model, cursor.y as i32), Some(Widget::Input { .. })) {
                    cursor.x -= 1;
                }
            }
            Some(Input::KeyRight) => {
                if let Some(Widget::Input { content, .. }) = find_widget_by_y(model, cursor.y as i32) {
                    if cursor.x < content.len() {
                        cursor.x += 1;
                    }
                }
            }
            Some(Input::KeyBackspace) |
            Some(Input::KeyDC) |
            Some(Input::Character('\u{7f}')) => {
                let mut content;
                let filter;
                // let current_level = cursor.y;

                if let Some(Widget::Input { content: ref_content, filter: ref_filter, .. }) = find_widget_by_y(model, cursor.y as i32).cloned() {
                    content = ref_content.clone();
                    filter = ref_filter.clone();
                } else {
                    continue;
                }

                if cursor.x > 0 {
                    cursor.x -= 1;
                    window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
                    window.delch();
                    
                    if let Some(widget) = find_widget_by_y_mut(model, cursor.y as i32) {
                        if let Widget::Input { content: widget_content, .. } = widget {
                            widget_content.remove(cursor.x);
                        }
                    }

                    content.remove(cursor.x);
                    filter_widgets(model, filter, &content);
                    draw(window, model);
                }
            }
            Some(Input::Character(c)) => {
                let mut content;
                let filter;
                let current_level = cursor.y;
                
                if let Some(Widget::Input { content: ref_content, filter: ref_filter, .. }) = find_widget_by_y(model, cursor.y as i32).cloned() {
                    content = ref_content.clone();
                    filter = ref_filter.clone();
                } else {
                    continue;
                }

                window.mvinsch(cursor.y as i32, (cursor.x + left_margin) as i32, c as chtype);
                cursor.x += 1;

                if let Some(widget) = find_widget_by_y_mut(model, current_level as i32) {
                    if let Widget::Input { content: widget_content, .. } = widget {
                        widget_content.insert(cursor.x - 1, c);
                    }
                }

                content.insert(cursor.x - 1, c);
                filter_widgets(model, filter, &content);
                draw(window, model);
                
            }
            _ => {}
        }

        window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
        window.refresh();
    }

    return (break_condition, current_widget);
}

pub fn init(model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let window = get_window();
    window.keypad(true);
    window.nodelay(true);
    window.timeout(0);
    noecho();
    curs_set(1);

    return wait_for_input(&window, model);
}

pub fn destroy() {
    endwin();
}
