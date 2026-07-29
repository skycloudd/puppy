use crate::{interpreter::value::Value, ir::Spanned};

#[derive(Clone, Debug, Default)]
pub struct Scopes(Vec<(Spanned<&'static str>, Value)>);

impl Scopes {
    pub fn push(&mut self, k: Spanned<&'static str>, v: Value) {
        self.0.push((k, v));
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn get(&self, k: &'static str) -> Option<&Value> {
        self.0
            .iter()
            .rev()
            .find_map(|v| (v.0.inner == k).then_some(&v.1))
    }

    pub const fn len(&self) -> ScopeLen {
        ScopeLen(self.0.len())
    }

    pub fn truncate(&mut self, len: ScopeLen) {
        self.0.truncate(len.0);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScopeLen(usize);
