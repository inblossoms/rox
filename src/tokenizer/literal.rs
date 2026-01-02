#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    None,
    String(String),
    Number(f64),
    Tuple(Vec<Literal>),
    List(Vec<Literal>),
    Dict(Vec<(Literal, Literal)>),
    Boolean(bool),
}
