pub trait CloneToRaw {
	/// SAFETY: `*mut ()` returned should actually be a pointer to the Box where the data is stored
	unsafe fn clone_to_raw(&self) -> *mut dyn CloneToRaw;
}

impl <T: Clone + 'static> CloneToRaw for T {
	unsafe fn clone_to_raw(&self) -> *mut dyn CloneToRaw {
		Box::into_raw(Box::new(self.clone())) as *mut dyn CloneToRaw
	}
}
