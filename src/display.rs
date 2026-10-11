use std::io::{self, stdout, IsTerminal, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{
    self, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{execute, queue};
use unicode_width::UnicodeWidthStr;

use crate::cli::Cli;
use crate::frames::{self, Animation};

const HINT: &str = "q / Esc / Ctrl-C to close";

pub fn run(cli: &Cli) -> io::Result<()> {
    let anim = frames::load();
    let mut out = stdout();

    if !out.is_terminal() {
        print_static(&anim);
        return Ok(());
    }

    let (cols, rows) = terminal::size()?;
    let block_w = anim.width.max(UnicodeWidthStr::width(HINT));
    if (cols as usize) < block_w + 2 || (rows as usize) < anim.height + 2 {
        println!("terminal too small for the cat (need ~36x22)");
        return Ok(());
    }

    terminal::enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, DisableLineWrap, Hide)?;

    let running = Arc::new(AtomicBool::new(true));
    spawn_input_listener(Arc::clone(&running));

    let start_x = (cols as usize).saturating_sub(anim.width) / 2;
    let start_y = (rows as usize).saturating_sub(anim.height) / 2;
    let frame_dur = Duration::from_micros(1_000_000 / cli.fps.max(1) as u64);

    let mut played: u32 = 0;
    'outer: loop {
        for frame in &anim.frames {
            if !sleep_responsive(frame_dur, &running) {
                break 'outer;
            }
            draw(&mut out, start_x, start_y, frame, rows as usize)?;
        }
        played += 1;
        if cli.repeat != 0 && played >= cli.repeat {
            break;
        }
    }

    execute!(out, Show, EnableLineWrap, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw(
    out: &mut io::Stdout,
    start_x: usize,
    start_y: usize,
    frame: &[&str],
    rows: usize,
) -> io::Result<()> {
    let term_w = terminal::size()?.0 as usize;

    for (y, line) in (start_y..).zip(frame.iter().copied()) {
        queue!(
            out,
            MoveTo(start_x as u16, y as u16),
            Clear(ClearType::UntilNewLine),
            Print(line)
        )?;
    }

    let hx = term_w.saturating_sub(UnicodeWidthStr::width(HINT)) / 2;
    queue!(
        out,
        MoveTo(hx as u16, (rows - 1) as u16),
        SetAttribute(Attribute::Dim),
        Clear(ClearType::UntilNewLine),
        Print(HINT),
        SetAttribute(Attribute::Reset)
    )?;

    out.flush()
}

fn print_static(anim: &Animation) {
    if let Some(frame) = anim.frames.first() {
        for line in frame {
            println!("{line}");
        }
    }
}

fn spawn_input_listener(running: Arc<AtomicBool>) {
    thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            if event::poll(Duration::from_millis(20)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    let quit = matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                        || (key.code == KeyCode::Char('c')
                            && key.modifiers.contains(KeyModifiers::CONTROL));
                    if quit {
                        running.store(false, Ordering::SeqCst);
                        return;
                    }
                }
            }
        }
    });
}

fn sleep_responsive(dur: Duration, running: &AtomicBool) -> bool {
    let step = Duration::from_millis(5);
    let mut left = dur;
    while left > Duration::ZERO {
        if !running.load(Ordering::SeqCst) {
            return false;
        }
        let s = left.min(step);
        thread::sleep(s);
        left = left.saturating_sub(s);
    }
    running.load(Ordering::SeqCst)
}
