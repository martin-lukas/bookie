use crate::{
    domain::layout::Rect,
    util::{lpad, rpad},
};
use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{Color, Print, PrintStyledContent, SetBackgroundColor, SetForegroundColor, Stylize},
};
use std::{
    cmp::max,
    io::{self, stdout},
};
use unicode_width::UnicodeWidthStr;

pub struct Table {
    header: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    col_widths: Vec<usize>,
}

impl Table {
    pub fn new(header: Vec<TableCell>, rows: Vec<Vec<TableCell>>) -> Self {
        let col_widths: Vec<usize> = (0..header.len())
            .map(|col| {
                let header_width = header[col].value.width();
                let cell_max_width = rows
                    .iter()
                    .map(|row| row[col].value.width())
                    .max()
                    .unwrap_or(0);
                max(header_width, cell_max_width)
            })
            .collect();
        Self {
            header,
            rows,
            col_widths,
        }
    }
    pub fn col_widths(mut self, col_widths: Vec<usize>) -> Self {
        self.col_widths = col_widths;
        self
    }

    pub fn render(&self, rect: &Rect, active_row: usize) -> io::Result<()> {
        let mut out = stdout();

        let header_cells: Vec<TableCell> = (0..self.header.len())
            .map(|col| {
                let mut cell = self.header[col].clone();
                let col_width = self.col_widths[col];
                let mut padded_value = match cell.align {
                    Align::Left => rpad(&cell.value, col_width),
                    Align::Right => lpad(&cell.value, col_width),
                };
                if col != self.header.len() - 1 {
                    padded_value.push_str(&" │ ");
                }
                cell.value = padded_value;
                cell
            })
            .collect();
        let table_rows: Vec<Vec<TableCell>> = (0..self.rows.len())
            .map(|row| {
                let row = &self.rows[row];
                (0..row.len())
                    .map(|col| {
                        let mut cell = row[col].clone();
                        let col_width = self.col_widths[col];
                        let mut padded_value = match cell.align {
                            Align::Left => rpad(&cell.value, col_width),
                            Align::Right => lpad(&cell.value, col_width),
                        };
                        if col != row.len() - 1 {
                            padded_value.push_str(&" │ ");
                        }
                        cell.value = padded_value;
                        cell
                    })
                    .collect()
            })
            .collect();

        // HEADER
        for cell in header_cells.iter() {
            if let Some(color) = cell.color {
                execute!(out, SetForegroundColor(color))?;
            }
            execute!(
                out,
                PrintStyledContent(cell.value.to_string().bold()),
                SetForegroundColor(Color::Reset)
            )?;
        }

        // SEPARATOR
        execute!(out, MoveToNextLine(1))?;
        for (i, col_width) in self.col_widths.iter().enumerate() {
            execute!(out, Print("─".repeat(*col_width)))?;
            if i != self.col_widths.len() - 1 {
                execute!(out, Print("─┼─"))?;
            }
        }
        execute!(out, MoveToNextLine(1))?;

        // BODY
        for (i, row) in table_rows.iter().take(rect.height as usize - 2).enumerate() {
            if i == active_row {
                execute!(out, SetBackgroundColor(Color::Black))?;
            }
            for cell in row.iter() {
                if let Some(color) = cell.color {
                    execute!(out, SetForegroundColor(color))?;
                }
                execute!(
                    out,
                    Print(cell.value.to_string()),
                    SetForegroundColor(Color::Reset)
                )?;
            }
            if i == active_row {
                execute!(out, SetBackgroundColor(Color::Reset))?;
            }
            execute!(out, MoveToNextLine(1))?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct TableCell {
    value: String,
    color: Option<Color>,
    align: Align,
}

impl TableCell {
    pub fn new(value: String) -> Self {
        Self {
            value,
            color: None,
            align: Align::Left,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
}

#[derive(Clone)]
pub enum Align {
    Left,
    Right,
}
