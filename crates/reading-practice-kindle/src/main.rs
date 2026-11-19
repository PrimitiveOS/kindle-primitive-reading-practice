use std::path::PathBuf;

use reading_practice_render::View;

fn main() -> Result<(), String> {
    let directory = std::env::var_os("PRIMITIVE_APP_DIR")
        .map_or_else(|| PathBuf::from("/mnt/us/reading-practice"), PathBuf::from);
    primitive_app::run(View::open(directory.join("progress.json")), &directory)
}
