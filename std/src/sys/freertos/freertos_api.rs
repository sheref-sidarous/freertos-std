
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::ffi::{c_void, c_char};
use crate::ptr;

pub type TickType_t = u32;
pub type BaseType_t = i32;
pub type configSTACK_DEPTH_TYPE = u16;
pub type UBaseType_t = u32;
pub type TaskFunction_t = extern "C" fn (*mut c_void);
pub type SemaphoreHandle_t = *mut c_void;
pub type TaskHandle_t = *mut c_void;

pub const portMAX_DELAY : TickType_t = 0xffffffffu32;
pub const queueQUEUE_TYPE_MUTEX : u8 = 1;
pub const semGIVE_BLOCK_TIME : TickType_t = 0;
pub const queueSEND_TO_BACK : BaseType_t = 0;


pub const pdFALSE            : BaseType_t = 0;
pub const pdTRUE             : BaseType_t = 1;
pub const pdFALSE_SIGNED     : BaseType_t = 0;
pub const pdTRUE_SIGNED      : BaseType_t = 1;
pub const pdFALSE_UNSIGNED   : UBaseType_t = 0;
pub const pdTRUE_UNSIGNED    : UBaseType_t = 1;

pub const errQUEUE_EMPTY     : BaseType_t = 0;
pub const errQUEUE_FULL      : BaseType_t = 0;


pub const pdPASS : BaseType_t = pdTRUE;
pub const pdFAIL : BaseType_t = pdFALSE;

// TODO: better Freertos task priority handling
pub const DefaultTaskPriority : UBaseType_t = 5u32;

pub const UBaseType_max : UBaseType_t = u32::MAX;

extern "C" {

    pub fn vTaskDelay(xTicksToDelay : TickType_t);
    pub fn uart_write ( buff : *const u8, buff_len : usize);

    pub fn xTaskCreate (pxTaskCode : TaskFunction_t,
        pcName : *const c_char,
        usStackDepth : configSTACK_DEPTH_TYPE,
        pvParameters : *const c_void,
        uxPriority : UBaseType_t,
        pxCreatedTask : *mut TaskHandle_t ) -> BaseType_t;


    // semaphore API
    pub fn rust_std_xSemaphoreCreateMutex() -> SemaphoreHandle_t;

    pub fn rust_std_xSemaphoreCreateCounting(
        uxMaxCount : UBaseType_t,
        uxInitialCount : UBaseType_t) -> SemaphoreHandle_t;

    pub fn rust_std_xSemaphoreTake(xSemaphore : SemaphoreHandle_t, xTicksToWait : TickType_t) -> bool;

    pub fn rust_std_xSemaphoreGive(xSemaphore : SemaphoreHandle_t);

    pub fn rust_std_vSemaphoreDelete(xSemaphore : SemaphoreHandle_t);


    // task related API
    pub fn rust_std_taskYIELD();

    pub fn rust_std_msec_to_ticks(millis : u32) -> TickType_t;

    pub fn rust_std_get_configNUM_THREAD_LOCAL_STORAGE_POINTERS () -> u32;

    pub fn rust_std_vTaskSetThreadLocalStoragePointer(
        xTaskToSet : TaskHandle_t,
        xIndex : BaseType_t, pvValue : *mut c_void );

    pub fn rust_std_pvTaskGetThreadLocalStoragePointer(
        xTaskToQuery : TaskHandle_t,
        xIndex : BaseType_t ) -> *mut c_void;

    pub fn rust_std_vTaskDelete( xTask : TaskHandle_t );

    pub fn rust_std_xTaskGetTickCount( ) -> TickType_t;

    pub fn rust_std_ticks_to_msec (ticks : TickType_t) -> u32;

}
