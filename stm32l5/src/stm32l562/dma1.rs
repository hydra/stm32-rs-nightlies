///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt status register
    pub isr: ISR,
    ///0x04 - interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08 - channel x configuration register
    pub ccr1: CCR1,
    ///0x0c - channel x number of data register
    pub cndtr1: CNDTR1,
    ///0x10 - channel x peripheral address register
    pub cpar1: CPAR1,
    ///0x14 - channel x memory address register
    pub cm0ar1: CM0AR1,
    ///0x18 - channel x memory address register
    pub cm1ar1: CM1AR1,
    ///0x1c - channel x configuration register
    pub ccr2: CCR2,
    ///0x20 - channel x number of data register
    pub cndtr2: CNDTR2,
    ///0x24 - channel x peripheral address register
    pub cpar2: CPAR2,
    ///0x28 - channel x memory address register
    pub cm0ar2: CM0AR2,
    ///0x2c - channel x memory address register
    pub cm1ar2: CM1AR2,
    ///0x30 - channel x configuration register
    pub ccr3: CCR3,
    ///0x34 - channel x number of data register
    pub cndtr3: CNDTR3,
    ///0x38 - channel x peripheral address register
    pub cpar3: CPAR3,
    ///0x3c - channel x memory address register
    pub cm0ar3: CM0AR3,
    ///0x40 - channel x memory address register
    pub cm1ar3: CM1AR3,
    ///0x44 - channel x configuration register
    pub ccr4: CCR4,
    ///0x48 - channel x number of data register
    pub cndtr4: CNDTR4,
    ///0x4c - channel x peripheral address register
    pub cpar4: CPAR4,
    ///0x50 - channel x memory address register
    pub cm0ar4: CM0AR4,
    ///0x54 - channel x memory address register
    pub cm1ar4: CM1AR4,
    ///0x58 - channel x configuration register
    pub ccr5: CCR5,
    ///0x5c - channel x number of data register
    pub cndtr5: CNDTR5,
    ///0x60 - channel x peripheral address register
    pub cpar5: CPAR5,
    ///0x64 - channel x memory address register
    pub cm0ar5: CM0AR5,
    ///0x68 - channel x memory address register
    pub cm1ar5: CM1AR5,
    ///0x6c - channel x configuration register
    pub ccr6: CCR6,
    ///0x70 - channel x number of data register
    pub cndtr6: CNDTR6,
    ///0x74 - channel x peripheral address register
    pub cpar6: CPAR6,
    ///0x78 - channel x memory address register
    pub cm0ar6: CM0AR6,
    ///0x7c - channel x memory address register
    pub cm1ar6: CM1AR6,
    ///0x80 - channel x configuration register
    pub ccr7: CCR7,
    ///0x84 - channel x number of data register
    pub cndtr7: CNDTR7,
    ///0x88 - channel x peripheral address register
    pub cpar7: CPAR7,
    ///0x8c - channel x memory address register
    pub cm0ar7: CM0AR7,
    ///0x90 - channel x memory address register
    pub cm1ar7: CM1AR7,
    ///0x94 - channel x configuration register
    pub ccr8: CCR8,
    ///0x98 - channel x number of data register
    pub cndtr8: CNDTR8,
    ///0x9c - channel x peripheral address register
    pub cpar8: CPAR8,
    ///0xa0 - channel x peripheral address register
    pub cm0ar8: CM0AR8,
    ///0xa4 - channel x peripheral address register
    pub cm1ar8: CM1AR8,
    ///0xa8 - channel selection register
    pub cselr: CSELR,
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///interrupt flag clear register
pub mod ifcr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///channel x configuration register
pub mod ccr1;
///CNDTR1 (rw) register accessor: an alias for `Reg<CNDTR1_SPEC>`
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
///channel x number of data register
pub mod cndtr1;
///CPAR1 (rw) register accessor: an alias for `Reg<CPAR1_SPEC>`
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
///channel x peripheral address register
pub mod cpar1;
///CM0AR1 (rw) register accessor: an alias for `Reg<CM0AR1_SPEC>`
pub type CM0AR1 = crate::Reg<cm0ar1::CM0AR1_SPEC>;
///channel x memory address register
pub mod cm0ar1;
///CM1AR1 (rw) register accessor: an alias for `Reg<CM1AR1_SPEC>`
pub type CM1AR1 = crate::Reg<cm1ar1::CM1AR1_SPEC>;
///channel x memory address register
pub mod cm1ar1;
///CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///channel x configuration register
pub mod ccr2;
///CNDTR2 (rw) register accessor: an alias for `Reg<CNDTR2_SPEC>`
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
///channel x number of data register
pub mod cndtr2;
///CPAR2 (rw) register accessor: an alias for `Reg<CPAR2_SPEC>`
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
///channel x peripheral address register
pub mod cpar2;
///CM0AR2 (rw) register accessor: an alias for `Reg<CM0AR2_SPEC>`
pub type CM0AR2 = crate::Reg<cm0ar2::CM0AR2_SPEC>;
///channel x memory address register
pub mod cm0ar2;
///CM1AR2 (rw) register accessor: an alias for `Reg<CM1AR2_SPEC>`
pub type CM1AR2 = crate::Reg<cm1ar2::CM1AR2_SPEC>;
///channel x memory address register
pub mod cm1ar2;
///CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///channel x configuration register
pub mod ccr3;
///CNDTR3 (rw) register accessor: an alias for `Reg<CNDTR3_SPEC>`
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
///channel x number of data register
pub mod cndtr3;
///CPAR3 (rw) register accessor: an alias for `Reg<CPAR3_SPEC>`
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
///channel x peripheral address register
pub mod cpar3;
///CM0AR3 (rw) register accessor: an alias for `Reg<CM0AR3_SPEC>`
pub type CM0AR3 = crate::Reg<cm0ar3::CM0AR3_SPEC>;
///channel x memory address register
pub mod cm0ar3;
///CM1AR3 (rw) register accessor: an alias for `Reg<CM1AR3_SPEC>`
pub type CM1AR3 = crate::Reg<cm1ar3::CM1AR3_SPEC>;
///channel x memory address register
pub mod cm1ar3;
///CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
///channel x configuration register
pub mod ccr4;
///CNDTR4 (rw) register accessor: an alias for `Reg<CNDTR4_SPEC>`
pub type CNDTR4 = crate::Reg<cndtr4::CNDTR4_SPEC>;
///channel x number of data register
pub mod cndtr4;
///CPAR4 (rw) register accessor: an alias for `Reg<CPAR4_SPEC>`
pub type CPAR4 = crate::Reg<cpar4::CPAR4_SPEC>;
///channel x peripheral address register
pub mod cpar4;
///CM0AR4 (rw) register accessor: an alias for `Reg<CM0AR4_SPEC>`
pub type CM0AR4 = crate::Reg<cm0ar4::CM0AR4_SPEC>;
///channel x memory address register
pub mod cm0ar4;
///CM1AR4 (rw) register accessor: an alias for `Reg<CM1AR4_SPEC>`
pub type CM1AR4 = crate::Reg<cm1ar4::CM1AR4_SPEC>;
///channel x memory address register
pub mod cm1ar4;
///CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
///channel x configuration register
pub mod ccr5;
///CNDTR5 (rw) register accessor: an alias for `Reg<CNDTR5_SPEC>`
pub type CNDTR5 = crate::Reg<cndtr5::CNDTR5_SPEC>;
///channel x number of data register
pub mod cndtr5;
///CPAR5 (rw) register accessor: an alias for `Reg<CPAR5_SPEC>`
pub type CPAR5 = crate::Reg<cpar5::CPAR5_SPEC>;
///channel x peripheral address register
pub mod cpar5;
///CM0AR5 (rw) register accessor: an alias for `Reg<CM0AR5_SPEC>`
pub type CM0AR5 = crate::Reg<cm0ar5::CM0AR5_SPEC>;
///channel x memory address register
pub mod cm0ar5;
///CM1AR5 (rw) register accessor: an alias for `Reg<CM1AR5_SPEC>`
pub type CM1AR5 = crate::Reg<cm1ar5::CM1AR5_SPEC>;
///channel x memory address register
pub mod cm1ar5;
///CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
///channel x configuration register
pub mod ccr6;
///CNDTR6 (rw) register accessor: an alias for `Reg<CNDTR6_SPEC>`
pub type CNDTR6 = crate::Reg<cndtr6::CNDTR6_SPEC>;
///channel x number of data register
pub mod cndtr6;
///CPAR6 (rw) register accessor: an alias for `Reg<CPAR6_SPEC>`
pub type CPAR6 = crate::Reg<cpar6::CPAR6_SPEC>;
///channel x peripheral address register
pub mod cpar6;
///CM0AR6 (rw) register accessor: an alias for `Reg<CM0AR6_SPEC>`
pub type CM0AR6 = crate::Reg<cm0ar6::CM0AR6_SPEC>;
///channel x memory address register
pub mod cm0ar6;
///CM1AR6 (rw) register accessor: an alias for `Reg<CM1AR6_SPEC>`
pub type CM1AR6 = crate::Reg<cm1ar6::CM1AR6_SPEC>;
///channel x memory address register
pub mod cm1ar6;
///CCR7 (rw) register accessor: an alias for `Reg<CCR7_SPEC>`
pub type CCR7 = crate::Reg<ccr7::CCR7_SPEC>;
///channel x configuration register
pub mod ccr7;
///CNDTR7 (rw) register accessor: an alias for `Reg<CNDTR7_SPEC>`
pub type CNDTR7 = crate::Reg<cndtr7::CNDTR7_SPEC>;
///channel x number of data register
pub mod cndtr7;
///CPAR7 (rw) register accessor: an alias for `Reg<CPAR7_SPEC>`
pub type CPAR7 = crate::Reg<cpar7::CPAR7_SPEC>;
///channel x peripheral address register
pub mod cpar7;
///CM0AR7 (rw) register accessor: an alias for `Reg<CM0AR7_SPEC>`
pub type CM0AR7 = crate::Reg<cm0ar7::CM0AR7_SPEC>;
///channel x memory address register
pub mod cm0ar7;
///CM1AR7 (rw) register accessor: an alias for `Reg<CM1AR7_SPEC>`
pub type CM1AR7 = crate::Reg<cm1ar7::CM1AR7_SPEC>;
///channel x memory address register
pub mod cm1ar7;
///CCR8 (rw) register accessor: an alias for `Reg<CCR8_SPEC>`
pub type CCR8 = crate::Reg<ccr8::CCR8_SPEC>;
///channel x configuration register
pub mod ccr8;
///CNDTR8 (rw) register accessor: an alias for `Reg<CNDTR8_SPEC>`
pub type CNDTR8 = crate::Reg<cndtr8::CNDTR8_SPEC>;
///channel x number of data register
pub mod cndtr8;
///CPAR8 (rw) register accessor: an alias for `Reg<CPAR8_SPEC>`
pub type CPAR8 = crate::Reg<cpar8::CPAR8_SPEC>;
///channel x peripheral address register
pub mod cpar8;
///CM0AR8 (rw) register accessor: an alias for `Reg<CM0AR8_SPEC>`
pub type CM0AR8 = crate::Reg<cm0ar8::CM0AR8_SPEC>;
///channel x peripheral address register
pub mod cm0ar8;
///CM1AR8 (rw) register accessor: an alias for `Reg<CM1AR8_SPEC>`
pub type CM1AR8 = crate::Reg<cm1ar8::CM1AR8_SPEC>;
///channel x peripheral address register
pub mod cm1ar8;
///CSELR (rw) register accessor: an alias for `Reg<CSELR_SPEC>`
pub type CSELR = crate::Reg<cselr::CSELR_SPEC>;
///channel selection register
pub mod cselr;
