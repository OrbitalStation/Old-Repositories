use crate::*;

#[derive(Debug, Clone, Parseable)]
pub struct SynType {
    pub ident: Ident
}

impl TypeIndex for SynType {
    fn resolute(&self, mut type_list: TypeIndexIter) -> Option <Type> {
        // `Iterator::find` requires `Self` to be sized, but dyn types can never be sized
        //      so the call cannot be performed directly.
        // However, there exists an impl of Iterator for `&mut I` where I: Iterator, and we can
        //      call `find` on that impl and thus work around `dyn` restriction
        let index = <TypeIndexIter as Iterator>::find(&mut type_list, |item| item.1.val.name.value.val == self.ident.value.val)?.0;

        Some(Type {
            inner: Box::new(ScopeType {
                index,
                span: self.ident.span.clone()
            })
        })
    }

    fn span(&self) -> Span {
        self.ident.span.val.clone()
    }
}
