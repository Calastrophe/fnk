use super::canvas::Canvas;
use crate::api::{
    test::{get_questions, register_student, set_level, Question},
    APIError,
};
use dioxus::prelude::*;

struct TestState {
    state: State,
    level: i32,
    attempt: i32,
    has_drawn: bool,
}

pub enum Action {
    Next,
    LevelUp,
}

enum State {
    Registration,
    Testing,
    Finished,
}

impl TestState {
    pub fn new() -> Self {
        TestState {
            state: State::Registration,
            level: 1,
            attempt: 1,
            has_drawn: false,
        }
    }

    pub fn perform_action(&mut self, action: Action) {
        match action {
            Action::Next => match self.attempt {
                1..=2 => self.attempt += 1,
                3 => self.state = State::Finished,
                _ => unreachable!(),
            },
            Action::LevelUp => match self.level {
                1..=7 => {
                    self.level += 1;
                    self.attempt = 1;
                }
                _ => self.state = State::Finished,
            },
        }
    }
}

#[inline_props]
pub fn Test(cx: Scope, id: String) -> Element {
    let _ = use_shared_state_provider(cx, || TestState::new());
    let test_state = use_shared_state::<TestState>(cx).unwrap();

    cx.render(match test_state.read().state {
        State::Testing => {
            rsx! {
                QuestionBar { level: test_state.read().level, attempt: test_state.read().attempt }

                Canvas {
                    ondraw: move |_| test_state.write().has_drawn = true,
                    onclear: move |_| test_state.write().has_drawn = false,
                }

                button { onclick: move |_| test_state.write().perform_action(Action::Next) }
                button { onclick: move |_| test_state.write().perform_action(Action::LevelUp) }
            }
        }
        State::Registration => {
            rsx! { Registration { id: id.clone() }
            }
        }
        State::Finished => {
            rsx! { Finished { id: id.clone(), level: test_state.read().level } }
        }
    })
}

#[inline_props]
fn QuestionBar(cx: Scope, level: i32, attempt: i32) -> Element {
    let questions = use_future(cx, level, |level| async move { get_questions(level).await });

    let question = match questions.value() {
        Some(Ok(questions)) => {
            let question = &questions[(*attempt - 1) as usize].question;

            // TODO: Implement image in question, if needed
            rsx! { div { "{question}" } }
        }
        Some(Err(_)) => rsx! { div { "There was an error fetching questions..." } },
        None => rsx! { div { "Fetching a question..." } },
    };

    cx.render(question)
}

#[inline_props]
fn Registration(cx: Scope, id: String) -> Element {
    let resp_text = use_state(cx, || String::new());
    let test_state = use_shared_state::<TestState>(cx).unwrap();

    let onsubmit = move |evt: FormEvent| {
        to_owned![resp_text];
        to_owned![test_state];
        to_owned![id];

        cx.spawn(async move {
            let resp = register_student(&id, evt.values["name"][0].as_str()).await;

            match resp {
                Err(e) => match e {
                    APIError::Validation(validation_errs) => {
                        resp_text.set(validation_errs.join(", "))
                    }
                    _ => resp_text.set(e.to_string()),
                },

                Ok(_) => test_state.write().state = State::Testing,
            }
        });
    };

    render! {
        h1 { "Register" }
        form {
            onsubmit: onsubmit,
            div { "Name: " }
            input { r#type: "text", name: "name" }
            br {}
            br {}
            button { "Submit" }
            br {}
            div { "{resp_text}" }
        }
    }
}

#[inline_props]
fn Finished(cx: Scope, id: String, level: i32) -> Element {
    let resp = use_future(cx, (id, level), |(id, level)| async move {
        set_level(&id, level).await
    });

    cx.render(match resp.value() {
        Some(Ok(_)) => {
            rsx! { "Thank you, your score has been submitted." }
        }
        Some(Err(e)) => {
            rsx! { "There was an error submitting your score... {e}" }
        }
        None => {
            rsx! { "Submitting your score..." }
        }
    })
}
