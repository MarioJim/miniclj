#[derive(Debug)]
pub enum Address {
    Literal(Type),
    Variable(Type),
}

#[derive(Debug)]
pub enum Type {
    Number,
    String,
    List,
    Vector,
    Set,
    Map,
}
