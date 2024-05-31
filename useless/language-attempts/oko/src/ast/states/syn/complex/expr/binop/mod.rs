modules!(bin_ops);

use crate::*;

#[derive(Debug, Clone, Visitable)]
pub struct ExprBinOp {
	pub op: Leaf <BinOp>,
	pub lhs: Expr,
	pub rhs: Expr
}

impl Parseable for ExprBinOp {
    /// Should never be called as instances of this type are yielded by the function below
	fn __parse(_: &mut Parser) -> Result <Self, ParseError> {
		unimplemented!()
	}

	fn add_to_span(&mut self, offset: usize) {
		self.lhs.add_to_span(offset);
		self.rhs.add_to_span(offset);
	}
}

pub(super) fn launch_precedence_climbing(parser: &mut Parser, min_prec: Precedence) -> Result <Expr, ParseError> {
    let check = |parser: &mut Parser| {
        let offset = parser.offset;
        let x = parser.parse::<Surrounded <BinOp, Whitespace>>().ok().and_then(|x| {
            let pa = x.value.get_prec_assoc();
            if pa.0 >= min_prec { Some((pa.0, pa.1, x)) } else { None }
        });
        if x.is_none() {
            parser.offset = offset
        }
        x
    };

    let mut lhs = Expr::Atom(parser.parse::<ExprAtom>()?);
    while let Some((prec, assoc, op)) = check(parser) {
        let rhs = launch_precedence_climbing(parser, prec + if assoc == BinOpAssociativity::Left { 1 } else { 0 })?;
        lhs = Expr::BinOp(Box::new(ExprBinOp { op: Leaf::new(op.value), lhs, rhs }));
    }

    Ok(lhs)
}
