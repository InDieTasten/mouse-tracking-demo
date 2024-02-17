use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut mouse_row = 0;
    let mut mouse_column = 0;

    loop {
        let message = format!(
            "Press q to exit\nMouse position: {} {}",
            mouse_row, mouse_column
        );

        terminal.draw(|frame| {
            let full_area = frame.size();

            let mouse_area = Rect::new(mouse_column, mouse_row, 1, 1);

            frame.render_widget(Paragraph::new(message).white().on_black(), full_area);
            frame.render_widget(Paragraph::new("+").black().on_red(), mouse_area);
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            let ev = event::read()?;
            if let event::Event::Key(key) = ev {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
            if let event::Event::Mouse(mouse) = ev {
                if mouse.kind == MouseEventKind::Moved {
                    mouse_row = mouse.row;
                    mouse_column = mouse.column;
                }
            }
        }
    }

    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
