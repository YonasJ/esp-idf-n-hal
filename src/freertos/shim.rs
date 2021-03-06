#![allow(non_snake_case)]

use crate::freertos::base::*;

#[cfg(target_os = "none")]
extern "C" {
    pub fn freertos_rs_sizeof(_type: u8) -> u8;

    pub fn freertos_rs_vTaskDelayUntil(pxPreviousWakeTime: *mut FreeRtosTickType, xTimeIncrement: FreeRtosTickType);
    pub fn freertos_rs_vTaskDelay(xTicksToDelay: FreeRtosTickType);
    pub fn freertos_rs_get_portTICK_PERIOD_MS() -> FreeRtosTickType;

    pub fn freertos_rs_get_number_of_tasks() -> FreeRtosUBaseType;

    pub fn freertos_rs_xTaskGetTickCount() -> FreeRtosTickType;

    pub fn freertos_rs_create_recursive_mutex() -> FreeRtosQueueHandle;
    pub fn freertos_rs_create_mutex() -> FreeRtosQueueHandle;

    pub fn freertos_rs_take_recursive_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType;
    pub fn freertos_rs_take_mutex(mutex: FreeRtosQueueHandle, max: FreeRtosTickType) -> FreeRtosBaseType;
    pub fn freertos_rs_give_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType;
    pub fn freertos_rs_give_recursive_mutex(mutex: FreeRtosQueueHandle) -> FreeRtosBaseType;

    pub fn freertos_rs_delete_semaphore(mutex: FreeRtosQueueHandle);

    pub fn freertos_rs_create_binary_semaphore() -> FreeRtosQueueHandle;
    pub fn freertos_rs_create_counting_semaphore(max: FreeRtosUBaseType, initial: FreeRtosUBaseType) -> FreeRtosQueueHandle;

    pub fn freertos_rs_isr_yield();

    pub fn freertos_rs_task_notify_take(clear_count: u8, wait: FreeRtosTickType) -> u32;
    pub fn freertos_rs_task_notify_wait(ulBitsToClearOnEntry: u32, ulBitsToClearOnExit: u32, pulNotificationValue: *mut u32,
                                        xTicksToWait: FreeRtosTickType)
                                        -> FreeRtosBaseType;

    pub fn freertos_rs_task_notify(task: FreeRtosTaskHandle, value: u32, action: u8) -> FreeRtosBaseType;
    pub fn freertos_rs_task_notify_isr(task: FreeRtosTaskHandle, value: u32, action: u8, xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr)
                                       -> FreeRtosBaseType;

    pub fn freertos_rs_spawn_task(f: extern "C" fn(FreeRtosMutVoidPtr) -> FreeRtosMutVoidPtr, value: FreeRtosMutVoidPtr,
                                  name: FreeRtosCharPtr, name_len: u8, stack_size: u16, priority: FreeRtosUBaseType,
                                  task_handle: FreeRtosMutTaskHandle)
                                  -> FreeRtosUBaseType;
    pub fn freertos_rs_delete_task(task: FreeRtosTaskHandle);
    pub fn freertos_rs_task_get_name(task: FreeRtosTaskHandle) -> FreeRtosCharPtr;
    pub fn freertos_rs_get_stack_high_water_mark(task: FreeRtosTaskHandle) -> FreeRtosBaseType;

    pub fn freertos_rs_get_current_task() -> FreeRtosTaskHandle;
    pub fn freertos_rs_get_system_state(tasks: *mut FreeRtosTaskStatusFfi, tasks_len: FreeRtosUBaseType, total_run_time: *mut u32)
                                        -> FreeRtosUBaseType;

    pub fn freertos_rs_max_wait() -> FreeRtosTickType;

    pub fn freertos_rs_timer_create(name: FreeRtosCharPtr, name_len: u8, period: FreeRtosTickType, auto_reload: u8,
                                    timer_id: FreeRtosVoidPtr, callback: extern "C" fn(FreeRtosTimerHandle) -> ())
                                    -> FreeRtosTimerHandle;
    pub fn freertos_rs_timer_start(timer: FreeRtosTimerHandle, block_time: FreeRtosTickType) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_stop(timer: FreeRtosTimerHandle, block_time: FreeRtosTickType) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_delete(timer: FreeRtosTimerHandle, block_time: FreeRtosTickType) -> FreeRtosBaseType;
    pub fn freertos_rs_timer_change_period(timer: FreeRtosTimerHandle, block_time: FreeRtosTickType, new_period: FreeRtosTickType)
                                           -> FreeRtosBaseType;
    pub fn freertos_rs_timer_get_id(timer: FreeRtosTimerHandle) -> FreeRtosVoidPtr;

    // #[cfg(not(target = "xtensa-esp32-none-elf"))]
    // pub fn freertos_rs_enter_critical();
    // #[cfg(not(target = "xtensa-esp32-none-elf"))]
    // pub fn freertos_rs_exit_critical();
    //
    // #[cfg(target = "xtensa-esp32-none-elf")]
    // pub fn freertos_rs_enter_critical(_mux: FreeRtosQueueHandle);
    // #[cfg(target = "xtensa-esp32-none-elf")]
    // pub fn freertos_rs_exit_critical(_mux: FreeRtosQueueHandle);
}

// mocks for testing
#[cfg(not(target_os = "none"))]
pub mod freertos_rs_mocked {
    use crate::freertos::base::*;

    pub unsafe fn freertos_rs_sizeof(_type: u8) -> u8 { 0 }

