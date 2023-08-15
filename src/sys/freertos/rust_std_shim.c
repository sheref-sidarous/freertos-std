
/*
Why this file is needed ?
* To capture Freertos configurations that are expressed as #defines in its headers
  for example: configTICK_RATE_HZ
* Many Freertos APIs resolve to other APIs via macros, for example
  xSemaphoreCreateMutex is a macro to xQueueCreateMutex( queueQUEUE_TYPE_MUTEX )



*/

/* Standard includes. */
#include <stdio.h>

/* Kernel includes. */
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"
#include "semphr.h"

#if configNUM_THREAD_LOCAL_STORAGE_POINTERS < 1
#error "std Rust needs at least pne thread local storage slot"
#endif

// Some APIs must be called only when the scheduler is started
#define SCHEDULER_STARTED_GUARD() configASSERT ( xTaskGetSchedulerState() != taskSCHEDULER_NOT_STARTED )


void rust_std_taskYIELD(void)
{
  SCHEDULER_STARTED_GUARD();
  taskYIELD();
}

void rust_std_vTaskDelete( TaskHandle_t xTask ) {
  SCHEDULER_STARTED_GUARD();
  vTaskDelete(xTask);
}

SemaphoreHandle_t rust_std_xSemaphoreCreateCounting(UBaseType_t uxMaxCount,
                                                    UBaseType_t uxInitialCount)
{
  return xSemaphoreCreateCounting(uxMaxCount, uxInitialCount);
}

SemaphoreHandle_t rust_std_xSemaphoreCreateMutex(void)
{
  return xSemaphoreCreateMutex();
}

SemaphoreHandle_t rust_std_xSemaphoreCreateBinary(void)
{
  return xSemaphoreCreateBinary();
}

void rust_std_xSemaphoreTake(SemaphoreHandle_t xSemaphore,
                    TickType_t xTicksToWait)
{
  //SCHEDULER_STARTED_GUARD();
  xSemaphoreTake(xSemaphore, xTicksToWait);
}

void rust_std_xSemaphoreGive(SemaphoreHandle_t xSemaphore)
{
  //SCHEDULER_STARTED_GUARD();
  xSemaphoreGive(xSemaphore);
}

void rust_std_vSemaphoreDelete(SemaphoreHandle_t xSemaphore)
{
  vSemaphoreDelete(xSemaphore);
}

TickType_t rust_std_msec_to_ticks (uint32_t millis) {
  return millis * portTICK_PERIOD_MS;
}

uint32_t rust_std_get_configNUM_THREAD_LOCAL_STORAGE_POINTERS () {
  return configNUM_THREAD_LOCAL_STORAGE_POINTERS;
}


void rust_std_vTaskSetThreadLocalStoragePointer( TaskHandle_t xTaskToSet,
                                        BaseType_t xIndex,
                                        void * pvValue ) {
    vTaskSetThreadLocalStoragePointer(xTaskToSet, xIndex, pvValue);
}


void * rust_std_pvTaskGetThreadLocalStoragePointer( TaskHandle_t xTaskToQuery,
                                            BaseType_t xIndex ) {
    return pvTaskGetThreadLocalStoragePointer( xTaskToQuery, xIndex );
}

TickType_t rust_std_xTaskGetTickCount( void ) {
  return xTaskGetTickCount();
}

uint32_t rust_std_ticks_to_msec (TickType_t ticks) {
  return ticks / portTICK_PERIOD_MS;
}
