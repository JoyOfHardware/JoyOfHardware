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
            // .s(Align::center())
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::new().y(25))
            .s(Gap::new().y(25))
            .items(vec![
                title.unify(),
                self.name_input().unify(),
                self.password_input().unify()
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

    fn name_input(&self) -> impl Element {
        TextInput::new()
            .s(Align::new().center_x())
            .s(Padding::new().x(10).y(10))
            .s(Width::fill().max(300))
            .placeholder(Placeholder::new("Username"))
            .s(Borders::new().bottom(
                Border::new().width(2).color(color!("#dddddd"))))
            .s(Background::new().color(color!("#f0f0f0")))
            .s(RoundedCorners::all(5))
            .label_hidden("Username")
            .text_signal(
                self.username
                    .signal_cloned()
            )
            .on_change(clone!((self => s) move |name| s.username.set(Arc::new(name))))
    }

    fn password_input(&self) -> impl Element {
        TextInput::new()
            .input_type(InputType::password())
            .s(Align::new().center_x())
            .s(Padding::new().x(10).y(10))
            .s(Width::fill().max(300))
            .placeholder(Placeholder::new("Password"))
            .s(Borders::new().bottom(
                Border::new().width(2).color(color!("#dddddd"))))
            .s(Background::new().color(color!("#f0f0f0")))
            .s(RoundedCorners::all(5))
            .label_hidden("Password")
            .text_signal(
                self.password
                    .signal_cloned()
            )
            .on_change(clone!((self => s) move |name| s.password.set(Arc::new(name))))
    }
    
    pub fn page(&self) -> impl Element {

        El::new()
            .s(Align::center())
            .s(Width::percent(90).max(400))
            .s(Height::percent(90).max(600))
            .child(self.login_form())
    }

}
