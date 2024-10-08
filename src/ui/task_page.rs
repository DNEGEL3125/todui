use crate::{app::App, configuration::KeyBindings, key, task_form::TaskForm};
use std::{cell::RefCell, rc::Rc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use super::{InputMode, Page};

pub struct TaskPage {
    task_form: TaskForm,
    pub input_mode: InputMode,
    pub editing_task: Option<usize>,
    pub error: Option<String>,
    pub app: Rc<RefCell<App>>,
}

impl TaskPage {
    pub fn new(app: Rc<RefCell<App>>) -> TaskPage {
        TaskPage {
            task_form: TaskForm::default(),
            input_mode: InputMode::Normal,
            error: None,
            editing_task: None,
            app,
        }
    }

    pub fn new_from_task(app: Rc<RefCell<App>>, task_id: usize) -> TaskPage {
        let task = app.borrow().get_task(task_id).unwrap().clone();
        let task_form = TaskForm::from_task(&task, &app.borrow().settings);

        TaskPage {
            task_form,
            input_mode: InputMode::Normal,
            error: None,
            editing_task: Some(task_id),
            app,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.task_form.add_char(c);
    }

    pub fn remove_char(&mut self) {
        self.task_form.remove_char();
    }

    pub fn next_field(&mut self) {
        self.task_form.next_field();
    }

    pub fn prev_field(&mut self) {
        self.task_form.prev_field();
    }

    pub fn move_cursor(&mut self, diff: isize) {
        self.task_form.move_cursor(diff);
    }

    pub fn submit(&mut self) -> bool {
        let mut app = self.app.borrow_mut();
        let settings = &app.settings;
        let form_result = self.task_form.submit(settings);
        match form_result {
            Ok(new_task) => {
                if let Some(task_id) = self.editing_task {
                    app.delete_task(task_id);
                }
                app.add_task(new_task);
                true
            }
            Err(e) => {
                self.error = Some(e.to_string());
                false
            }
        }
    }

    fn border_style(&self, idx: usize) -> Style {
        if self.task_form.current_field_index() == idx && self.input_mode == InputMode::Insert {
            Style::default().fg(self.get_primary_color())
        } else {
            Style::default()
        }
    }

    fn get_date_hint(&self) -> String {
        let date_hint = self
            .app
            .borrow()
            .settings
            .date_formats
            .input_date_hint
            .clone();
        let datetime_hint = self
            .app
            .borrow()
            .settings
            .date_formats
            .input_datetime_hint
            .clone();
        format!("{} or {}", date_hint, datetime_hint)
    }

    fn get_keybind_hint(&self) -> Spans {
        let color = self.get_secondary_color();
        let kb = &self.app.borrow().settings.keybindings;
        let i = key!(kb.enter_insert_mode, color);
        let q = key!(kb.quit, color);
        let j = key!(kb.down, color);
        let k = key!(kb.up, color);
        let enter = key!(kb.save_changes, color);
        let esc = key!(kb.enter_normal_mode, color);
        let b = key!(kb.go_back, color);

        Spans::from(vec![
            Span::raw("Press "),
            i,
            Span::raw(" to enter insert mode, "),
            q,
            Span::raw(" to quit, "),
            k,
            Span::raw(" and "),
            j,
            Span::raw(" to move up and down, "),
            enter,
            Span::raw(" to save, "),
            esc,
            Span::raw(" to exit input mode, and "),
            b,
            Span::raw(" to go back to the main screen. (*) Fields are required."),
        ])
    }

    pub fn get_primary_color(&self) -> Color {
        self.app.borrow().settings.colors.primary_color
    }

    pub fn get_secondary_color(&self) -> Color {
        self.app.borrow().settings.colors.secondary_color
    }
}

impl<B> Page<B> for TaskPage
where
    B: Backend,
{
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        // Draw border around area
        let border_style = match focused {
            true => Style::default().fg(self.get_primary_color()),
            false => Style::default(),
        };
        let border_type = match focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Task")
            .border_style(border_style)
            .border_type(border_type);
        f.render_widget(block, area);

        // Keybinds description paragraph
        let keybinds = Paragraph::new(self.get_keybind_hint())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(keybinds, chunks[0]);

        // Name
        let curr_text = self.task_form.name().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(0))
            .block(Block::default().borders(Borders::ALL).title("Name (*)"));
        f.render_widget(input, chunks[1]);

        // Date
        let curr_text = self.task_form.date().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(1))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Date ({})", self.get_date_hint())),
            );
        f.render_widget(input, chunks[2]);

        // Repeats
        let curr_text = self.task_form.repeats().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(2))
            .block(Block::default().borders(Borders::ALL).title(
                "Repeats (Never | Daily | Weekly | Monthly | Yearly | Mon,Tue,Wed,Thu,Fri,Sat,Sun)",
            ));
        f.render_widget(input, chunks[3]);

        // Group
        let curr_text = self.task_form.group().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(3))
            .block(Block::default().borders(Borders::ALL).title("Group"));
        f.render_widget(input, chunks[4]);

        // Description
        let curr_text = self.task_form.description().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(4))
            .block(Block::default().borders(Borders::ALL).title("Description"));
        f.render_widget(input, chunks[5]);

        // Description
        let curr_text = self.task_form.url().clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(5))
            .block(Block::default().borders(Borders::ALL).title("URL"));
        f.render_widget(input, chunks[6]);

        let current_field_index = self.task_form.current_field_index();

        // Place cursor
        if focused {
            f.set_cursor(
                chunks[current_field_index + 1].x + 1 + self.task_form.cursor_pos() as u16,
                chunks[current_field_index + 1].y + 1,
            );
        }

        // Error message
        if let Some(error) = &self.error {
            let error = Paragraph::new(error.as_ref())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, chunks[7]);
        }
    }
}
