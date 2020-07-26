use std::{
    io::{stdout, Write},
    thread, time,
};
use termion::{async_stdin, clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Debug)]
pub enum KeyEvent {
    Left,
    Right,
    Up,
    Down,
}

pub fn mount_terminal(f: &mut dyn FnMut(Option<KeyEvent>) -> String) {
    let mut stdin = async_stdin().keys();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    println!("{}{}{}", cursor::Hide, clear::All, cursor::Goto(1, 1));

    loop {
        let b = stdin.next();
        let key_event = match b {
            Some(Ok(Key::Char('q'))) => break,
            Some(Ok(Key::Left)) => Some(KeyEvent::Left),
            Some(Ok(Key::Right)) => Some(KeyEvent::Right),
            Some(Ok(Key::Up)) => Some(KeyEvent::Up),
            Some(Ok(Key::Down)) => Some(KeyEvent::Down),
            _ => None,
        };
        let out = f(key_event);
        write!(stdout, "{}{}{}\r\n", clear::All, cursor::Goto(1, 1), out).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}
