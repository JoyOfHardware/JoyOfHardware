use zoon::*;
use zoon::eprintln;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();
static NEW_MESSAGE_TEXT: Lazy<Mutable<String>> = lazy::default();

fn build_amazing_hardware_column_hero() -> impl Element {
    let gradient = "linear-gradient(to bottom, orange, white)";
    El::new()
        .s(Width::percent(50).min(600))
        .s(Padding::all(15))
        .s(Height::fill())
        .child(
            Column::new()
                .s(Align::new().top().right().center_y()) // Align the banner to the top
                .s(Width::fill().max(600))
                .s(Padding::new().y(20).x(10)) // Add padding around the banner
                .s(RoundedCorners::all(25))
                .s(Height::fill().max(800))
                .s(Shadows::new([Shadow::new().blur(50).color("#dddddd")]))
                .update_raw_el(|raw_el| {
                    raw_el.style("background", gradient)
                    }
                )
                .item(
                    El::new()
                        .s(Font::new().size(100).weight(FontWeight::ExtraBold)) // Set font size and thickness
                        .s(Align::center())
                        .s(Font::new().color(color!("orange"))) // Set the font color to orange
                        .child("Build Amazing Hardware"),
                )
        )
}

fn lets_go_call_to_action_banner() -> impl Element {
    let (hovered, hover_signal) = Mutable::new_and_signal(false);
    El::new()
        .s(Align::center())
        .child(
            Button::new()
                .s(Width::exact(200))
                .s(Padding::new().y(10))
                .s(Font::new()
                    .size(30)
                    .weight(FontWeight::Bold)
                    .line(FontLine::new().underline_signal(hover_signal.map_bool(|| true, || false)))
                )
                .label("Let's Go! 👉")
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(|| {
                    eprintln!("Login button pressed");
                })
        )
}

fn root() -> impl Element {
    let gradient = "linear-gradient(to bottom, orange, white)";
    Row::new()

        .s(Background::new().color(color!("#ffffff")))
        .s(Width::fill())
        .s(Height::fill())
        .item(build_amazing_hardware_column_hero())
        .item(lets_go_call_to_action_banner())
}