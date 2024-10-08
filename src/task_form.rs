use anyhow::Context;
use anyhow::Result;

use crate::configuration::Settings;
use crate::repeat::Repeat;
use crate::task::Task;
use crate::utils;

#[derive(Default)]
pub struct TaskForm {
    pub id: Option<usize>,
    _name: String,
    _date: String,
    _repeats: String,
    _group: String,
    _description: String,
    _url: String,
    fields: Vec<fn(&TaskForm) -> &String>,
    mut_fields: Vec<fn(&mut TaskForm) -> &mut String>,
    current_field_index: usize,
    cursor_pos: usize,
}

impl TaskForm {
    pub fn new() -> Self {
        let mut_fields = vec![
            TaskForm::mut_name,
            TaskForm::mut_date,
            TaskForm::mut_repeats,
            TaskForm::mut_group,
            TaskForm::mut_description,
            TaskForm::mut_url,
        ];
        let fields = vec![
            TaskForm::name,
            TaskForm::date,
            TaskForm::repeats,
            TaskForm::group,
            TaskForm::description,
            TaskForm::url,
        ];

        Self {
            id: None,
            _name: String::new(),
            _date: String::new(),
            _repeats: String::new(),
            _group: String::new(),
            _description: String::new(),
            _url: String::new(),
            fields,
            mut_fields,
            current_field_index: 0,
            cursor_pos: 0,
        }
    }

    pub fn from_task(task: &Task, settings: &Settings) -> Self {
        let mut_fields = vec![
            TaskForm::mut_name,
            TaskForm::mut_date,
            TaskForm::mut_repeats,
            TaskForm::mut_group,
            TaskForm::mut_description,
            TaskForm::mut_url,
        ];
        let fields = vec![
            TaskForm::name,
            TaskForm::date,
            TaskForm::repeats,
            TaskForm::group,
            TaskForm::description,
            TaskForm::url,
        ];
        Self {
            id: task.id,
            _name: task.name.to_string(),
            _date: utils::date_to_input_str(&task.date, settings),
            _repeats: task.repeats.to_string(),
            _group: task.group.clone().unwrap_or_default(),
            _description: task.description.clone().unwrap_or_default(),
            _url: task.url.clone().unwrap_or_default(),
            fields,
            mut_fields,
            current_field_index: 0,
            cursor_pos: task.name.len(),
        }
    }

    pub fn submit(&mut self, settings: &Settings) -> Result<Task> {
        let mut task = Task::default();

        let repeat = Repeat::parse_from_str(&self._repeats).context("Invalid repeat format")?;
        let date = utils::parse_date(&self._date, settings).unwrap_or(utils::get_today());

        if self._name.is_empty() {
            return Err(anyhow::anyhow!("Task name cannot be empty"));
        }

        task.set_id(self.id);
        task.set_name(self._name.clone());
        task.set_date(date);
        task.set_repeats(repeat);
        if !self._group.is_empty() {
            task.set_group(self._group.clone());
        }
        if !self._description.is_empty() {
            task.set_description(self._description.clone());
        }
        if !self._url.is_empty() {
            task.set_url(self._url.clone());
        }

        Ok(task)
    }

    pub fn next_field(&mut self) {
        if self.current_field_index < self.num_fields() - 1 {
            self.current_field_index += 1;
            self.cursor_pos = self.current_field().len();
        }
    }

    pub fn prev_field(&mut self) {
        if self.current_field_index > 0 {
            self.current_field_index -= 1;
            self.cursor_pos = self.current_field().len();
        }
    }

    pub fn move_cursor(&mut self, diff: isize) {
        if diff < 0 && self.cursor_pos <= diff.abs() as usize {
            self.cursor_pos = 0;
            return;
        }

        self.cursor_pos = self
            .current_field()
            .len()
            .min((self.cursor_pos as isize + diff) as usize);
    }

    pub fn add_char(&mut self, c: char) {
        let cursor_pos = self.cursor_pos;
        self.mut_current_field().insert(cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        let remove_idx = self.cursor_pos - 1;
        self.mut_current_field().remove(remove_idx);
        if self.current_field().is_empty() {
            self.cursor_pos = 0;
        } else {
            self.cursor_pos -= 1;
        }
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor_pos
    }

    fn mut_current_field(&mut self) -> &mut String {
        self.mut_fields[self.current_field_index](self)
    }

    pub fn current_field(&self) -> &String {
        self.fields[self.current_field_index](self)
    }

    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn current_field_index(&self) -> usize {
        self.current_field_index
    }

    pub fn date(&self) -> &String {
        &self._date
    }

    pub fn name(&self) -> &String {
        &self._name
    }

    pub fn repeats(&self) -> &String {
        &self._repeats
    }

    pub fn group(&self) -> &String {
        &self._group
    }

    pub fn url(&self) -> &String {
        &self._url
    }

    pub fn description(&self) -> &String {
        &self._description
    }

    fn mut_date(&mut self) -> &mut String {
        &mut self._date
    }

    fn mut_name(&mut self) -> &mut String {
        &mut self._name
    }

    fn mut_repeats(&mut self) -> &mut String {
        &mut self._repeats
    }

    fn mut_group(&mut self) -> &mut String {
        &mut self._group
    }

    fn mut_description(&mut self) -> &mut String {
        &mut self._description
    }

    fn mut_url(&mut self) -> &mut String {
        &mut self._url
    }
}
