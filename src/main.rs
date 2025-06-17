use vizia::prelude::*;

const VIZIA_LOGO: &[u8; 8129] = include_bytes!("vizia-logo.png");

fn main() {
    let res = match app() {
        Ok(()) => (),
        Err(e) => {
            dbg!(e);
            ()
        }
    };
}

fn app() -> Result<(), vizia::ApplicationError> {
    Application::new(|cx| {
        cx.load_image("vizia-logo.png", VIZIA_LOGO, ImageRetentionPolicy::Forever);

        VStack::new(cx, |cx| {
            Image::new(cx, "vizia-logo.png");
        })
        .alignment(Alignment::Center);
    })
    .title("Vizia Example")
    .inner_size((400, 400))
    .run()
}
