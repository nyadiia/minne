use std::io;
use std::io::Read;
use std::time::Instant;
use termion::style;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::Terminal;
struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
struct App<'a> {
    items: StatefulList<(&'a str, usize)>,
    events: Vec<(&'a str, &'a str)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
            events: vec![
                ("Event1", "INFO"),
                ("Event2", "INFO"),
                ("Event3", "CRITICAL"),
                ("Event4", "ERROR"),
                ("Event5", "INFO"),
                ("Event6", "INFO"),
                ("Event7", "WARNING"),
                ("Event8", "INFO"),
                ("Event9", "INFO"),
                ("Event10", "INFO"),
                ("Event11", "CRITICAL"),
                ("Event12", "INFO"),
                ("Event13", "INFO"),
                ("Event14", "INFO"),
                ("Event15", "INFO"),
                ("Event16", "INFO"),
                ("Event17", "ERROR"),
                ("Event18", "ERROR"),
                ("Event19", "INFO"),
                ("Event20", "INFO"),
                ("Event21", "WARNING"),
                ("Event22", "INFO"),
                ("Event23", "INFO"),
                ("Event24", "WARNING"),
                ("Event25", "INFO"),
                ("Event26", "INFO"),
            ],
        }
    }
}
fn main() -> Result<(), io::Error> {
    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    let mut cur_color = Color::White;
    let mut app = App::new();
    // Clear the terminal before first draw.
    terminal.clear()?;
    loop {
        // Lock the terminal and start a drawing session.
        terminal.draw(|frame| {
            // Create a layout into which to place our blocks.
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(4)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(frame.size());
            // let left_chunks = Layout::default()
            //     .direction(Direction::Horizontal)
            //     .constraints([Constraint::Percentage(100)].as_ref())
            //     .split(chunks[0]);

            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(i.0)];
                    for _ in 0..i.1 {
                        lines.push(Spans::from(Span::styled(
                            "funny content",
                            Style::default().add_modifier(Modifier::ITALIC),
                        )));
                    }
                    ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Reset))
                })
                .collect();

            let items = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(" List "),
                )
                .style(Style::default().fg(Color::White).bg(Color::Reset))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );

            // We can now render the item list
            frame.render_stateful_widget(items, chunks[0], &mut app.items.state);

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            // TODO: Change to list type and be able to tab between the different boxes
            // The text lines for our text box.
            let today_txt = vec![Spans::from(
                "The box above will change colors every three seconds.\n",
            )];
            // Create a paragraph with the above text...
            let today_view = Paragraph::new(today_txt)
                // In a block with borders and the given title...
                .block(
                    Block::default()
                        .title(" Today View ")
                        .title_alignment(tui::layout::Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::White).bg(Color::Reset));

            // Render into the second chunk of the layout.
            frame.render_widget(today_view, right_chunks[0]);
            let details_txt = vec![Spans::from(
                "The box above will change colors every three seconds.\n",
            )];

            let detail_view = Paragraph::new(details_txt)
                // In a block with borders and the given title...
                .block(
                    Block::default()
                        .title("| Details |")
                        .title_alignment(tui::layout::Alignment::Center)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                // With white foreground and black background...
                .style(Style::default().fg(Color::White).bg(Color::Reset));

            // Render into the second chunk of the layout.
            frame.render_widget(detail_view, right_chunks[1]);
        })?;

        // Iterate over all the keys that have been pressed since the
        // last time we checked.
        for k in asi.by_ref().keys() {
            match k.unwrap() {
                // If any of them is q, quit
                Key::Char('q') => {
                    // Clear the terminal before exit so as not to leave
                    // a mess.
                    terminal.clear()?;
                    return Ok(());
                }
                Key::Up => {
                    app.items.previous();
                }
                Key::Down => {
                    app.items.next();
                }
                Key::Right => {
                    app.items.unselect();
                }
                // Otherwise, throw them away.
                _ => (),
            }
        }
    }
}
