use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

use primitive_app::{App, Change, WIDTH};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Input, Rect, TextStyle, BODY, CAPTION, PAPER,
};
use reading_practice_core::{Study, LEVEL_SIZE, PASSAGES};

pub struct View {
    pub study: Study,
    path: Option<PathBuf>,
    in_library: bool,
    level: usize,
    pending_save: bool,
}

impl View {
    pub fn open(path: PathBuf) -> Self {
        let mut view = Self::preview();
        view.study = load(&path).unwrap_or_default();
        view.level = view.study.active().unwrap_or(0) / LEVEL_SIZE;
        view.path = Some(path);
        view
    }

    pub fn preview() -> Self {
        Self {
            study: Study::default(),
            path: None,
            in_library: true,
            level: 0,
            pending_save: false,
        }
    }

    fn persist(&mut self) {
        self.pending_save = true;
        let _ = self.save();
    }

    fn draw_library(&self, canvas: &mut Canvas, w: i32) {
        header(canvas, "Reading Practice", "Done");
        if let Some(active) = self
            .study
            .active()
            .filter(|_| self.study.current().is_some_and(|r| !r.finished()))
        {
            canvas.button(primary(w, 200), "Continue", true, false);
            canvas.text(
                screen_rect(w, 656, 233, 528, 52),
                PASSAGES[active].title,
                BODY,
            );
        } else {
            canvas.text(screen_rect(w, 40, 235, 1184, 70), "Choose a passage", BODY);
        }
        for (level, title) in ["Level 1", "Level 2"].into_iter().enumerate() {
            canvas.button(level_button(w, level), title, self.level == level, false);
        }
        for (wanted, row) in PASSAGES[self.level * LEVEL_SIZE..(self.level + 1) * LEVEL_SIZE]
            .iter()
            .enumerate()
        {
            let index = self.level * LEVEL_SIZE + wanted;
            let rect = passage_button(w, wanted);
            canvas.button(rect, row.title, false, false);
            let status = match self.study.at(index) {
                Some(round) if round.finished() => "Done",
                Some(_) => "Started",
                None => "",
            };
            canvas.text(
                screen_rect(w, 1003, 526 + wanted as i32 * 137, 185, 40),
                status,
                CAPTION.right(),
            );
        }
    }

    fn draw_passage(&self, canvas: &mut Canvas, w: i32) {
        let active = self.study.active().expect("an open passage");
        let passage = &PASSAGES[active];
        header(canvas, "Reading Practice", "Back");
        canvas.text(
            screen_rect(w, 40, 205, 1184, 72),
            passage.title,
            TextStyle::new(52).bold(),
        );
        canvas.panel(screen_rect(w, 40, 305, 1184, 1000), 22, 2, PAPER);
        canvas.text(
            screen_rect(w, 90, 358, 1084, 880),
            passage.text,
            TextStyle::new(43).lines(15),
        );
        canvas.button(primary(w, 1440), "Questions", true, false);
    }

    fn draw_question(&self, canvas: &mut Canvas, w: i32) {
        let active = self.study.active().expect("an open passage");
        let round = self.study.current().expect("an open passage");
        let passage = &PASSAGES[active];
        header(canvas, "Reading Practice", "Back");
        canvas.text(
            screen_rect(w, 40, 207, 1184, 48),
            passage.title,
            BODY.bold(),
        );
        if let Some(index) = round.question() {
            let question = &passage.questions[index];
            let (current, total) = round.position();
            canvas.text(
                screen_rect(w, 40, 284, 1184, 64),
                &format!("Question {current} of {total}"),
                CAPTION,
            );
            canvas.text(
                screen_rect(w, 40, 367, 1184, 156),
                question.prompt,
                TextStyle::new(48).bold().lines(2),
            );
            for (i, option) in question.choices.iter().enumerate() {
                canvas.button(
                    choice_button(w, i),
                    option,
                    round.selected() == Some(i),
                    false,
                );
            }

            if let Some(answer) = round.selected() {
                canvas.panel(screen_rect(w, 40, 1040, 1184, 290), 22, 2, PAPER);
                let result = if answer == question.correct {
                    "Correct"
                } else {
                    "Not quite"
                };
                canvas.text(screen_rect(w, 68, 1068, 1128, 56), result, BODY.bold());
                canvas.text(
                    screen_rect(w, 68, 1142, 1128, 154),
                    question.explanation,
                    BODY.lines(3),
                );
                canvas.button(primary(w, 1440), "Next", true, false);
            }
            canvas.button(secondary(w, 1440), "Passage", false, false);
        } else {
            let (correct, total) = round.score(passage);
            canvas.text(
                screen_rect(w, 40, 402, 1184, 90),
                "Passage complete",
                TextStyle::new(64).bold().center(),
            );
            canvas.text(
                screen_rect(w, 40, 543, 1184, 70),
                &format!("{correct} of {total} correct"),
                BODY.center(),
            );
            canvas.button(primary(w, 960), "Read again", correct == total, false);
            if correct < total {
                canvas.button(secondary(w, 960), "Retry missed", true, false);
            }
            canvas.button(primary(w, 1440), "Passages", false, false);
        }
    }

