#[derive(Debug)]
pub enum CommandParameter {
    Decimal(f64),
    Number(i128),
    String(String),
    Pair((String, String)),
}
