use crossterm::style::Color;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub dim: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::White,
            bg: Color::Black,
            bold: false,
            dim: false,
        }
    }
}

pub struct FrameBuffer {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
    prev_cells: Vec<Cell>,
    pub offset_x: i16,
    pub offset_y: i16,
}

impl FrameBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width as usize) * (height as usize);
        Self {
            width,
            height,
            cells: vec![Cell::default(); size],
            prev_cells: vec![Cell::default(); size],
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let size = (width as usize) * (height as usize);
        self.cells = vec![Cell::default(); size];
        self.prev_cells = vec![Cell::default(); size];
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    pub fn clear_with_color(&mut self, bg: Color) {
        for cell in &mut self.cells {
            *cell = Cell { bg, ..Cell::default() };
        }
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let sx = x + self.offset_x as i32;
        let sy = y + self.offset_y as i32;
        if sx >= 0 && sx < self.width as i32 && sy >= 0 && sy < self.height as i32 {
            let idx = (sy as usize) * (self.width as usize) + (sx as usize);
            self.cells[idx] = cell;
        }
    }

    pub fn set_char(&mut self, x: i32, y: i32, ch: char, fg: Color, bg: Color) {
        self.set(x, y, Cell { ch, fg, bg, bold: false, dim: false });
    }

    pub fn set_str(&mut self, x: i32, y: i32, s: &str, fg: Color, bg: Color) {
        for (i, ch) in s.chars().enumerate() {
            self.set_char(x + i as i32, y, ch, fg, bg);
        }
    }

    pub fn set_str_bold(&mut self, x: i32, y: i32, s: &str, fg: Color, bg: Color) {
        for (i, ch) in s.chars().enumerate() {
            self.set(x + i as i32, y, Cell { ch, fg, bg, bold: true, dim: false });
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, ch: char, fg: Color, bg: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            self.set_char(x, y, ch, fg, bg);
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_box(&mut self, x: i32, y: i32, w: i32, h: i32, fg: Color, bg: Color) {
        self.set_char(x, y, '╭', fg, bg);
        self.set_char(x + w - 1, y, '╮', fg, bg);
        self.set_char(x, y + h - 1, '╰', fg, bg);
        self.set_char(x + w - 1, y + h - 1, '╯', fg, bg);

        for i in 1..(w - 1) {
            self.set_char(x + i, y, '─', fg, bg);
            self.set_char(x + i, y + h - 1, '─', fg, bg);
        }
        for i in 1..(h - 1) {
            self.set_char(x, y + i, '│', fg, bg);
            self.set_char(x + w - 1, y + i, '│', fg, bg);
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, ch: char, fg: Color, bg: Color) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_char(x + dx, y + dy, ch, fg, bg);
            }
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, ch: char, fg: Color, bg: Color) {
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        while x <= y {
            self.set_char(cx + x, cy + y, ch, fg, bg);
            self.set_char(cx - x, cy + y, ch, fg, bg);
            self.set_char(cx + x, cy - y, ch, fg, bg);
            self.set_char(cx - x, cy - y, ch, fg, bg);
            self.set_char(cx + y, cy + x, ch, fg, bg);
            self.set_char(cx - y, cy + x, ch, fg, bg);
            self.set_char(cx + y, cy - x, ch, fg, bg);
            self.set_char(cx - y, cy - x, ch, fg, bg);

            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    /// Diff-render: only redraw cells that changed since last frame
    pub fn render_diff<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        use crossterm::style::{SetForegroundColor, SetBackgroundColor, SetAttribute, Attribute, ResetColor};
        use crossterm::cursor::MoveTo;
        use crossterm::QueueableCommand;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let cell = self.cells[idx];
                let prev = self.prev_cells[idx];

                if cell != prev {
                    out.queue(MoveTo(x, y))?;
                    out.queue(SetForegroundColor(cell.fg))?;
                    out.queue(SetBackgroundColor(cell.bg))?;
                    if cell.bold {
                        out.queue(SetAttribute(Attribute::Bold))?;
                    }
                    if cell.dim {
                        out.queue(SetAttribute(Attribute::Dim))?;
                    }
                    write!(out, "{}", cell.ch)?;
                    if cell.bold || cell.dim {
                        out.queue(SetAttribute(Attribute::Reset))?;
                    }
                    out.queue(ResetColor)?;
                }
            }
        }

        self.prev_cells.clone_from(&self.cells);
        out.flush()?;
        Ok(())
    }

    /// Full render: draw entire buffer
    pub fn render_full<W: Write>(&mut self, out: &mut W) -> std::io::Result<()> {
        use crossterm::style::{SetForegroundColor, SetBackgroundColor, SetAttribute, Attribute, ResetColor};
        use crossterm::cursor::MoveTo;
        use crossterm::QueueableCommand;

        for y in 0..self.height {
            out.queue(MoveTo(0, y))?;
            for x in 0..self.width {
                let idx = (y as usize) * (self.width as usize) + (x as usize);
                let cell = self.cells[idx];
                out.queue(SetForegroundColor(cell.fg))?;
                out.queue(SetBackgroundColor(cell.bg))?;
                if cell.bold {
                    out.queue(SetAttribute(Attribute::Bold))?;
                }
                write!(out, "{}", cell.ch)?;
                if cell.bold {
                    out.queue(SetAttribute(Attribute::Reset))?;
                }
            }
            out.queue(ResetColor)?;
        }

        self.prev_cells.clone_from(&self.cells);
        out.flush()?;
        Ok(())
    }
}