    fn tap(&mut self, x: f32, y: f32) -> Change {
        let w = WIDTH as i32;
        if back_button(w).contains(x, y) {
            if self.in_library {
                return Change::Exit;
            }
            self.in_library = true;
            return Change::Full;
        }

        if self.in_library {
            if primary(w, 200).contains(x, y) && self.study.current().is_some_and(|r| !r.finished())
            {
                self.in_library = false;
                return Change::Full;
            }
            if let Some(level) = (0..2).find(|&i| level_button(w, i).contains(x, y)) {
                self.level = level;
                return Change::Partial;
            }
            if let Some(row) = (0..LEVEL_SIZE).find(|&i| passage_button(w, i).contains(x, y)) {
                self.study.open(self.level * LEVEL_SIZE + row);
                self.in_library = false;
                self.persist();
                return Change::Full;
            }
            return Change::None;
        }

        let Some(round) = self.study.current() else {
            return Change::None;
        };
        if round.passage_open {
            if primary(w, 1440).contains(x, y) {
                self.study.show_passage(false);
                self.persist();
                return Change::Full;
            }
        } else if round.finished() {
            if primary(w, 1440).contains(x, y) {
                self.in_library = true;
                return Change::Full;
            }
            let redo = if primary(w, 960).contains(x, y) {
                self.study.restart(false)
            } else if secondary(w, 960).contains(x, y) {
                self.study.restart(true)
            } else {
                false
            };
            if redo {
                self.persist();
                return Change::Full;
            }
        } else {
            if secondary(w, 1440).contains(x, y) {
                self.study.show_passage(true);
                self.persist();
                return Change::Full;
            }
            if primary(w, 1440).contains(x, y) && self.study.advance() {
                self.persist();
                return Change::Full;
            }
            if let Some(choice) = (0..3).find(|&i| choice_button(w, i).contains(x, y)) {
                if self.study.answer(choice) {
                    self.persist();
                    return Change::Partial;
                }
            }
        }
        Change::None
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let w = canvas.bounds().w;
        if self.in_library {
            self.draw_library(canvas, w);
        } else if self.study.current().is_some_and(|r| r.passage_open) {
            self.draw_passage(canvas, w);
        } else if self.study.current().is_some() {
            self.draw_question(canvas, w);
        }
        if self.pending_save {
            canvas.text(
                screen_rect(w, 40, 1538, 1184, 68),
                "Progress could not be saved yet.",
                CAPTION,
            );
        }
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) => self.tap(x, y),
            Input::PageBack if !self.in_library => {
                self.in_library = true;
                Change::Full
            }
            Input::PageForward if self.in_library && self.level == 0 => {
                self.level = 1;
                Change::Partial
            }
            Input::PageBack if self.level == 1 => {
                self.level = 0;
                Change::Partial
            }
            _ => Change::None,
        }
    }

    fn save(&mut self) -> io::Result<()> {
        if self.pending_save {
            if let Some(path) = &self.path {
                primitive_app::save(path, &self.study.pack())?;
            }
            self.pending_save = false;
        }
        Ok(())
    }

    fn storage_returned(&mut self) -> io::Result<()> {
        self.save()
    }
}

fn load(path: &Path) -> Option<Study> {
    primitive_app::with_storage(|| {
        let mut bytes = Vec::new();
        File::open(path)?.take(16_385).read_to_end(&mut bytes)?;
        Ok(Study::unpack(&bytes).filter(|_| bytes.len() <= 16_384))
    })
    .ok()
    .flatten()
}

fn primary(w: i32, y: i32) -> Rect {
    screen_rect(w, 40, y, 564, 110)
}

fn secondary(w: i32, y: i32) -> Rect {
    screen_rect(w, 660, y, 564, 110)
}

fn level_button(w: i32, level: usize) -> Rect {
    if level == 0 {
        primary(w, 352)
    } else {
        secondary(w, 352)
    }
}

fn passage_button(w: i32, row: usize) -> Rect {
    screen_rect(w, 40, 500 + row as i32 * 137, 1184, 112)
}

fn choice_button(w: i32, row: usize) -> Rect {
    screen_rect(w, 40, 553 + row as i32 * 151, 1184, 126)
}
