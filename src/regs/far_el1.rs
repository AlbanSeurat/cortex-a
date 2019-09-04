/*
 * Copyright (c) 2019 by the author(s)
 *
 * =============================================================================
 *
 * Licensed under either of
 *   - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
 *   - MIT License (http://opensource.org/licenses/MIT)
 * at your option.
 *
 * =============================================================================
 *
 * Author(s):
 *   - Andre Richter <andre.o.richter@gmail.com>
 *   - Alban Seurat <alban.seurat@me.com>
 */

//! Fault Address Register - EL1
//!
//! When taking an exception to EL1, holds the faulting Virtual Address .

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "FAR_EL1");
    sys_coproc_write_raw!(u64, "FAR_EL1");
}

pub static FAR_EL1: Reg = Reg {};
