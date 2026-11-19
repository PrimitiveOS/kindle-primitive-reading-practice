use serde::{Deserialize, Serialize};

mod content;
pub use content::{Passage, Question, PASSAGES};

const QUESTION_COUNT: usize = 3;
pub const LEVEL_SIZE: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    queue: Vec<usize>,
    step: usize,
    answers: [Option<usize>; QUESTION_COUNT],
    pub passage_open: bool,
}

impl Round {
    fn new(queue: Vec<usize>) -> Self {
        Self {
            queue,
            step: 0,
            answers: [None; QUESTION_COUNT],
            passage_open: true,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.step + 1, self.queue.len())
    }

    pub fn question(&self) -> Option<usize> {
        self.queue.get(self.step).copied()
    }

    pub fn selected(&self) -> Option<usize> {
        self.question().and_then(|question| self.answers[question])
    }

    pub fn answered(&self) -> bool {
        self.selected().is_some()
    }

    pub fn finished(&self) -> bool {
        self.step >= self.queue.len()
    }

    fn answer(&mut self, choice: usize) -> bool {
        let Some(question) = self.question() else {
            return false;
        };
        if choice >= QUESTION_COUNT || self.answers[question].is_some() || self.passage_open {
            return false;
        }
        self.answers[question] = Some(choice);
        true
    }

    fn advance(&mut self) -> bool {
        if self.answered() && !self.passage_open {
            self.step += 1;
            true
        } else {
            false
        }
    }

    pub fn missed(&self, passage: &Passage) -> Vec<usize> {
        self.queue
            .iter()
            .copied()
            .filter(|&question| {
                self.answers[question]
                    .is_some_and(|answer| answer != passage.questions[question].correct)
            })
            .collect()
    }

    pub fn score(&self, passage: &Passage) -> (usize, usize) {
        let correct = self
            .queue
            .iter()
            .filter(|&&question| {
                self.answers[question] == Some(passage.questions[question].correct)
            })
            .count();
        (correct, self.queue.len())
    }

    fn valid(&self) -> bool {
        !self.queue.is_empty()
            && self.queue.len() <= QUESTION_COUNT
            && self.step <= self.queue.len()
            && self.queue.iter().enumerate().all(|(i, &choice)| {
                choice < QUESTION_COUNT
                    && !self.queue[..i].contains(&choice)
                    && (i >= self.step || self.answers[choice].is_some())
            })
            && self
                .answers
                .iter()
                .flatten()
                .all(|&choice| choice < QUESTION_COUNT)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Study {
    active: Option<usize>,
    rounds: [Option<Round>; PASSAGES.len()],
}

impl Study {
    pub fn unpack(bytes: &[u8]) -> Option<Self> {
        let state = serde_json::from_slice::<Self>(bytes).ok()?;
        if state
            .active
            .is_some_and(|i| i >= PASSAGES.len() || state.rounds[i].is_none())
            || state.rounds.iter().flatten().any(|round| !round.valid())
        {
            return None;
        }
        Some(state)
    }

    pub fn pack(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("reading progress is serializable")
    }

    pub fn active(&self) -> Option<usize> {
        self.active
    }

    pub fn at(&self, index: usize) -> Option<&Round> {
        self.rounds.get(index)?.as_ref()
    }

    pub fn current(&self) -> Option<&Round> {
        self.at(self.active?)
    }

    fn current_mut(&mut self) -> Option<&mut Round> {
        self.rounds.get_mut(self.active?)?.as_mut()
    }

    pub fn open(&mut self, index: usize) -> bool {
        let Some(round) = self.rounds.get_mut(index) else {
            return false;
        };
        round.get_or_insert_with(|| Round::new((0..QUESTION_COUNT).collect()));
        self.active = Some(index);
        true
    }

    pub fn show_passage(&mut self, open: bool) {
        if let Some(round) = self.current_mut() {
            round.passage_open = open;
        }
    }

    pub fn answer(&mut self, choice: usize) -> bool {
        self.current_mut().is_some_and(|round| round.answer(choice))
    }

    pub fn advance(&mut self) -> bool {
        self.current_mut().is_some_and(Round::advance)
    }

    pub fn restart(&mut self, missed_only: bool) -> bool {
        let Some(index) = self.active else {
            return false;
        };
        let Some(round) = &self.rounds[index] else {
            return false;
        };
        if !round.finished() {
            return false;
        }
        let queue = if missed_only {
            round.missed(&PASSAGES[index])
        } else {
            (0..QUESTION_COUNT).collect()
        };
        if queue.is_empty() {
            return false;
        }
        self.rounds[index] = Some(Round::new(queue));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_contains_two_levels_and_three_unique_choices_per_question() {
        assert_eq!(PASSAGES.len(), LEVEL_SIZE * 2);
        for passage in PASSAGES {
            assert!(!passage.text.is_empty());
            for question in passage.questions {
                assert!(question.correct < question.choices.len());
                for (i, choice) in question.choices.iter().enumerate() {
                    assert!(!question.choices[..i].contains(choice));
                }
            }
        }
    }

    #[test]
    fn passage_return_and_other_lessons_preserve_progress() {
        let mut study = Study::default();
        assert!(study.open(0));
        assert!(!study.answer(1));
        study.show_passage(false);
        assert!(study.answer(1));
        study.show_passage(true);
        assert!(!study.advance());
        assert!(study.open(1));
        assert!(study.open(0));
        let mut restored = Study::unpack(&study.pack()).unwrap();
        assert_eq!(restored.current().unwrap().selected(), Some(1));
        restored.show_passage(false);
        assert!(restored.advance());
        assert_eq!(restored.current().unwrap().question(), Some(1));
    }

    #[test]
    fn retry_contains_only_missed_questions() {
        let mut study = Study::default();
        assert!(study.open(0));
        study.show_passage(false);
        for answer in [1, 1, 2] {
            assert!(study.answer(answer));
            assert!(study.advance());
        }
        assert_eq!(study.current().unwrap().score(&PASSAGES[0]), (2, 3));
        assert!(study.restart(true));
        assert_eq!(study.current().unwrap().question(), Some(1));
        assert_eq!(study.current().unwrap().position(), (1, 1));
    }
}
