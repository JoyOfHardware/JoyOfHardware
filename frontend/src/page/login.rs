use zoon::*;
use zoon::eprintln;
use std::sync::Arc;

#[derive(Clone)]
pub struct LoginPage {
    username: Mutable<Arc<String>>,
    password: Mutable<Arc<String>>
}


impl LoginPage {
    pub fn new() -> impl Element {
        Self {
            username: Mutable::new(Arc::new("".to_string())),
            password: Mutable::new(Arc::new("".to_string()))
        }.page()
    }

    fn login_form(&self) -> impl Element {
        let is_mobile = crate::WINDOW_SIZE.signal().map(|window_width| {
            window_width < 760
        });

        let adventure = El::new()
            .s(Font::new().size(50).weight(FontWeight::ExtraBold))
            .s(Align::new().left())
            .s(Font::new().color(color!("gray")))
            .child("Adventure");

        let awaits = El::new()
            .s(Font::new().size(50).weight(FontWeight::ExtraBold))
            .s(Align::new().right())
            .s(Font::new().color(color!("gray")))
            .child("Awaits!");

        let title = Column::new()
            .s(Align::new().center_x())
            .s(Width::fill())
            .s(Padding::new().bottom(25))
            .items(vec![
                adventure.unify(),
                awaits.unify(),
            ]);

        let column = Column::new()
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::new().y(25))
            .s(Gap::new().y(25))
            .items(vec![
                title.unify(),
                crate::assets::form_input_field(
                    "Username",
                    self.username.clone(),
                    InputType::text(),
                    clone!((self => s) move |name| s.username.set(Arc::new(name)))
                ).unify(),
                crate::assets::form_input_field(
                    "Password",
                    self.password.clone(),
                    InputType::password(),
                    clone!((self => s) move |name| s.password.set(Arc::new(name)))
                )
                .unify(),
            ]);

        El::new()
            .s(Align::center())
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::all(25))
            .s(RoundedCorners::all(25))
            // show a rounded shadow on desktop
            .s(Shadows::with_signal(is_mobile.map_bool(
                || [Shadow::new()],
                || [Shadow::new().blur(50).color("#dddddd")],
            )))
            .child(column)
    }

    pub fn page(&self) -> impl Element {

        El::new()
            .s(Align::center())
            .s(Width::percent(90).max(400))
            .s(Height::percent(90).max(600))
            .child(self.login_form())
    }

}
