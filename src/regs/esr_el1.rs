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

//! Exception Status Register - EL1
//!
//! When taking an exception to EL1, holds the status of the exception.

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    ESR_EL1 [

        /// Exception Class.
        EC OFFSET(26) NUMBITS(6) [
            InstructionAbortLowerLevel = 0b100000,
            InstructionAbort = 0b100001
        ],

        /// Instruction Length. The possible values of this bit are:
        ///
        /// 0 16bits.
        /// 1 32bits.
        IL OFFSET(25) NUMBITS(1) [
            WordSize = 0,
            DWordSize = 1
        ],

        /// Instruction Specific Syndrome.
        ISS OFFSET(0) NUMBITS(25) []

    ]
}


pub struct Reg;

impl RegisterReadWrite<u32, ESR_EL1::Register> for Reg {
    sys_coproc_read_raw!(u32, "ESR_EL1");
    sys_coproc_write_raw!(u32, "ESR_EL1");
}

pub static ESR_EL1: Reg = Reg {};
