#[derive(Debug)]
pub struct Source {
    pub path: String,
    pub content: String,
    /// The byte positions where each new line starts
    pub line_starts: Vec<u32>,
}

impl Source {
    pub fn new(path: String) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(&path)?;

        let mut line_starts = vec![0]; // Line 1 starts at byte 0

        // Efficiently find all newline characters
        for (idx, byte) in content.as_bytes().iter().enumerate() {
            if *byte == b'\n' {
                line_starts.push((idx + 1) as u32);
            }
        }

        Ok(Self {
            path,
            content,
            line_starts,
        })
    }

    /// Dynamically grab a line's text without duplicating strings in memory!
    /// `line_idx` is 0-indexed here.
    pub fn get_line(&self, line_idx: usize) -> Option<&str> {
        if line_idx >= self.line_starts.len() {
            return None;
        }

        let start = self.line_starts[line_idx] as usize;

        // If it's the last line, it goes to the end of the file
        let end = if line_idx + 1 < self.line_starts.len() {
            (self.line_starts[line_idx + 1] - 1) as usize
        } else {
            self.content.len()
        };

        Some(&self.content[start..end])
    }
}
