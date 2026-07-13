# Photon

An emulator for the w65c816 microprocessor.

Photon is an emulator written in Rust based on the [W65C816S](https://www.westerndesigncenter.com/wdc/w65c816s-chip.php) processor. It has been designed to be usable in both Rust and C, and exposes APIs for both.

# Usage

## Rust
```rust
use photon::CPU;

pub fn main() {
    let mut cpu = CPU::new();
    // TODO
    // ...

    // Cpu is destroyed by drop from memory
}
```

## C
```c
#include "photon.h"

int main(void) {
    CPU *cpu = p816CreateCpu();
    // TODO
    // ...
    p816DestroyCpu(cpu);
}

```