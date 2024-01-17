#[derive(Debug)]
pub enum CommandParameter {
    Decimal(f64),
    Number(i64),
    String(String),
    Pair(String, Box<CommandParameter>),
}
