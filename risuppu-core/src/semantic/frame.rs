use gc::{Gc, GcCell, Trace, Finalize};

use crate::sexp::{Ptr, Sexp};

type MutPtr<T> = Gc<GcCell<T>>;
type InnerFrame = std::collections::HashMap<String, Ptr<Sexp>>;

#[derive(Debug, Clone, Trace, Finalize, PartialEq, Eq)]
pub struct Frame {
    pub inner: InnerFrame,
    pub pre: Option<MutPtr<Frame>>,
}

impl Frame {
    pub fn new() -> MutPtr<Self> {
        Gc::new(GcCell::new(Self {
            inner: InnerFrame::new(),
            pre: None,
        }))
    }

    pub fn push(cur: Option<MutPtr<Self>>) -> MutPtr<Self> {
        let new_cur = Self::new();
        new_cur.borrow_mut().pre = cur;
        new_cur
    }

    pub fn pop(cur: MutPtr<Self>) -> Option<MutPtr<Self>> {
        cur.borrow_mut().pre.clone()
    }

    pub fn modify(frame_ptr: MutPtr<Self>, mut f: impl FnMut(&mut InnerFrame)) -> MutPtr<Self> {
        f(&mut frame_ptr.borrow_mut().inner);
        frame_ptr
    }

    pub fn read<O>(frame_ptr: MutPtr<Self>, mut f: impl FnMut(&InnerFrame) -> O) -> O {
        f(& frame_ptr.borrow().inner)
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        println!("--- FRAME ---");
        for (k, v) in self.inner.iter() {
            println!("{k} -> {v}");
        }
        if let Some(pre) = self.pre.as_ref() {
            pre.borrow().debug()
        }
    }
}
