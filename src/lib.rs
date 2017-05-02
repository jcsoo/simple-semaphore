#![no_std]

use core::cell::Cell;

macro_rules! static_semaphore {
    () => {        
        {
            static mut SEM: Option<Semaphore> = None;;
            unsafe {
                SEM = Some(Semaphore { reader: Cell::new(0), writer: Cell::new(0) });
                (
                    SemaphoreReader { semaphore: &SEM.as_ref().unwrap()},
                    SemaphoreWriter { semaphore: &SEM.as_ref().unwrap()}
                )
            }
        }
    }
}

/// A simple single-reader, single-writer counting semaphore. Access via the SemaphoreReader and SemaphoreWriter
pub struct Semaphore {
    reader: Cell<usize>,
    writer: Cell<usize>,
}

impl Semaphore {
    /// Create a new Semaphore
    pub fn new() -> Semaphore {
        Semaphore { reader: Cell::new(0), writer: Cell::new(0) }
    }

    // Create a SemaphoreReader / SemaphoreWriter pair refererring to this semaphore.
    pub fn pair<'a>(&'a self) -> (SemaphoreReader<'a>, SemaphoreWriter<'a>) {
        let r = SemaphoreReader { semaphore: self };
        let w = SemaphoreWriter { semaphore: self };
        (r, w)
    }
}

pub struct SemaphoreReader<'a> {
    semaphore: &'a Semaphore
}

impl<'a> SemaphoreReader<'a> {
    /// Read and clear the value of the Semaphore
    pub fn read(&self) -> usize {
        let sem = self.semaphore;
        let writer = sem.writer.get();
        let reader = sem.reader.get();
        sem.reader.set(writer);
        writer.wrapping_sub(reader)
    }
}

pub struct SemaphoreWriter<'a> {
    semaphore: &'a Semaphore
}

impl<'a> SemaphoreWriter<'a> {
    /// Increment the value of the Semaphore
    pub fn write(&self, value: usize) {
        let sem = self.semaphore;
        let writer = sem.writer.get().wrapping_add(value);
        sem.writer.set(writer)
    }    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semaphore() {
        let s = Semaphore::new();
        let (r, w) = s.pair();
        w.write(1);
        assert_eq!(r.read(), 1);
        assert_eq!(r.read(), 0);
        w.write(1);
        w.write(2);
        assert_eq!(r.read(), 3);
    }
    
    #[test]
    fn test_static_semaphore() {
        let (r, w) = static_semaphore!();
        w.write(1);
        assert_eq!(r.read(), 1);
        assert_eq!(r.read(), 0);
        w.write(1);
        w.write(2);
        assert_eq!(r.read(), 3);

        let sr: SemaphoreReader<'static> = r;
        assert_eq!(sr.read(), 0);
    }
}