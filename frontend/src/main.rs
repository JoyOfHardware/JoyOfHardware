use zoon::*;
use zoon::eprintln;
use std::rc::Rc;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<u32>> = lazy::default();  // Changed to u32
static NEW_MESSAGE_TEXT: Lazy<Mutable<String>> = lazy::default();

fn text_orang_bold(text: &str, font_size_signal: impl Signal<Item = u32> + Unpin + 'static) -> impl Element {
    El::new()
        .s(Font::new().size_signal(font_size_signal).weight(FontWeight::ExtraBold))
        .s(Align::center())
        .s(Font::new().color(color!("orange")))
        .child(text)
}

fn build_amazing_hardware_column_hero(create_font_size_signal: Rc<dyn Fn() -> Box<dyn Signal<Item = u32> + Unpin>>) -> impl Element {
    Column::new()
        .s(Align::new().top().right().center_y())
        .s(Width::fill().max(600))
        .s(Padding::new().left(15))
        .s(RoundedCorners::all(25))
        .s(Height::fill().max(800))
        .s(Shadows::new([Shadow::new().blur(50).color("#dddddd")]))
        .update_raw_el(|raw_el| {
            raw_el.style("background", "linear-gradient(to bottom, orange, white)")
        })
        .items(vec![
            text_orang_bold("BUILD", create_font_size_signal()),
            text_orang_bold("AMAZING", create_font_size_signal()),
            text_orang_bold("HARDWARE", create_font_size_signal()),
        ])
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
        .on_press(|| eprintln!("Login button pressed"))
}

fn create_font_size_signal(container_width: Rc<Mutable<u32>>) -> Rc<dyn Fn() -> Box<dyn Signal<Item = u32> + Unpin>> {
    let min_width = 700;
    let max_width = 1200;
    Rc::new(move || {
        Box::new(container_width.signal().map(move |width| {
            match width {
                w if w <= min_width => 50,
                w if w >= max_width => 100,
                w => {
                    let scale = (w - min_width) as f32 / (max_width - min_width) as f32;
                    (50.0 + scale * 50.0).round() as u32
                }
            }
        })) as Box<dyn Signal<Item = u32> + Unpin>
    })
}

fn root() -> impl Element {
    let container_width = Rc::new(Mutable::new(0u32));  // Changed to u32

    Row::new()
        .s(Background::new().color(color!("#ffffff")))
        .s(Width::fill())
        .s(Height::fill().min(600))
        .on_viewport_size_change({
            let container_width = Rc::clone(&container_width);
            move |width, _| {
                container_width.set_neq(width as u32);  // Ensure it's u32
            }
        })
        .multiline()
        .item(build_amazing_hardware_column_hero(create_font_size_signal(Rc::clone(&container_width))))
        .item(call_to_action_panel())
}
