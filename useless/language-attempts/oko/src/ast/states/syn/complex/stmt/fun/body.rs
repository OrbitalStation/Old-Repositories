use crate::*;

#[derive(Debug, Clone, Parseable, Visitable)]
pub enum FunBody {
    Short(ShortFunBody),
    Long(LongFunBody)
}

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct ShortFunBody {
    pub explicit_whitespace: Whitespace,
    pub eq: Seq <"=">,
    pub expr: Box <Expr>,
    pub newline: Newline
}

#[derive(Debug, Clone, Parseable, Visitable)]
pub struct LongFunBody {
    pub block: Block <FunAct>
}
