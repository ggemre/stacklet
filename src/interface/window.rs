extern crate pancurses;

use pancurses::*;
use crate::external::widget::{Filter, Widget};
use crate::utils::fuzzy::{exact_match, fuzzy_match};
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

// holds current ncurses window
static mut WINDOW: Option<Window> = None;

/// Return the ncurses window if it exists, or initialize it.
///
/// Operation is unsafe for 2 reasons:
/// * accessing shared mutable state, (this will break if `get_window` is called asynchronously), 
///   which it is not
/// * foreign function interface, ("I'm responsible for ensuring the safety when interacting with this external C code")
fn get_window() -> &'static mut Window {
    unsafe {
        WINDOW.get_or_insert_with(|| initscr())
    }
}

// holds vertical offset of aforementioned window
static mut SCROLL_OFFSET: i32 = 0;

/// Return the current scroll offset.
///
/// Operation is unsafe because it accesses shared mutable state, (so `get_scroll_offset` must only be called synchronously)
fn get_scroll_offset() -> &'static i32 {
    unsafe {
        &SCROLL_OFFSET
    }
}

/// Set the scroll offset shared variable to 0.
fn reset_scroll_offset() {
    unsafe {
        SCROLL_OFFSET = 0;
    }
}

/// Add a positive or negative increment to the shared scroll offset variable.
fn scroll(inc: i32) {
    unsafe {
        SCROLL_OFFSET += inc;
    }
}

