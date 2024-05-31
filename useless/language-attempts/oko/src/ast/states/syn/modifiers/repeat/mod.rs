modules!(policies);

use crate::*;
use core::marker::PhantomData;
use core::any::TypeId;
use std::fmt::{Debug, Formatter};

/// Indicator to place zero separators inbetween items
pub type NoSep = Seq <"">;

#[derive(Clone, Visitable)]
pub struct ParseRepeatedly <
    T: Debug + Clone + Visitable,
    Sep,
    DoCollectPolicy: Boolean,
    const AT_LEAST: usize
> where Self: ParseRepeatedlyDoCollectSpecific <T>
{
    pub vec: <Self as ParseRepeatedlyDoCollectSpecific <T>>::VecType,
    #[novisit]
    pub trailing_sep: bool,
    #[novisit]
    pub last_error: Box <ParseError>,
    #[novisit]
    pub _marker: PhantomData <Sep>
}

impl <T: Parseable + Debug + Clone + Visitable, Sep: Parseable + 'static, DoCollect: Boolean, const AT_LEAST: usize> Parseable for ParseRepeatedly <T, Sep, DoCollect, AT_LEAST> where
    Self: ParseRepeatedlyDoCollectSpecific <T>
{
    fn __parse(parser: &mut Parser) -> Result <Self, ParseError> {
        let mut already_done = 0;
        let mut vec = vec![];
        loop {
            match parser.parse::<T>() {
                Ok(x) => Self::push_if_do_collect(&mut vec, x),
                Err(x) => return if already_done >= AT_LEAST {
                    Ok(Self {
                        trailing_sep: !vec.is_empty(),
                        vec: Self::create_vec_if_do_collect(vec),
                        last_error: Box::new(x),
                        _marker: PhantomData
                    })
                } else {
                    Err(x)
                }
            };
            // A small optimization
            if TypeId::of::<Sep>() != TypeId::of::<NoSep>() {
                match parser.parse::<Sep>() {
                    Ok(_) => (),
                    Err(x) => return if already_done >= AT_LEAST {
                        Ok(Self {
                            vec: Self::create_vec_if_do_collect(vec),
                            trailing_sep: false,
                            last_error: Box::new(x),
                            _marker: PhantomData
                        })
                    } else {
                        Err(x)
                    }
                }
            }
            already_done += 1
        }
    }

    fn add_to_span(&mut self, offset: usize) {
        Self::add_to_span_if_do_collect(&mut self.vec, offset)
    }
}

impl <T: Debug + Clone + Visitable, Sep, DoCollect: Boolean + 'static, const AT_LEAST: usize> Debug for ParseRepeatedly <T, Sep, DoCollect, AT_LEAST> where
    Self: ParseRepeatedlyDoCollectSpecific <T>
{
    fn fmt(&self, f: &mut Formatter <'_>) -> std::fmt::Result {
        let mut str = f.debug_struct("ParseRepeatedly");

        if TypeId::of::<DoCollect>() == TypeId::of::<True>() {
            str.field("vec", &self.vec);
        }

        str.field("trailing_sep", &self.trailing_sep).finish()
    }
}
