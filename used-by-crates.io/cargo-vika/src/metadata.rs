use cargo_metadata::Metadata;

static mut METADATA: Option <Metadata> = None;

pub fn init(metadata: Metadata) {
    unsafe { METADATA = Some(metadata) }
}

pub fn metadata() -> &'static Metadata {
    unsafe {
        match METADATA {
            Some(ref meta) => meta,
            None => crate::pretty_output::error("this command requires being in package")
        }
    }
}
