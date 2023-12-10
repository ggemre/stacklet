extern crate pancurses;

use pancurses::*;
use crate::Widget;

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

fn draw(window: &Window, model: &Vec<Widget>) {
    let left_margin: i32 = 2;
    
    window.clear();
    
    for widget in model {
        match widget {
            Widget::Input { y, max_width, filter, label, placeholder, content } => {
                window.mvprintw(*y as i32, left_margin, &content);
            }
            Widget::Text { y, content } => {
                window.mvprintw(*y as i32, left_margin, &content);
            }
        }
    }
    window.refresh();
}

fn wait_for_input(window: &Window, model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let mut cursor = Cursor { x: 0, y: 0 };
    let break_condition: BreakCondition;
    let limit = model.len() - 1;
    let left_margin: usize = 2;

    if let Some(Widget::Input { content, .. }) = model.get(cursor.y) {
        cursor.x = content.len();
    }

    window.mv(cursor.y as i32, cursor.x as i32);

    loop {
        let ch = window.getch();
        match ch {
            Some(Input::Character('\u{1b}')) => {
                break_condition = BreakCondition::QUIT;
                break;
            }
            Some(Input::KeyEnter) |
            Some(Input::Character('\n')) => {
                if matches!(model.get(cursor.y), Some(Widget::Input { .. })) {
                    break_condition = BreakCondition::INPUT;
                } else {
                    break_condition = BreakCondition::SELECTION;
                }
                break;
            }
            Some(Input::KeyUp) => {
                if cursor.y > 0 {
                    window.mvprintw(cursor.y as i32, 0, " ");
                    cursor.y -= 1;

                    if let Some(Widget::Input { content, .. }) = model.get(cursor.y) {
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

                    if let Some(Widget::Input { content, .. }) = model.get(cursor.y) {
                        curs_set(1);
                        cursor.x = content.len();
                    } else {
                        curs_set(0);
                        window.mvprintw(cursor.y as i32, 0, ">");
                    }

                    // window.mv(cursor.y as i32, cursor.x as i32);
                }
            }
            Some(Input::KeyLeft) => {
                if cursor.x > 0 && matches!(model.get(cursor.y), Some(Widget::Input { .. })) {
                    cursor.x -= 1;
                }
            }
            Some(Input::KeyRight) => {
                if let Some(Widget::Input { content, .. }) = model.get(cursor.y) {
                    if cursor.x < content.len() {
                        cursor.x += 1;
                    }
                }
            }
            Some(Input::KeyBackspace) |
            Some(Input::KeyDC) |
            Some(Input::Character('\u{7f}')) => {
                if cursor.x > 0 {
                    cursor.x -= 1;
                    window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
                    window.delch();
                    
                    if let Some(widget) = model.get_mut(cursor.y) {
                        if let Widget::Input { content, .. } = widget {
                            content.remove(cursor.x);
                        }
                    }
                }
            }
            Some(Input::Character(c)) => {
                if let Some(Widget::Input { content, .. }) = model.get(cursor.y) {
                    window.mvinsch(cursor.y as i32, (cursor.x + left_margin) as i32, c as chtype);
                    cursor.x += 1;
                    // window.mv(cursor.y as i32, cursor.x as i32);

                    if let Some(widget) = model.get_mut(cursor.y) {
                        if let Widget::Input { content, .. } = widget {
                            content.insert(cursor.x - 1, c);
                        }
                    }
                }
            }
            _ => {}
        }

        window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
        window.refresh();
    }

    return (break_condition, cursor.y);
}

pub fn init(model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let window = get_window();
    window.keypad(true);
    window.nodelay(true);
    window.timeout(0);
    noecho();
    curs_set(1);

    draw(&window, &model);
    return wait_for_input(&window, model);
}

pub fn update(model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let window = get_window();
    draw(&window, &model);
    return wait_for_input(&window, model);
}

pub fn destroy() {
    endwin();
}
