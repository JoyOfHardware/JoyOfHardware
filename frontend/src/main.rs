use zoon::*;
use zoon::eprintln;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();
static NEW_MESSAGE_TEXT: Lazy<Mutable<String>> = lazy::default();

use zoon::*;

fn banner() -> impl Element {
    Column::new()
        .s(Align::new().top()) // Align the banner to the top
        .s(Padding::new().y(20).x(10)) // Add padding around the banner
        .item(
            El::new()
                .s(Font::new().size(100).weight(FontWeight::ExtraBold)) // Set font size and thickness
                .s(Align::center())
                .s(Font::new().color(color!("orange"))) // Set the font color to orange
                .child("Build Amazing Hardware"),
        )
}

fn do_thing() {
    eprintln!("Got Enter!");;
}

fn login_form() -> impl Element {
    Column::new()
        .s(Align::new().right()) // Align the form to the right
        .s(Padding::new().y(20).x(20)) // Add padding around the form
        .item(
            TextInput::new()
            .s(Padding::all(10))
            .s(RoundedCorners::new().left(5))
            .s(Width::fill())
            .s(Font::new().size(17))
            .focus(true)
            .on_change(|text| NEW_MESSAGE_TEXT.set(text))
            .label_hidden("New message text")
            .placeholder(Placeholder::new("Message"))
            .on_key_down_event(|event| event.if_key(Key::Enter, do_thing))
            .text_signal(NEW_MESSAGE_TEXT.signal_cloned())
        )
        // .item(
        //     TextInput::new()
        //         .s(Width::exact(200))
        //         .placeholder("Username") // Set a placeholder
        //         .on_change(|text| {
        //             // Handle the change in the username input
        //             eprintln!("Username: {}", text);
        //         })
        // )
        // .item(
        //     TextInput::new()
        //         .s(Width::exact(200))
        //         .input_type(InputType::password()) // Use the correct password input type
        //         .placeholder("Password") // Set a placeholder
        //         .on_change(|text| {
        //             // Handle the change in the password input
        //             eprintln!("Username: {}", text);
        //         })
        // )
        .item(
            Button::new()
                .s(Width::exact(200))
                .s(Padding::new().y(10))
                .label("Login")
                .on_press(|| {
                    // Handle login logic here
                    eprintln!("Login button pressed");
                })
        )
}

// Modify the root function to include the banner and rounded corners
fn root() -> impl Element {
    let gradient = "linear-gradient(to bottom, orange, white)";
    Row::new()
        .s(Background::new().color(color!("#ffffff")))
        .s(Width::fill())
        .s(Height::fill())
        .item(
            El::new()
                .s(Width::percent(50).min(600))
                .child(banner())
        )
        .item(
            El::new()
                .s(Width::fill())
                .child(login_form())
        )
}

// fn counter_button(label: &str, step: i32) -> impl Element {
//     let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//     Button::new()
//         .s(Width::exact(45))
//         .s(RoundedCorners::all_max())
//         .s(Background::new()
//             .color_signal(hovered_signal.map_bool(|| color!("#edc8f5"), || color!("#E1A3EE", 0.8))))
//         .s(Borders::all(
//             Border::new()
//                 .width(2)
//                 .color(color!("oklch(0.6 0.182 350.53 / .7")),
//         ))
//         .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
//         .label(label)
//         .on_press(move || *COUNTER.lock_mut() += step)
// }
