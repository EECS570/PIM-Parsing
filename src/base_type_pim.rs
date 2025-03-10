pub enum PIMBaseType {
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    Char,
}

pub enum PIMType {
    Basic(PIMBaseType),
    Array(PIMBaseType, i32),
}
