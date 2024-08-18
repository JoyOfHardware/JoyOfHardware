use zoon::*;
use zoon::eprintln;

fn login_form() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Width::fill())
        .s(Height::fill())
        .s(Padding::all(25))
        .s(RoundedCorners::all(25))
        .s(Shadows::new([Shadow::new().blur(50).color("#dddddd")]))
        .child("login")
}

pub fn page() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Width::percent(90).max(400))
        .s(Height::percent(90).max(600))
        .child(login_form())
}