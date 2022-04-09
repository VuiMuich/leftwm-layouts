use crate::{
    geometry::{Rect, SplitAxis},
    Layout, LayoutModifiers,
};

use crate::layouts::columns;

#[derive(Debug)]
pub struct Fibonacci;

impl Layout for Fibonacci {
    fn apply(
        &self,
        window_count: usize,
        container: Rect,
        modifiers: &LayoutModifiers,
    ) -> Vec<Rect> {
        columns::main_stack(
            window_count,
            container,
            &LayoutModifiers {
                first_stack_split: SplitAxis::Fibonacci,
                ..*modifiers
            },
        )
    }
}

mod tests {}
