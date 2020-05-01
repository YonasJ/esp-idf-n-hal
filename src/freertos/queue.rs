use crate::{esp_idf::{queueSEND_TO_BACK,
                      std::os::raw::c_void,
                      vQueueDelete,
                      xQueueGenericCreate,
                      xQueueGenericReceive,
                      xQueueGenericSend,
                      xQueueGenericSendFromISR},
            freertos::{base::*, isr::*, prelude::v1::*, units::*}};

unsafe impl<T: Sized + Copy> Send for Queue<T> {}
unsafe impl<T: Sized + Copy> Sync for Queue<T> {}

/// A queue with a finite size. The items are owned by the queue and are
/// copied.
#[derive(Debug)]
pub struct Queue<T: Sized + Copy> {
    queue:     *mut c_void,
    item_type: PhantomData<T>,
}

impl<T: Sized + Copy> Queue<T> {
    pub fn new(max_size: usize) -> Result<Queue<T>, FreeRtosError> {
        let item_size = mem::size_of::<T>();

        let handle = unsafe { xQueueGenericCreate(max_size as u32, item_size as u32, crate::esp_idf::queueQUEUE_TYPE_BASE) };

        if handle == 0 as *mut c_void {
            return Err(FreeRtosError::OutOfMemory);
        }

        Ok(Queue { queue:     handle,
                   item_type: PhantomData, })
    }

    /// Send an item to the end of the queue. Wait for the queue to have empty space for it.
    pub fn send<D: DurationTicks>(&self, item: T, max_wait: D) -> Result<(), FreeRtosError> {
        unsafe {
            if xQueueGenericSend(self.queue, &item as *const _ as *const c_void, max_wait.to_ticks(), queueSEND_TO_BACK) != 0 {
                Err(FreeRtosError::QueueSendTimeout)
            } else {
                Ok(())
            }
        }
    }

    /// Send an item to the end of the queue, from an interrupt.
    pub fn send_from_isr(&self, context: &mut InterruptContext, item: T) -> Result<(), FreeRtosError> {
        unsafe {
            if xQueueGenericSendFromISR(self.queue, &item as *const _ as *const c_void, context.get_task_field_mut(), queueSEND_TO_BACK) != 0 {
                Err(FreeRtosError::QueueFull)
            } else {
                Ok(())
            }
        }
    }

    /// Wait for an item to be available on the queue.
    pub fn receive<D: DurationTicks>(&self, max_wait: D) -> Result<T, FreeRtosError> {
        unsafe {
            let mut buff = mem::zeroed::<T>();

            let r = xQueueGenericReceive(self.queue, &mut buff as *mut _ as *mut c_void, max_wait.to_ticks(), 0);

            if r == 0 {
                Ok(buff)
            } else {
                Err(FreeRtosError::QueueReceiveTimeout)
            }
        }
    }
}

impl<T: Sized + Copy> Drop for Queue<T> {
    fn drop(&mut self) {
        unsafe {
            vQueueDelete(self.queue);
        }
    }
}
