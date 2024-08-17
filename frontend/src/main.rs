use zoon::*;
use zoon::eprintln;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();
static NEW_MESSAGE_TEXT: Lazy<Mutable<String>> = lazy::default();
static VIEWPORT_X: Lazy<Mutable<i32>> = lazy::default();

fn banner() -> impl Element {
    let gradient = "linear-gradient(to bottom, orange, white)";
    El::new()
        .s(Width::percent(50).min(600))
        .s(Padding::all(15))
        .s(Height::fill())
        .child(
            Column::new()
                .s(Align::new().top()) // Align the banner to the top
                .s(Padding::new().y(20).x(10)) // Add padding around the banner
                .s(RoundedCorners::all(25))
                .s(Height::fill())
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

// Modify the root function to include the banner and rounded corners
fn root() -> impl Element {
    let gradient = "linear-gradient(to bottom, orange, white)";
    let (hovered, hover_signal) = Mutable::new_and_signal(false);
    Row::new()

        .s(Background::new().color(color!("#ffffff")))
        .s(Width::fill())
        .s(Height::fill())
        .item(
            banner()
        )
        .item(
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
                            // Handle login logic here
                            eprintln!("Login button pressed");
                        })
                )
        )
}