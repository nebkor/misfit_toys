use std::marker::PhantomPinned;
use std::pin::Pin;

const MAX_PACKET_SIZE: usize = 64;

trait USBWrite {
    fn poll(self: Pin<Self>);
}

struct USB<'a, D>
where
    D: AsRef<[u8]>,
{
    queued: Option<D>,
    sending: Option<D>,
    iter: std::slice::Chunks<'a, u8>,
    _pin: PhantomPinned,
}

impl<'a, D> USB<'a, D>
where
    D: AsRef<[u8]>,
{
    fn new() -> Self {
        USB {
            queued: None,
            sending: None,
            iter: [].chunks(1),
        }
    }

    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmov {
            data,
            // we only create the pointer once the data is in place
            // otherwise it will have already moved before we even started
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let slice = NonNull::from(&boxed.data);
        // we know this is safe because modifying a field doesn't move the whole struct
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).slice = slice;
        }
        boxed
    }

    fn queue(&mut self, new_data: D) {
        self.queued = Some(new_data);
    }

    fn poll(&'a mut self) {
        match (self.iter.next(), self.queued.is_some()) {
            (Some(_bytes), _) => {
                //write one packet's worth of bytes to USB
            }

            (None, true) => {
                //we finished sending last message and have a new one to start sending
                self.sending = Some(self.queued.take().unwrap());
                self.iter = self
                    .sending
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .chunks(MAX_PACKET_SIZE as usize);

                self.poll(); //recur so first chunk of new data is sent
            }

            _ => {
                //do nothing
            }
        }
    }
}

fn main() {
    let mut usb = USB::new();
    usb.queue([1; 20]);
    loop {
        usb.poll();
    }
}
