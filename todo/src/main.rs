mod headers;

use headers::*;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{fs, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks: Vec<Task> = Vec::new();
    let mut input = String::new();
    let mut show_help = true;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(2),
                ])
                .split(f.size());

            let input_box = Paragraph::new(input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input_box, chunks[0]);

            if show_help {
                let help_text = vec![
                    Spans::from(vec![Span::styled(
                        "To-Do App Help:",
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    )]),
                    Spans::from("Commands:"),
                    Spans::from("  - todo add <task1> <task2> ...: Add tasks"),
                    Spans::from("  - todo done <id1> <id2> ...: Mark tasks as done"),
                    Spans::from("  - todo export: Save tasks to tasks.txt"),
                    Spans::from("Press Tab to switch to tasks view."),
                ];
                let help_widget = Paragraph::new(help_text)
                    .block(Block::default().borders(Borders::ALL).title("Help"));
                f.render_widget(help_widget, chunks[1]);
            } else {
                let tasks_list: Vec<ListItem> = tasks
                    .iter()
                    .map(|task| {
                        let task_text = if task.is_done {
                            Spans::from(vec![
                                Span::styled(
                                    format!("[{}] ", task.id),
                                    Style::default().fg(Color::Blue),
                                ),
                                Span::styled(
                                    &task.description,
                                    Style::default()
                                        .fg(Color::Green)
                                        .add_modifier(Modifier::CROSSED_OUT),
                                ),
                            ])
                        } else {
                            Spans::from(vec![
                                Span::styled(
                                    format!("[{}] ", task.id),
                                    Style::default().fg(Color::Blue),
                                ),
                                Span::styled(&task.description, Style::default().fg(Color::White)),
                            ])
                        };
                        ListItem::new(task_text)
                    })
                    .collect();
                let tasks_widget = List::new(tasks_list)
                    .block(Block::default().borders(Borders::ALL).title("Tasks"));
                f.render_widget(tasks_widget, chunks[1]);
            }
        })?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        handle_command(&input, &mut tasks);
                        input.clear();
                    }
                    KeyCode::Tab => {
                        show_help = !show_help;
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_command(input: &str, tasks: &mut Vec<Task>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.as_slice() {
        ["todo", "add", task_descriptions @ ..] => {
            let descriptions: Vec<String> =
                task_descriptions.iter().map(|&s| s.to_string()).collect();
            add_multiple_tasks(tasks, &descriptions);
        }
        ["todo", "done", task_ids @ ..] => {
            let ids: Vec<usize> = task_ids.iter().filter_map(|id| id.parse().ok()).collect();
            mark_tasks_done(tasks, &ids);
        }
        ["todo", "export"] => {
            export_tasks(tasks);
        }
        _ => {}
    }
}

fn export_tasks(tasks: &[Task]) {
    let mut content = String::new();
    for task in tasks {
        let status = if task.is_done { "Done" } else { "Pending" };
        content.push_str(&format!("{}: {} [{}]\n", task.id, task.description, status));
    }
    if let Err(e) = fs::write("tasks.txt", content) {
        eprintln!("Error saving tasks: {}", e);
    }
}