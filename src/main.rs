use druid::{AppLauncher, Color, PlatformError, Widget, WindowDesc};
use druid::widget::{Flex, Label};

mod animechan;
mod gui;
mod controller;


fn build_ui() -> impl Widget<()> {
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top left"), 1.0)
                .with_flex_child(Label::new("bottom left"), 1.0),
            1.0)
        .with_flex_child(
            Flex::column()
                .with_flex_child(Label::new("top right"), 1.0)
                .with_flex_child(Label::new("bottom right"), 1.0),
            1.0)
}

#[tokio::main]
async fn main() {
    let window = WindowDesc::new(build_ui).title("External Event Demo");
    let launcher = AppLauncher::with_window(window);
    let event_sink = launcher.get_external_handle();
    tokio::spawn(async move {
        println!("hello world")
    });
    launcher
        .launch(())
        .expect("launch failed");
}