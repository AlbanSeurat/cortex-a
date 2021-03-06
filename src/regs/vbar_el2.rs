// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2021 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Alban Seurat <alban.seurat@me.com>

//! Vector Base Address Register - EL2
//!
//! Holds the vector base address for any exception that is taken to EL2.

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "VBAR_EL2", "x");
    sys_coproc_write_raw!(u64, "VBAR_EL2", "x");
}

pub static VBAR_EL2: Reg = Reg {};
