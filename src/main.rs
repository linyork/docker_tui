#![allow(clippy::cognitive_complexity)]
use std::io::{self, Write};
use std::process::Command as StdCommand;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

const MENU: &str = r#"__   __            _
\ \ / /___   _ __ | | __ ___  _ __
 \ V // _ \ | '__|| |/ // _ \| '__|
  | || (_) || |   |   <|  __/| |
  |_| \___/ |_|   |_|\_\\___||_|   v0.1.0
-------------------------------------------
l.      顯示所有容器
a.      開啟所有容器 (docker-compose.yml)
c.      關閉所有容器
q.      離開
"#;

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;

        for line in MENU.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match read_char()? {
            'l' => {
                StdCommand::new("docker")
                    .arg("ps")
                    .arg("-a")
                    .arg("--format")
                    .arg("table \r{{.Names}}\t{{.Status}}\t{{.Ports}}\t{{.ID}}")
                    .spawn()
                    .expect("Show container error.");
            }
            'a' => {
                StdCommand::new("docker-compose")
                    .arg("up")
                    .arg("-d")
                    .spawn()
                    .expect("docker-compose up error.");
            }
            'c' => {
                StdCommand::new("docker-compose")
                    .arg("down")
                    .spawn()
                    .expect("docker-compose down error.");
            }
            'q' => break,
            _ => {}
        };
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

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

fn main() -> Result<()> {
    let mut stderr = io::stdout();
    run(&mut stderr)
}
