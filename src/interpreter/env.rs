use crate::{interpreter::value::Value, ir::Ident};

#[derive(Clone, Debug, Default)]
pub struct Env(Vec<(Ident, Value)>);

impl Env {
    pub fn push(&mut self, k: Ident, v: Value) {
        self.0.push((k, v));
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn get(&self, k: Ident) -> Option<Value> {
        self.0
            .iter()
            .rev()
            .find_map(|v| (v.0 == k).then_some(&v.1))
            .cloned()
    }

    pub const fn len(&self) -> EnvLen {
        EnvLen(self.0.len())
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0);
    }

    pub fn truncate(&mut self, len: EnvLen) {
        self.0.truncate(len.0);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnvLen(usize);
