use std::{fs, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

use crate::models::{Status, Task};
use dirs::home_dir;

pub fn run(project: &str) {
    let path = home_dir()
        .unwrap()
        .join(".pmcli")
        .join(project)
        .join("tasks.json");

    let mut tasks: Vec<Task> = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut selected: usize = 0;
    let mut search = String::new();
    let mut search_mode = false;

    loop {
        let filtered: Vec<&Task> = if search.is_empty() {
            tasks.iter().collect()
        } else {
            tasks
                .iter()
                .filter(|t| t.description.to_lowercase().contains(&search))
                .collect()
        };

        if selected >= filtered.len() && !filtered.is_empty() {
            selected = filtered.len() - 1;
        }

        terminal
            .draw(|f| {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(1)])
                    .split(f.size());

                let body = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
                    .split(layout[1]);

                // ===== SEARCH BAR =====
                let search_bar = Paragraph::new(if search_mode {
                    format!("🔍 /{}", search)
                } else {
                    "Press / to search | ↑↓ Enter b q".into()
                })
                .block(Block::default().borders(Borders::ALL).title("Search"));

                f.render_widget(search_bar, layout[0]);

                // ===== TASK LIST =====
                let items: Vec<ListItem> = filtered
                    .iter()
                    .map(|t| {
                        let color = match t.status {
                            Status::Todo => Color::White,
                            Status::Done => Color::Green,
                            Status::Blocked => Color::Red,
                        };

                        ListItem::new(format!(
                            "[{}] {:<8} {}",
                            t.id,
                            format!("{:?}", t.status),
                            t.description
                        ))
                        .style(Style::default().fg(color))
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().title("Tasks").borders(Borders::ALL))
                    .highlight_style(Style::default().bg(Color::Blue));

                let mut state = ratatui::widgets::ListState::default();
                state.select(Some(selected));
                f.render_stateful_widget(list, body[0], &mut state);

                // ===== DETAIL PANEL =====
                if let Some(task) = filtered.get(selected) {
                    let detail = Paragraph::new(format!(
                        "ID       : {}\n\
                     Status   : {:?}\n\
                     Priority : {}\n\
                     Deadline : {}\n\
                     Owner    : {}\n\n\
                     Description:\n{}",
                        task.id,
                        task.status,
                        task.priority,
                        task.deadline.map(|d| d.to_string()).unwrap_or("—".into()),
                        task.owner,
                        task.description
                    ))
                    .wrap(Wrap { trim: false })
                    .block(Block::default().title("Detail").borders(Borders::ALL));

                    f.render_widget(detail, body[1]);
                }
            })
            .unwrap();

        if event::poll(Duration::from_millis(200)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') if !search_mode => break,

                    KeyCode::Char('/') if !search_mode => {
                        search_mode = true;
                        search.clear();
                    }

                    KeyCode::Esc if search_mode => {
                        search_mode = false;
                        search.clear();
                        selected = 0;
                    }

                    KeyCode::Backspace if search_mode => {
                        search.pop();
                        selected = 0;
                    }

                    KeyCode::Char(c) if search_mode => {
                        search.push(c.to_ascii_lowercase());
                        selected = 0;
                    }

                    KeyCode::Down if !search_mode => {
                        if selected + 1 < filtered.len() {
                            selected += 1;
                        }
                    }

                    KeyCode::Up if !search_mode => {
                        selected = selected.saturating_sub(1);
                    }

                    KeyCode::Enter if !filtered.is_empty() && !search_mode => {
                        let id = filtered[selected].id;
                        if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                            t.status = match t.status {
                                Status::Todo => Status::Done,
                                Status::Done => Status::Todo,
                                Status::Blocked => Status::Blocked,
                            };
                            save(&path, &tasks);
                        }
                    }

                    KeyCode::Char('b') if !filtered.is_empty() && !search_mode => {
                        let id = filtered[selected].id;
                        if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                            t.status = match t.status {
                                Status::Blocked => Status::Todo,
                                _ => Status::Blocked,
                            };
                            save(&path, &tasks);
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().unwrap();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    terminal.show_cursor().unwrap();
}

fn save(path: &std::path::Path, tasks: &Vec<Task>) {
    fs::write(path, serde_json::to_string_pretty(tasks).unwrap()).unwrap();
}
