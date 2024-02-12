use std::{fmt::Debug, cell::RefCell};

use gc::{Finalize, Trace};

use super::{Ptr, Sexp};

type InnerRustFn = Box<RefCell<dyn FnMut(Ptr<Sexp>) -> Ptr<Sexp> + 'static>>;

#[derive(Finalize)]
pub struct RustFn {
    inner: InnerRustFn,
}

impl RustFn {
    /// # Safety
    /// Don't capture `Gc` value in the closure, which will escape from the gc management.
    pub unsafe fn new(f: impl FnMut(Ptr<Sexp>) -> Ptr<Sexp> + 'static) -> Self {
        Self {
            inner: Box::new(RefCell::new(f))
        }
    }

    pub fn call(&self, arg: Ptr<Sexp>) -> Ptr<Sexp> {
        self.inner.borrow_mut()(arg)
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
