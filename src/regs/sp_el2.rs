// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Alban Seurat <alban.seurat@me.com>

//! The stack pointer - EL2
//!
//! Holds the stack pointer associated with EL2. When executing at EL2, the value of SPSel.SP
//! determines the current stack pointer:
//!
//! SPSel.SP | current stack pointer
//! --------------------------------
//! 0        | SP_EL0
//! 1        | SP_EL1
//! 2        | SP_EL2

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u64, ()> for Reg {
    sys_coproc_read_raw!(u64, "SP_EL2", "x");
    sys_coproc_write_raw!(u64, "SP_EL2", "x");
}

pub static SP_EL2: Reg = Reg {};
