use std::sync::atomic::{AtomicU32, Ordering};

static SEQ: AtomicU32 = AtomicU32::new(1);
const MAX_SEQ: u32 = u16::MAX as u32;

pub fn next_seq() -> u32 {
    let seq = SEQ.fetch_update(Ordering::SeqCst, Ordering::Relaxed, |seq| {
        let seq = match seq {
            MAX_SEQ => 1,
            _ => seq + 1,
        };
        Some(seq)
    });
    // Safety: safe because fn in fetch_update always returns Some(seq).
    unsafe { seq.unwrap_unchecked() }
}
