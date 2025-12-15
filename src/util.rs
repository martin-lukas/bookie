pub fn rpad(s: &str, width: usize) -> String {
    format!("{:<width$}", s, width = width)
}
