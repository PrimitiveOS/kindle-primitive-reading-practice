use std::{
    fs,
    path::{Path, PathBuf},
};

use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{screen_rect, Canvas, Input, TextStyle};
use reading_practice_core::PASSAGES;
use reading_practice_render::View;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    for passage in &PASSAGES {
        let pages = canvas.text_pages(
            screen_rect(WIDTH as i32, 90, 358, 1084, 880),
            passage.text,
            TextStyle::new(43).lines(15),
        );
        assert_eq!(
            pages.len(),
            1,
            "passage fits the reading view: {}",
            passage.title
        );
    }
    let save = std::env::temp_dir().join(format!("reading-practice-{}.json", std::process::id()));
    let mut view = View::open(save.clone());
    let mut capture = |view: &View, name: &str, dark: bool| {
        write_png(
            &output.join(name),
            WIDTH,
            HEIGHT,
            &frame(view, &mut canvas, dark),
        )
    };
    capture(&view, "passages.png", false)?;
    assert_eq!(view.input(Input::Tap(510.0, 555.0)), Change::Full);
    capture(&view, "passage.png", false)?;
    assert_eq!(view.input(Input::Tap(350.0, 1490.0)), Change::Full);
    assert_eq!(view.input(Input::Tap(510.0, 745.0)), Change::Partial);
    capture(&view, "question-dark.png", true)?;
    assert_eq!(view.input(Input::Tap(970.0, 1490.0)), Change::Full);
    let mut resumed = View::open(save.clone());
    assert_eq!(resumed.input(Input::Tap(350.0, 250.0)), Change::Full);
    assert_eq!(resumed.input(Input::Tap(350.0, 1490.0)), Change::Full);
    assert_eq!(resumed.study.current().unwrap().selected(), Some(1));
    assert_eq!(resumed.input(Input::Tap(350.0, 1490.0)), Change::Full);
    assert_eq!(resumed.study.current().unwrap().question(), Some(1));
    fs::remove_file(save)?;
    Ok(())
}
