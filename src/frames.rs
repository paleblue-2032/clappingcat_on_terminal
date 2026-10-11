//! 埋め込んだ ASCII フレーム群。
//!
//! `assets/frames.txt` は `tools/render.py` が生成する。
//! 各フレームは `\x0c` (form feed) 区切り、行は `\n` 区切り。

const FRAMES_RAW: &str = include_str!("../assets/frames.txt");

pub struct Animation {
    pub frames: Vec<Vec<&'static str>>,
    pub width: usize,
    pub height: usize,
}

pub fn load() -> Animation {
    let frames: Vec<Vec<&'static str>> = FRAMES_RAW
        .split('\x0c')
        .map(|frame| {
            let mut lines: Vec<&'static str> = frame.split('\n').collect();
            while lines.last() == Some(&"") {
                lines.pop();
            }
            lines
        })
        .filter(|frame| !frame.is_empty())
        .collect();

    let height = frames.iter().map(|f| f.len()).max().unwrap_or(0);
    let width = frames
        .iter()
        .flat_map(|f| f.iter())
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    Animation {
        frames,
        width,
        height,
    }
}
