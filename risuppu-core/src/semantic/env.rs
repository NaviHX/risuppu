use crate::semantic::evaluate;
use std::collections::HashMap;
use std::iter::from_fn;
use std::ops::ControlFlow::*;

use crate::sexp::{Ptr, Sexp};
use gc::{Gc, GcCell};
use super::frame::Frame;

#[derive(Clone)]
pub struct Env {
    global_table: HashMap<String, Ptr<Sexp>>,
    provided_table: HashMap<String, Ptr<Sexp>>,
    stack_frame_ptr: Option<Gc<GcCell<Frame>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            global_table: HashMap::new(),
            stack_frame_ptr: None,
            provided_table: HashMap::new(),
        }
    }

    pub fn push_frame(&mut self) {
        let new_ptr = Frame::push(self.stack_frame_ptr.take());
        self.stack_frame_ptr = Some(new_ptr);
    }

    pub fn pop_frame(&mut self) {
        self.stack_frame_ptr = match self.stack_frame_ptr.take() {
            Some(ptr) => Frame::pop(ptr),
            None => None,
        }
    }

    pub fn top_frame(&self) -> Option<Gc<GcCell<Frame>>> {
        self.stack_frame_ptr.clone()
    }

    pub fn set_frame_ptr(
        &mut self,
        new_frame_ptr: Option<Gc<GcCell<Frame>>>,
    ) -> Option<Gc<GcCell<Frame>>> {
        #[cfg(debug_assertions)]
        {
            if let Some(p) = new_frame_ptr.as_ref() {
                let raw = gc::Gc::into_raw(p.clone());
                println!("Switched frame ptr to {:p}:", raw);
                p.borrow().debug();
                unsafe { Gc::from_raw(raw) };
            }
        }
        let old_ptr = self.stack_frame_ptr.take();
        self.stack_frame_ptr = new_frame_ptr;
        old_ptr
    }

    pub fn get(&self, identity: impl AsRef<str>) -> Option<Ptr<Sexp>> {
        let identity = identity.as_ref();
        let mut cur = self.stack_frame_ptr.clone();
        match from_fn(|| match cur.clone() {
            None => None,
            Some(frame_ptr) => {
                cur = frame_ptr.borrow().pre.clone();
                Some(frame_ptr)
            }
        })
        .try_fold(Option::<()>::None, |_, frame| {
            match Frame::read(frame, |frame| frame.get(identity).cloned()) {
                Some(d) => Break(Some(d.clone())),
                None => Continue(None),
            }
        }) {
            Break(p) => p,
            Continue(_) => self.global_table.get(identity).cloned(),
        }
    }

    pub fn set(&mut self, identity: impl ToString, expr: Ptr<Sexp>) {
        if let Some(frame) = self.stack_frame_ptr.clone() {
            Frame::modify(frame, |frame| {
                frame.insert(identity.to_string(), expr.clone());
            });
        } else {
            panic!("No stack frame!");
        }
    }

    pub fn set_global(&mut self, identity: impl ToString, expr: Ptr<Sexp>) {
        self.global_table.insert(identity.to_string(), expr);
    }

    pub fn evaluate(&mut self, expr: Ptr<Sexp>) -> Ptr<Sexp> {
        evaluate(expr, self)
    }

    pub fn add_provided(&mut self, identity: impl ToString, expr: Ptr<Sexp>) {
        self.provided_table.insert(identity.to_string(), expr);
    }

    pub fn get_provided(self) -> HashMap<String, Ptr<Sexp>> {
        self.provided_table
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

