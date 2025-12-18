use crate::util::{lpad, rpad};
use crossterm::{
    cursor::MoveToNextLine,
    execute,
    style::{Print, PrintStyledContent, Stylize},
};
use std::{
    cmp::max,
    io::{self, stdout},
};

pub struct Table {
    header: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    col_widths: Vec<usize>,
    sep_width: usize,
}

impl Table {
    pub(crate) fn new(header: Vec<TableCell>, rows: Vec<Vec<TableCell>>) -> Self {
        let col_widths: Vec<usize> = (0..header.len())
            .map(|col| {
                let header_width = header[col].value.len();
                let cell_max_width = rows
                    .iter()
                    .map(|row| row[col].value.len())
                    .max()
                    .unwrap_or(0);
                max(header_width, cell_max_width)
            })
            .collect();
        Self {
            header,
            rows,
            col_widths,
            sep_width: 1,
        }
    }
    pub(crate) fn col_widths(mut self, col_widths: Vec<usize>) -> Self {
        self.col_widths = col_widths;
        self
    }
    pub(crate) fn sep_width(mut self, sep_width: usize) -> Self {
        self.sep_width = sep_width;
        self
    }
}

pub struct TableCell {
    value: String,
    align: Align,
}

impl TableCell {
    pub(crate) fn new(value: String) -> Self {
        Self {
            value,
            align: Align::Left,
        }
    }
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
}

pub enum Align {
    Left,
    Right,
}

pub fn render_table(table: &Table, active_row: usize) -> io::Result<()> {
    let mut out = stdout();

    let header_row = (0..table.header.len())
        .map(|col| {
            let cell = &table.header[col];
            let col_width = table.col_widths[col];
            let mut cell_value = match cell.align {
                Align::Left => rpad(&cell.value, col_width),
                Align::Right => lpad(&cell.value, col_width),
            };
            if col != table.header.len() - 1 {
                cell_value.push_str(&" ".repeat(table.sep_width));
                cell_value
            } else {
                cell_value
            }
        })
        .collect::<String>();
    let table_rows: Vec<String> = (0..table.rows.len())
        .map(|row| {
            let row = &table.rows[row];
            (0..row.len())
                .map(|col| {
                    let cell = &row[col];
                    let col_width = table.col_widths[col];
                    let mut cell_value = match cell.align {
                        Align::Left => rpad(&cell.value, col_width),
                        Align::Right => lpad(&cell.value, col_width),
                    };
                    if col != row.len() - 1 {
                        cell_value.push_str(&" ".repeat(table.sep_width));
                        cell_value
                    } else {
                        cell_value
                    }
                })
                .collect::<String>()
        })
        .collect();

    execute!(
        out,
        PrintStyledContent(header_row.to_string().bold()),
        MoveToNextLine(1),
        Print("-".repeat(header_row.len())),
        MoveToNextLine(1),
    )?;
    for (i, table_row) in table_rows.iter().enumerate() {
        if i == active_row {
            execute!(
                out,
                PrintStyledContent(table_row.to_string().bold().yellow())
            )?;
        } else {
            execute!(out, Print(table_row.to_string()))?;
        }
        execute!(out, MoveToNextLine(1))?;
    }

    Ok(())
}
