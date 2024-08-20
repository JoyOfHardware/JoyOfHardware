use zoon::*;
use std::sync::Arc;

pub fn button_impact(text: &str, on_press_action: impl Fn() + 'static) -> impl Element {
    let (hovered, hover_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::center())
        .s(Width::exact(200))
        .s(Padding::new().y(10))
        // .s(Borders::all(border)
        .s(Font::new()
            .size(30)
            .weight(FontWeight::Bold)
            .line(FontLine::new().underline_signal(hover_signal.map_bool(|| true, || false))))
        .label(text)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(on_press_action)
}

pub fn form_input_field<F>(
    field_name: &str,
    text_signal: Mutable<Arc<String>>,
    input_type: impl Into<InputType>,
    on_change: F) -> impl Element
where
    F: Fn(String) + 'static,
{
    TextInput::new()
        .s(Align::new().center_x())
        .s(Padding::new().x(10).y(10))
        .s(Width::fill().max(300))
        .placeholder(Placeholder::new(field_name))
        .s(Borders::new().bottom(
            Border::new().width(2).color(crate::theme::border_gray)))
        .s(Background::new().color(crate::theme::fill_gray))
        .s(RoundedCorners::all(5))
        .input_type(input_type)
        .label_hidden(field_name)
        .text_signal(text_signal.signal_cloned())
        .on_change(move |name| {
            on_change(name);
        })
}