    pub unsafe fn freertos_rs_vTaskDelayUntil(_pxPreviousWakeTime: *mut FreeRtosTickType, _xTimeIncrement: FreeRtosTickType) {}
    pub unsafe fn freertos_rs_vTaskDelay(_xTicksToDelay: FreeRtosTickType) {}
    pub unsafe fn freertos_rs_get_portTICK_PERIOD_MS() -> FreeRtosTickType { 1 }
    pub unsafe fn freertos_rs_get_number_of_tasks() -> FreeRtosUBaseType { 0 }

    pub unsafe fn freertos_rs_xTaskGetTickCount() -> FreeRtosTickType { 1 }

    pub unsafe fn freertos_rs_create_recursive_mutex() -> FreeRtosQueueHandle { 1 as _ }
    pub unsafe fn freertos_rs_create_mutex() -> FreeRtosQueueHandle { 1 as _ }

    pub unsafe fn freertos_rs_take_recursive_mutex(_mutex: FreeRtosQueueHandle, _max: FreeRtosTickType) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_take_mutex(_mutex: FreeRtosQueueHandle, _max: FreeRtosTickType) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_give_mutex(_mutex: FreeRtosQueueHandle) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_give_recursive_mutex(_mutex: FreeRtosQueueHandle) -> FreeRtosBaseType { 0 }

    pub unsafe fn freertos_rs_delete_semaphore(_mutex: FreeRtosQueueHandle) {}

    pub unsafe fn freertos_rs_create_binary_semaphore() -> FreeRtosQueueHandle { 1 as _ }
    pub unsafe fn freertos_rs_create_counting_semaphore(_max: FreeRtosUBaseType, _initial: FreeRtosUBaseType) -> FreeRtosQueueHandle {
        1 as _
    }

    pub unsafe fn freertos_rs_isr_yield() {}

    pub unsafe fn freertos_rs_task_notify_take(_clear_count: u8, _wait: FreeRtosTickType) -> u32 { 0 }
    pub unsafe fn freertos_rs_task_notify_wait(_ulBitsToClearOnEntry: u32, _ulBitsToClearOnExit: u32, _pulNotificationValue: *mut u32,
                                               _xTicksToWait: FreeRtosTickType)
                                               -> FreeRtosBaseType {
        0
    }

    pub unsafe fn freertos_rs_task_notify(_task: FreeRtosTaskHandle, _value: u32, _action: u8) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_task_notify_isr(_task: FreeRtosTaskHandle, _value: u32, _action: u8,
                                              _xHigherPriorityTaskWoken: FreeRtosBaseTypeMutPtr)
                                              -> FreeRtosBaseType {
        0
    }

    pub unsafe fn freertos_rs_spawn_task(_f: extern "C" fn(FreeRtosMutVoidPtr) -> FreeRtosMutVoidPtr, _value: FreeRtosMutVoidPtr,
                                         _name: FreeRtosCharPtr, _name_len: u8, _stack_size: u16, _priority: FreeRtosUBaseType,
                                         _task_handle: FreeRtosMutTaskHandle)
                                         -> FreeRtosUBaseType {
        0
    }
    pub unsafe fn freertos_rs_delete_task(_task: FreeRtosTaskHandle) {}
    pub unsafe fn freertos_rs_task_get_name(_task: FreeRtosTaskHandle) -> FreeRtosCharPtr { 0 as _ }
    pub unsafe fn freertos_rs_get_stack_high_water_mark(_task: FreeRtosTaskHandle) -> FreeRtosBaseType { 0 }

    pub unsafe fn freertos_rs_get_current_task() -> FreeRtosTaskHandle { 1 as _ }
    pub unsafe fn freertos_rs_get_system_state(_tasks: *mut FreeRtosTaskStatusFfi, _tasks_len: FreeRtosUBaseType,
                                               _total_run_time: *mut u32)
                                               -> FreeRtosUBaseType {
        0
    }

    pub unsafe fn freertos_rs_max_wait() -> FreeRtosTickType { 1000 }

    pub unsafe fn freertos_rs_timer_create(_name: FreeRtosCharPtr, _name_len: u8, _period: FreeRtosTickType, _auto_reload: u8,
                                           _timer_id: FreeRtosVoidPtr, _callback: extern "C" fn(FreeRtosTimerHandle) -> ())
                                           -> FreeRtosTimerHandle {
        0 as _
    }
    pub unsafe fn freertos_rs_timer_start(_timer: FreeRtosTimerHandle, _block_time: FreeRtosTickType) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_timer_stop(_timer: FreeRtosTimerHandle, _block_time: FreeRtosTickType) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_timer_delete(_timer: FreeRtosTimerHandle, _block_time: FreeRtosTickType) -> FreeRtosBaseType { 0 }
    pub unsafe fn freertos_rs_timer_change_period(_timer: FreeRtosTimerHandle, _block_time: FreeRtosTickType,
                                                  _new_period: FreeRtosTickType)
                                                  -> FreeRtosBaseType {
        0
    }
    pub unsafe fn freertos_rs_timer_get_id(_timer: FreeRtosTimerHandle) -> FreeRtosVoidPtr { 0 as _ }

    pub unsafe fn freertos_rs_enter_critical(_mux: FreeRtosQueueHandle) {}
    pub unsafe fn freertos_rs_exit_critical(_mux: FreeRtosQueueHandle) {}
}

#[cfg(not(target_os = "none"))]
pub use crate::freertos::shim::freertos_rs_mocked::*;