/// Given the current window and the ui model, draw all widgets to the window.
///
/// Clear the window, and iterate through model, (a vector of widgets) to draw them.
fn draw(window: &Window, model: &mut Vec<Widget>) {
    let left_margin: i32 = 2;
    let mut current_level: i32 = *get_scroll_offset();

    window.clear();
    for widget in model {
        match widget {
            Widget::Input { y, content, label, .. } => {
                // input widget found, write it, its label and content, (both default to "")
                window.mvprintw(current_level, left_margin as i32, &format!("{}{}", label, content));
                *y = current_level;
            }
            Widget::Text { y, content, show, .. } => {
                // text widget found, write its content if its show property is `true`
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

/// Filters widgets in the given model according to a string.
///
/// Unmatched text widgets have show property set to `false`,
/// matched text widgets have show property set to `true`.
/// Matching works as follows:
/// - **Filter is off**: set show to `true` for all widgets
/// - **Filter is Exact**: set show to `true` if the content contains the search string
/// - **Filter is Fuzzy**: set show to `true` according to Levenshtein distance
fn filter_widgets(model: &mut Vec<Widget>, filter: Filter, content: &str) {
    // iterate through all widgets of given model
    for widget in model {
        match widget {
            Widget::Input { .. } => {}
            Widget::Text { content: widget_content, show, .. } => {
                // widget is text, attempt to filter it
                let content_lower = content.to_lowercase();
                let widget_content_lower = widget_content.to_lowercase();

                if filter == Filter::Off || content_lower.is_empty() {
                    // no filter, make sure all widgets are shown
                    *show = true;
                    continue;
                } else if filter == Filter::Exact {
                    // filter is exact, hide/show widgets accordingly
                    if *show && !exact_match(&widget_content_lower, &content_lower) {
                        *show = false;
                    } else if !*show && exact_match(&widget_content_lower, &content_lower) {
                        *show = true;
                    }
                    continue;
                } else if filter == Filter::Fuzzy {
                    // filter is fuzzy, hide/show widgets accordingly
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

/// Main ui function to process user input.
///
/// # Update loop
///
/// - **Escape**: quit program
/// - **Enter**: select current widget
/// - **Arrow keys**: move cursor 1 character up/down/left/right
/// - **Any other charcater**: type given character, (including backspace)
fn wait_for_input(window: &Window, model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let mut cursor = Cursor { x: 0, y: 0 };
    reset_scroll_offset();
    let mut break_condition: BreakCondition;
    let mut limit = model.len() - 1;
    let left_margin: usize = 2;
    let mut current_widget: usize = 0;
    let height = window.get_max_y() as usize;

    if let Some(Widget::Input { content, label, .. }) = find_widget_by_y(model, cursor.y as i32) {
        // starting widget is input, move cursor to end of its content
        cursor.x = content.len() + label.len();
    } else {
        // starting widget is text, hide the cursor
        curs_set(0);
    }

    window.mv(cursor.y as i32, cursor.x as i32);

    draw(window, model);

    // main keyboard input loop
    loop {
        let ch = window.getch();
        match ch {
            Some(Input::Character('\u{1b}')) => {
                // escape pressed, exit loop and set program to quit
                break_condition = BreakCondition::QUIT;
                break;
            }
            Some(Input::KeyEnter) |
            Some(Input::Character('\n')) => {
                // enter/return pressed, prepare program to quit
                break_condition = BreakCondition::QUIT;

                for widget in model {
                    match widget {
                        Widget::Input { y, id, .. } if *y == (cursor.y as i32) => {
                            // selected widget is input, set break condition and selected id
                            current_widget = *id;
                            break_condition = BreakCondition::INPUT;
                            break;
                        }
                        Widget::Text { y, id, .. } if *y == (cursor.y as i32) => {
                            // selected widget is text, set break condition and selected id
                            current_widget = *id;
                            break_condition = BreakCondition::SELECTION;
                            break;
                        }
                        _ => {
                            break_condition = BreakCondition::QUIT;
                        },
                    }
                }

                // conditions have been set, exit loop
                break;
            }
            Some(Input::KeyUp) => {
                // up arrow pressed, move cursor up
                if *get_scroll_offset() < 0 && cursor.y == 0 {
                    // cursor at top of window and widgets exist above cursor, scroll up
                    scroll(1);
                    draw(window, model);
                    cursor.y = 1;
                    limit += 1;
                }

                if cursor.y > 0 {
                    // now move cursor up 1 level
                    window.mvprintw(cursor.y as i32, 0, " ");
                    cursor.y -= 1;

                    if let Some(Widget::Input { content, label, .. }) = find_widget_by_y(model, cursor.y as i32) {
                        // new row is input, show cursor at end of its content
                        curs_set(1);
                        cursor.x = content.len() + label.len();
                    } else {
                        // new row is text, hide cursor and show selection carrot
                        curs_set(0);
                        window.mvprintw(cursor.y as i32, 0, ">");
                    }
                }
            }
            Some(Input::KeyDown) => {
                // down arrow pressed, move cursor down
                if cursor.y < limit {
                    // only move down if more widgets to scroll to
                    window.mvprintw(cursor.y as i32, 0, " ");
                    cursor.y += 1;

                    if cursor.y > height - 1 {
                        // cursor at bottom of window, scroll window
                        scroll(-1);
                        draw(window, model);
                        cursor.y -= 1;
                        limit -= 1;
                    }

                    if let Some(Widget::Input { content, label, .. }) = find_widget_by_y(model, cursor.y as i32) {
                        // new row is input, show cursor at end of its content
                        curs_set(1);
                        cursor.x = content.len() + label.len();
                    } else {
                        // new row is text, hide cursor and show selection carrot
                        curs_set(0);
                        window.mvprintw(cursor.y as i32, 0, ">");
                    }
                }
            }
            Some(Input::KeyLeft) => {
                // left arrow pressed, move cursor up to label 1 cell if row is input
                if let Some(Widget::Input { label, .. }) = find_widget_by_y(model, cursor.y as i32) {
                    if cursor.x > label.len() {
                        cursor.x -= 1;
                    }
                }
            }
            Some(Input::KeyRight) => {
                // right arrow pressed, move cursor up to end to content 1 cell if row is input
                if let Some(Widget::Input { content, label, .. }) = find_widget_by_y(model, cursor.y as i32) {
                    if cursor.x < content.len() + label.len() {
                        cursor.x += 1;
                    }
                }
            }
            Some(Input::KeyBackspace) |
            Some(Input::KeyDC) |
            Some(Input::Character('\u{7f}')) => {
                // backspace/delete pressed

                let mut content;
                let filter;
                let label_len;

                if let Some(Widget::Input { content: ref_content, filter: ref_filter, label, .. }) = find_widget_by_y(model, cursor.y as i32).cloned() {
                    // current row is input, get its content and filter type
                    content = ref_content.clone();
                    filter = ref_filter.clone();
                    label_len = label.len();
                } else {
                    // current row is text, nothing to backspace so skip
                    continue;
                }

                if cursor.x > label_len { // TODO: may need to be label len
                    // move cursor 1 cell and delete current character
                    cursor.x -= 1;
                    window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
                    window.delch();
                    
                    if let Some(widget) = find_widget_by_y_mut(model, cursor.y as i32) {
                        if let Widget::Input { content: widget_content, .. } = widget {
                            // remove same character from widget's content property
                            // label_len = label.len();
                            widget_content.remove(cursor.x - label_len);
                        }
                    }

                    content.remove(cursor.x - label_len);

                    if filter != Filter::Off {
                        // input widget has a filter, apply it
                        filter_widgets(model, filter, &content);
                        draw(window, model);
                    }
                }
            }
            Some(Input::Character(c)) => {
                // any other character was typed
                
                let mut content;
                let filter;
                let current_level = cursor.y;
                let mut label_len = 0;
                
                if let Some(Widget::Input { content: ref_content, filter: ref_filter, .. }) = find_widget_by_y(model, cursor.y as i32).cloned() {
                    // current row is input, get its content and filter type
                    content = ref_content.clone();
                    filter = ref_filter.clone();
                } else {
                    // current row is text, nothing to type so skip
                    continue;
                }

                // insert typed character to screen
                window.mvinsch(cursor.y as i32, (cursor.x + left_margin) as i32, c as chtype);
                cursor.x += 1;

                if let Some(widget) = find_widget_by_y_mut(model, current_level as i32) {
                    if let Widget::Input { content: widget_content, label, .. } = widget {
                        // insert same character into widget's content property
                        label_len = label.len();
                        widget_content.insert(cursor.x - label_len - 1, c);
                    }
                }

                content.insert(cursor.x - label_len - 1, c);

                if filter != Filter::Off {
                    // input widget has a filter, apply it
                    filter_widgets(model, filter, &content);
                    draw(window, model);
                }
                
            }
            _ => {}
        }

        window.mv(cursor.y as i32, (cursor.x + left_margin) as i32);
        window.refresh();
    }

    return (break_condition, current_widget);
}

/// Initialize ui for first time.
pub fn init(model: &mut Vec<Widget>) -> (BreakCondition, usize) {
    let window = get_window();
    window.keypad(true);
    window.nodelay(true);
    window.timeout(0);
    noecho();
    curs_set(1);

    return wait_for_input(&window, model);
}

/// Tear down ui after program has run to completion.
pub fn destroy() {
    endwin();
}
