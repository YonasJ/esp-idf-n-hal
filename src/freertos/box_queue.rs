use crate::{esp_idf::{queueSEND_TO_BACK,
                      std::os::raw::c_void,
                      vQueueDelete,
                      xQueueGenericCreate,
                      xQueueGenericReceive,
                      xQueueGenericSend},
            freertos::{base::*, units::*}};
use alloc::boxed::Box;
use core::{marker::PhantomData, mem};
use core::ptr::NonNull;
use crate::esp_idf::pdPASS;
// use isr::*;

unsafe impl<T: Sized> Send for BoxQueue<T> {}
unsafe impl<T: Sized> Sync for BoxQueue<T> {}

/// A queue with a finite size. The items are owned by the queue and are
/// copied.
#[derive(Debug)]
pub struct BoxQueue<T: Sized> {
    queue:     *mut c_void,
    item_type: PhantomData<T>,
}


impl<T> BoxQueue<T>
{
    /// Splits a statically allocated queue into producer and consumer end points
    pub fn split(&mut self) -> (BoxQueueProducer<T>, BoxQueueConsumer<T>) {
        (
            BoxQueueProducer {
                rb: unsafe { NonNull::new_unchecked(self) },
                _marker: PhantomData,
            },
            BoxQueueConsumer {
                rb: unsafe { NonNull::new_unchecked(self) },
                _marker: PhantomData,
            },
        )
    }
}

/// A queue "consumer"; it can dequeue items from the queue
// NOTE the consumer semantically owns the `head` pointer of the queue
pub struct BoxQueueConsumer<'a, T>
{
    rb: NonNull<BoxQueue<T>>,
    _marker: PhantomData<&'a ()>,
}

unsafe impl<'a, T> Send for BoxQueueConsumer<'a, T>
{}

/// A queue "producer"; it can enqueue items into the queue
// NOTE the producer semantically owns the `tail` pointer of the queue
pub struct BoxQueueProducer<'a, T>
{
    rb: NonNull<BoxQueue<T>>,
    _marker: PhantomData<&'a ()>,
}

unsafe impl<'a, T> Send for BoxQueueProducer<'a, T>
{}



pub trait BoxQueueConsumerT<T: Sized> {
    fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<Box<T>, FreeRtosError>;
}
pub trait BoxQueueProducerT<T: Sized> {
    fn send<D: DurationTicks>(&self, item: Box<T>, max_wait: D) -> Result<(), FreeRtosError>;
}



impl<T: Sized> BoxQueue<T> {
    pub fn new(max_size: usize) -> Result<BoxQueue<T>, FreeRtosError> {
        let item_size = mem::size_of::<*mut T>();
        let handle = unsafe { xQueueGenericCreate(max_size as u32, item_size as u32, crate::esp_idf::queueQUEUE_TYPE_BASE) };

        if handle == 0 as *mut c_void {
            return Err(FreeRtosError::OutOfMemory);
        }

        Ok(BoxQueue {
            queue: handle,
            item_type: PhantomData,
        })
    }

}
impl<'a, T: Sized> BoxQueueProducerT<T> for BoxQueueProducer<'a, T> {
    /// Send an item to the end of the queue. Wait for the queue to have empty space for it.
    fn send<D: DurationTicks>(&self, item: Box<T>, max_wait: D) -> Result<(), FreeRtosError> {
        let mut ptr = Box::into_raw(item) as FreeRtosVoidPtr;
        let ptr_to_ptr = unsafe { ::core::intrinsics::transmute::<*mut _, *mut c_void>(&mut ptr) };

        let ret = unsafe { xQueueGenericSend(self.rb.as_ref().queue, ptr_to_ptr, max_wait.to_ticks(), queueSEND_TO_BACK) };
println!("sent ret == {}", ret);
        if ret != pdPASS as i32 {
            return Err(FreeRtosError::QueueSendTimeout);
        }

        Ok(())
    }
}
impl<'a, T: Sized> BoxQueueConsumerT<T> for BoxQueueConsumer<'a, T> {
    /// Wait for an item to be available on the queue.
    fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<Box<T>, FreeRtosError> {
        let mut ptr = 0 as *mut c_void;
        let ptr_to_ptr = unsafe { ::core::intrinsics::transmute::<*mut _, *mut c_void>(&mut ptr) };

        let ret = unsafe { xQueueGenericReceive(self.rb.as_ref().queue, ptr_to_ptr, max_wait.to_ticks(), 0) };

        if ret != pdPASS as i32 {
            return Err(FreeRtosError::QueueReceiveTimeout);
        }
        if ptr.is_null() {
            return Err(FreeRtosError::InvalidPointer);
        }

        let boxed = unsafe { Box::from_raw(ptr as *mut _) };
        Ok(boxed)
    }
}

impl<T: Sized> Drop for BoxQueue<T> {
    fn drop(&mut self) {
        unsafe {
            vQueueDelete(self.queue);
        }
    }
}
