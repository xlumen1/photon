#ifndef PHOTON_H
#define PHOTON_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef PHOTON_LEGACYNAMES

#define create_cpu() p816CreateCpu()
#define step(cpu) p816Step(cpu)
#define cycle(cpu) p816Cycle(cpu)
#define destroy_cpu(cpu) p816DestroyCpu(cpu)

#define is_halted(cpu) p816IsHalted(cpu)
#define is_ready(cpu) p816IsReady(cpu)
#define reset(cpu) p816Reset(cpu)

#define set_callbacks(cpu, read, write) p816SetCallbacks(cpu, read, write)

#define export_state(cpu) p816ExportState(cpu)
#define import_state(cpu, state) p816ImportState(cpu, state)

#define request_nmi(cpu) p816RequestNMI(cpu)
#define set_irq(cpu, state) p816SetIRQ(cpu, state)

#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef struct CPU CPU;

typedef struct {
    uint16_t x;
    uint16_t y;
    uint16_t a;

    uint16_t dp;
    uint8_t db;

    uint16_t sp;

    uint16_t pc;
    uint8_t pb;

    uint8_t status;
    bool emulation;
} CPU_State;

typedef uint8_t (*p816Read_callback_t)(uint32_t addr);
typedef void (*p816Write_callback_t)(uint32_t addr, uint8_t val);

/* Lifecycle */
CPU* p816CreateCpu(void);
uint64_t p816Step(CPU *cpu);
void p816Cycle(CPU *cpu);
void p816DestroyCpu(CPU *cpu);

uint8_t p816IsHalted(const CPU *cpu);
uint8_t p816IsReady(const CPU *cpu);
void p816Reset(CPU *cpu);

void p816RequestNMI(CPU *cpu);
void p816SetIRQ(CPU *cpu, bool state);

/* Memory */
/**
 * The callbacks may receive any address from 0 [0x00000000] to 16777215 [0x00FFFFFF] (inclusive)
 */
size_t p816SetCallbacks(CPU *cpu, p816Read_callback_t read_cb, p816Write_callback_t write_cb);

/* Peek/Poke */

CPU_State p816ExportState(const CPU *cpu);
void p816ImportState(CPU *cpu, CPU_State state);

#ifdef __cplusplus
}
#endif

#endif
