use ratatui::{text::Text, Frame};
use crossterm::event::{self, Event};

// Every application built with ratatui needs to implement the following steps:
//
//     Initialize the terminal
//     A main loop that:
//         Draws the UI
//         Handles input events
//     Restore the terminal state

pub fn basic_tui () {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw2).expect("Failed to draw frame");
        if matches!(event::read().expect("failed to read event"), Event::Key(_)){
            break;
        }
    }
    ratatui::restore();
}

fn draw (frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text,frame.area());
}


// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
//
// fn handle_events() -> std::io::Result<bool> {
//     match event::read()? {
//         Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
//             KeyCode::Char('q') => return Ok(true),
//             // handle other key events
//             _ => {}
//         },
//         // handle other events
//         _ => {}
//     }
//     Ok(false)
// }

use ratatui::{layout::{Constraint, Layout}, widgets::Block};

fn draw2 (frame: &mut Frame){
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0),Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
