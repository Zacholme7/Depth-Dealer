pub mod kind;
pub mod symbol;

pub struct Instument {
    pub base: Symbol,
    pub quote: Symbol,
    pub kind: InstrumentKind,
}


