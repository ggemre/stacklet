use crate::external::widget::Widget;

/// Return the widget from model that has the same y value.
pub fn find_widget_by_y(model: &Vec<Widget>, y: i32) -> Option<&Widget> {
    model.iter().find(|&widget| match widget {
        | Widget::Input { y: widget_y, .. }
        | Widget::Text { y: widget_y, .. } => *widget_y == y,
    })
}

/// Return the mutable widget from model that has the same y value.
pub fn find_widget_by_y_mut(
    model: &mut Vec<Widget>,
    y: i32,
) -> Option<&mut Widget> {
    model.iter_mut().find(|widget| match widget {
        | Widget::Input { y: widget_y, .. }
        | Widget::Text { y: widget_y, .. } => *widget_y == y,
    })
}
