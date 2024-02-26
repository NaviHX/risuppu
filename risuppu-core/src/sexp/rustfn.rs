use std::{cell::RefCell, fmt::Debug};

use gc::{Finalize, Trace};

use crate::semantic::Env;

use super::{Ptr, Sexp};

type InnerRustFnMut = Box<RefCell<dyn FnMut(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static>>;
type InnerRustFn = Box<RefCell<dyn Fn(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static>>;

#[derive(Finalize)]
pub struct RustFn {
    inner: InnerRustFnMut,
    preprocess: Option<InnerRustFn>,
}

impl RustFn {
    /// # Safety
    /// Don't capture `Gc` value in the closure, which will escape from the gc management.
    /// Don't recurse in the function body.
    /// Quote the ret-value if it might be a list and this function is not a macro.
    pub unsafe fn new(f: impl FnMut(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static) -> Self {
        Self {
            inner: Box::new(RefCell::new(f)),
            preprocess: None,
        }
    }

    /// # Safety
    /// Don't capture `Gc` value in the closure, which will escape from the gc management.
    /// Don't recurse in f's body.
    /// Quote the ret-value if it might be a list and this function is not a macro.
    pub unsafe fn new_with_preprocess(
        f: impl FnMut(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static,
        p: impl Fn(Ptr<Sexp>, &mut Env) -> Ptr<Sexp> + 'static,
    ) -> Self {
        Self {
            inner: Box::new(RefCell::new(f)),
            preprocess: Some(Box::new(RefCell::new(p))),
        }
    }

    pub fn call(&self, arg: Ptr<Sexp>, env: &mut Env) -> Ptr<Sexp> {
        // BUG: Recursion will cause `BorrowMut: alread borrowed` error.
        let arg = if let Some(preprocess) = self.preprocess.as_ref() {
            (**preprocess).borrow()(arg, env)
        } else {
            arg
        };

        self.inner.borrow_mut()(arg, env)
    }
}

impl Debug for RustFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustFn").finish()
    }
}

impl PartialEq for RustFn {
    fn eq(&self, other: &Self) -> bool {
        let s = &*self.inner as *const _;
        let o = &*other.inner as *const _;
        std::ptr::addr_eq(s, o)
    }
}

impl Eq for RustFn {}

unsafe impl Trace for RustFn {
    unsafe fn trace(&self) {}

    unsafe fn root(&self) {}

    unsafe fn unroot(&self) {}

    fn finalize_glue(&self) {
        self.finalize()
    }
}
