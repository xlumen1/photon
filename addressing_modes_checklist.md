## Addressing Modes

### Implied
- [x] `Implied`
    - No operand fetch needed. The instruction operates directly on registers or flags (e.g., NOP, CLC, INX).

### Immediate (Accumulator/Index)
- [x] `ImmediateAcc` → `addr_imm_acc()`
    - Operand is fetched directly from the next byte(s) after the opcode. Used for instructions like LDA #imm. Size depends on M/X flags.
- [x] `ImmediateIdx` → `addr_imm_idx()`
    - Same as above, but for index registers (X/Y). Size depends on X flag.

### Absolute
- [x] `Absolute` → `addr_abs()`
    - Fetch 16-bit address from the next two bytes. Access memory at (DB << 16) | address.
- [x] `Absolute,X` → `addr_abs_x()`
    - Fetch 16-bit base address, add X register. Access memory at (DB << 16) | (base + X).
- [x] `Absolute,Y` → `addr_abs_y()`
    - Fetch 16-bit base address, add Y register. Access memory at (DB << 16) | (base + Y).

### Direct Page (Zero Page)
- [x] `DirectPage` → `addr_dp()`
    - Fetch 8-bit offset, add to Direct Page register (DP). Access memory at DP + offset.
- [x] `DirectPage,X` → `addr_dp_x()`
    - Fetch 8-bit offset, add DP and X. Access memory at DP + offset + X.
- [x] `DirectPage,Y` → `addr_dp_y()`
    - Fetch 8-bit offset, add DP and Y. Access memory at DP + offset + Y.

### Long
- [x] `Long` → `addr_long()`
    - Fetch 24-bit address (3 bytes: low, mid, high). Access memory at that address.
- [x] `Long,X` → `addr_long_x()`
    - Fetch 24-bit base address, add X register. Access memory at (base + X).
- [x] `Long,Y` → `addr_long_y()`
    - Fetch 24-bit base address, add Y register. Access memory at (base + Y).

### Indirect
- [x] `(a)` Indirect Absolute → `addr_ind_abs()`
    - Fetch 16-bit pointer, read 16-bit address from memory at (DB << 16) | pointer. Use that as the effective address.
- [x] `(dp)` Indirect Direct Page → `addr_dp_ind()`
    - Fetch 8-bit offset, add to DP. Read 16-bit address from DP + offset. Use (DB << 16) | address.
- [x] `(dp,X)` Indirect Direct Page,X → `addr_dp_ind()_x`
    - Fetch 8-bit offset, add DP and X. Read 16-bit address from DP + offset + X. Use (DB << 16) | address.
- [x] `(dp),Y` Indirect Direct Page Indirect,Y → `addr_dp_ind_y()`
    - Fetch 8-bit offset, add to DP. Read 16-bit address from DP + offset, then add Y. Use (DB << 16) | (address + Y).
- [x] `[dp]` Indirect Long Direct Page → `addr_dp_long_ind()`
    - Fetch 8-bit offset, add to DP. Read 24-bit address (lo, mid, hi) from DP + offset. Use that as the effective address.
- [/] `[dp],Y` Indirect Long Direct Page,Y → `addr_dp_long_ind_y()`
    - Fetch 8-bit offset, add to DP. Read 24-bit address from DP + offset, then add Y. Use (address + Y).

### Stack Relative
- [x] `SR` Stack Relative → `addr_sr()`
    - Fetch 8-bit offset, add to Stack Pointer (SP). Access memory at SP + offset.
- [x] `SR Indirect,Y` Stack Relative Indirect,Y → `addr_sr_ind_y()`
    - Fetch 8-bit offset, add to SP. Read 16-bit address from SP + offset, then add Y. Use (DB << 16) | (address + Y).

### Relative
- [x] `Relative` → `addr_rel()`
    - Fetch 8-bit signed offset, add to Program Counter (PC). Used for branch instructions.
- [x] `RelativeLong` → `addr_rel_long()`
    - Fetch 16-bit signed offset, add to PC. Used for long branch instructions.
