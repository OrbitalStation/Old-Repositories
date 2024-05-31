use std::fmt::Debug;
use crate::*;

pub trait ParseRepeatedlyDoCollectSpecific <T> {
    type VecType: Debug + Clone + Visitable;

    fn push_if_do_collect(vec: &mut Vec <T>, val: T);

    fn create_vec_if_do_collect(vec: Vec <T>) -> Self::VecType;

    fn add_to_span_if_do_collect(vec: &mut Self::VecType, offset: usize);
}

impl <T: Debug + Clone + Visitable + Parseable, Sep, const AT_LEAST: usize> ParseRepeatedlyDoCollectSpecific <T> for ParseRepeatedly <T, Sep, True, AT_LEAST> {
    type VecType = Vec <T>;

    fn push_if_do_collect(vec: &mut Vec <T>, val: T) {
        vec.push(val)
    }

    fn create_vec_if_do_collect(vec: Vec <T>) -> Self::VecType {
        vec
    }

    fn add_to_span_if_do_collect(vec: &mut Self::VecType, offset: usize) {
        for x in vec {
            x.add_to_span(offset)
        }
    }
}

impl <T: Debug + Clone + Visitable + Parseable, Sep, const AT_LEAST: usize> ParseRepeatedlyDoCollectSpecific <T> for ParseRepeatedly <T, Sep, False, AT_LEAST> {
    type VecType = Leaf <()>;

    fn push_if_do_collect(_: &mut Vec <T>, _: T) {}

    fn create_vec_if_do_collect(_: Vec <T>) -> Self::VecType {
        Leaf::new(())
    }

    fn add_to_span_if_do_collect(_: &mut Self::VecType, _: usize) {}
}

