#![allow(clippy::cognitive_complexity)]

use std::io::{self, Write};

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

#[macro_use]
mod macros;
// mod test;

const MENU: &str = r#"__   __            _
\ \ / /___   _ __ | | __ ___  _ __
 \ V // _ \ | '__|| |/ // _ \| '__|
  | || (_) || |   |   <|  __/| |
  |_| \___/ |_|   |_|\_\\___||_|   v0.1.0
-------------------------------------------
l.      顯示所有容器
q.      離開
"#;

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    loop {
        // queue!(
        //     w,
        //     style::ResetColor,
        //     terminal::Clear(ClearType::All),
        //     cursor::Hide,
        //     cursor::MoveTo(1, 1)
        // )?;

        for line in MENU.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match read_char()? {
            // '1' => test::cursor::run(w)?,
            // '2' => test::color::run(w)?,
            // '3' => test::attribute::run(w)?,
            // '4' => test::event::run(w)?,
            'q' => break,
            _ => {}
        };
    }

    // execute!(
    //     w,
    //     style::ResetColor,
    //     cursor::Show,
    //     terminal::LeaveAlternateScreen
    // )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
