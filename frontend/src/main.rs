use zoon::*;
use zoon::eprintln;
use std::rc::Rc;

use zoon::*;

fn main() {
    start_app("app", root);
}

static WINDOW_SIZE: Lazy<Mutable<u32>> = Lazy::new(|| Mutable::new(0));

fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Width::fill())
        .s(Height::fill().min(600))
        .multiline()
        .on_viewport_size_change(|width, _| {
            WINDOW_SIZE.set_neq(width as u32);
        })
        .item_signal(WINDOW_SIZE.signal().map(|window_width| {
            build_amazing_hardware_column_hero(map_window_size_to_font_size(window_width))
        }))
        .item(call_to_action_panel())
}

fn build_amazing_hardware_column_hero(font_size: u32) -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Width::fill().max(600))
        .s(Padding::new().left(15))
        .s(RoundedCorners::all(25))
        .s(Height::fill().max(800))
        .s(Shadows::new([Shadow::new().blur(50).color("#dddddd")]))
        .update_raw_el(|raw_el| {
            raw_el.style("background", "linear-gradient(to bottom, orange, white)")
        })
        .items(vec![
            build_text_element("BUILD", font_size),
            build_text_element("AMAZING", font_size),
            build_text_element("HARDWARE", font_size),
        ])
}

fn build_text_element(text: &str, font_size: u32) -> impl Element {
    El::new()
        .s(Font::new().size(font_size).weight(FontWeight::ExtraBold))
        .s(Align::center())
        .s(Font::new().color(color!("orange")))
        .child(text)
}

fn map_window_size_to_font_size(window_width: u32) -> u32 {
    let min_width = 700;
    let max_width = 1200;
    match window_width {
        w if w <= min_width => 50,
        w if w >= max_width => 100,
        w => {
            let scale = (w - min_width) as f32 / (max_width - min_width) as f32;
            (50.0 + scale * 50.0).round() as u32
        }
    }
}

fn call_to_action_panel() -> impl Element {
    let (hovered, hover_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::center())
        .s(Width::exact(200))
        .s(Padding::new().y(10))
        .s(Font::new()
            .size(30)
            .weight(FontWeight::Bold)
            .line(FontLine::new().underline_signal(hover_signal.map_bool(|| true, || false))))
        .label("Let's Go! 👉")
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(|| eprintln!("Let's Go button pressed"))
}
