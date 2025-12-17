pub fn lpad(s: &str, width: usize) -> String {
    format!("{:>width$}", s, width = width)
}

pub fn rpad(s: &str, width: usize) -> String {
    format!("{:<width$}", s, width = width)
}
