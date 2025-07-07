use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    layout::Alignment, prelude::*, style::{Color, Stylize}, widgets::{Block, BorderType, Borders, Paragraph}, Frame, Terminal
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

const GITHUB: &str = "https://www.github.com/Ahbar1999";
const LEETCODE: &str = "https://www.leetcode.com/Ahbar";
const CODEFORCES: &str = "https://www.codeforces.com/ahbr";
const LINKEDIN: &str = "https://www.linkedin.com/in/ahbar-siddiqui-a603a8158";

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    counter: RefCell<u8>,
}

impl App {
    fn render(&self, frame: &mut Frame) {
        // let counter = self.counter.borrow();

        let layout1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(66),
            ])
            .split(frame.area());

        // layout3 is second part of layout2
        let layout2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(layout1[1]);

        
        let block1 = Block::bordered()
            .cyan()
            .title("About Me")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let block2 = Block::bordered()
            .cyan()
            .title("Education")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let block3 = Block::bordered()
            .cyan()
            .title("Profiles")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text1 = format!(
            "Hey, I'm Ahbar, currently a Master's student in Computer Science and Engineering @ IIT Guwahati.",
        );

        let para1 = Paragraph::new(text1)
            .block(block1)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        let text2 = format!(
            "\n\n\n\nMaster's(M.Tech.):     IIT Guwahati\n\n\n\nBachelors's(B.Tech.):   ZHCET, AMU"
        );

        let para2 = Paragraph::new(text2)
            .block(block2)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        let text2 = format!(
            "\n\n\nGithub: {}\n\n\nLeetcode: {}\n\n\nCodeforces: {}\n\n\nLinkedIn: {}\n\n\n",
            &GITHUB,
            &LEETCODE,
            &CODEFORCES,
            &LINKEDIN
        );

        let para3 = Paragraph::new(text2)
            .block(block3)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();
        
        frame.render_widget(
            para1
            ,layout1[0]);

        frame.render_widget(
            para2,
            layout2[0]);

        frame.render_widget(
            para3,
            layout2[1]
        );
    }

    fn handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            _ => {}
        }
    }
}
