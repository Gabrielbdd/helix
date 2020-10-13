use crate::commands;
use crate::View;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::string::String;

pub struct Prompt {
    pub buffer: String,
    pub cursor_loc: usize,
}

impl Prompt {
    pub fn new() -> Prompt {
        let prompt = Prompt {
            buffer: String::from(""),
            cursor_loc: 0,
        };
        prompt
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_loc, c);
        self.cursor_loc += 1;
    }

    pub fn move_char_left(&mut self) {
        if self.cursor_loc > 1 {
            self.cursor_loc -= 1;
        }
    }

    pub fn move_char_right(&mut self) {
        if self.cursor_loc < self.buffer.len() {
            self.cursor_loc += 1;
        }
    }

    pub fn move_start(&mut self) {
        self.cursor_loc = 0;
    }

    pub fn move_end(&mut self) {
        self.cursor_loc = self.buffer.len();
    }

    pub fn delete_char_backwards(&mut self) {
        if self.cursor_loc > 0 {
            self.buffer.remove(self.cursor_loc - 1);
            self.cursor_loc -= 1;
        }
    }

    pub fn success_fn() {
        // TODO:
    }

    pub fn handle_input(&mut self, key_event: KeyEvent, view: &mut View) {
        match key_event {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
            } => self.insert_char(c),
            KeyEvent {
                code: KeyCode::Esc, ..
            } => commands::normal_mode(view, 1),
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => self.move_char_right(),
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => self.move_char_left(),
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
            } => self.move_end(),
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            } => self.move_start(),
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => self.delete_char_backwards(),
            _ => (),
        }
    }
}