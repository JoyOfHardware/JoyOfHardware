use zoon::*;
use zoon::eprintln;
use std::rc::Rc;

fn scale_linearly_and_clamp(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    let x           = x as f32;
    let in_min      = in_min as f32;
    let in_max      = in_max as f32;
    let out_min     = out_min as f32;
    let out_max     = out_max as f32;
    let scaled_val  = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
    let clamped_val = scaled_val.max(out_min).min(out_max);
    clamped_val.round() as u32
}

fn build_text_element(text: &str, font_size: u32) -> impl Element {
    El::new()
        .s(Font::new().size(font_size).weight(FontWeight::ExtraBold))
        .s(Align::new().left())
        .s(Font::new().color(color!("orange")))
        .child(text)
}

fn build_amazing_hardware_column_hero(font_size: u32) -> impl Element {
    let column_of_text = 
        Column::new()
            .s(Align::center())
            .items(vec![
                build_text_element("BUILD", font_size),
                build_text_element("AMAZING", font_size),
                build_text_element("HARDWARE", font_size),
            ]);
    El::new()
        .s(Align::center())
        .s(Width::fill().max(600))
        .s(RoundedCorners::all(25))
        .s(Height::fill().max(800))
        .s(Shadows::new([Shadow::new().blur(50).color("#dddddd")]))
        .update_raw_el(|raw_el| {
            raw_el.style("background", "linear-gradient(to bottom, orange, white)")
        })
        .child(column_of_text)
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
        .on_press(|| 
            {
                crate::router::ROUTER.go(crate::router::Route::Login);
                eprintln!("Let's Go button pressed");
            }
        )
}

pub fn page() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Width::fill().max(1024))
        .s(Padding::all(25))
        .s(Height::fill().min(600))
        .multiline()
        .on_viewport_size_change(|width, _| {
            crate::WINDOW_SIZE.set_neq(width as u32);
        })
        .item_signal(crate::WINDOW_SIZE.signal().map(|window_width| {
            let font_size = scale_linearly_and_clamp(window_width, 300, 1200, 50, 100);
            build_amazing_hardware_column_hero(font_size)
        }))
        .item(call_to_action_panel())
}