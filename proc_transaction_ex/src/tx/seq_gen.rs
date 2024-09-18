use std::sync::atomic::AtomicUsize;

/**
 * AtomicUsize basically implements `Copy` trait
 * `SeqGen` also automatically implements `Copy` traits
 */
#[derive(Debug)]
pub struct SeqGen {
    seq: AtomicUsize,
}

impl SeqGen {
    pub fn new() -> Self {
        Self {
            seq: AtomicUsize::new(0),
        }
    }

    /**
     * Atomically increment the sequence number and return the new value
     */
    pub fn next(&self) -> usize {
        self.seq.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }
}

/**
 * Explicitly implement `Send` and `Sync` traits
 * to tell the Rust compiler that `SeqGen` is safe to share between threads
 */
unsafe impl Send for SeqGen {}
unsafe impl Sync for SeqGen {}
