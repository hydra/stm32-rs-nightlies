///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - EXTI rising trigger selection register
    pub rtsr1: RTSR1,
    ///0x04 - EXTI falling trigger selection register
    pub ftsr1: FTSR1,
    ///0x08 - EXTI software interrupt event register
    pub swier1: SWIER1,
    ///0x0c - EXTI D3 pending mask register
    pub d3pmr1: D3PMR1,
    ///0x10 - EXTI D3 pending clear selection register low
    pub d3pcr1l: D3PCR1L,
    ///0x14 - EXTI D3 pending clear selection register high
    pub d3pcr1h: D3PCR1H,
    _reserved6: [u8; 0x08],
    ///0x20 - EXTI rising trigger selection register
    pub rtsr2: RTSR2,
    ///0x24 - EXTI falling trigger selection register
    pub ftsr2: FTSR2,
    ///0x28 - EXTI software interrupt event register
    pub swier2: SWIER2,
    ///0x2c - EXTI D3 pending mask register
    pub d3pmr2: D3PMR2,
    ///0x30 - EXTI D3 pending clear selection register low
    pub d3pcr2l: D3PCR2L,
    ///0x34 - EXTI D3 pending clear selection register high
    pub d3pcr2h: D3PCR2H,
    _reserved12: [u8; 0x08],
    ///0x40 - EXTI rising trigger selection register
    pub rtsr3: RTSR3,
    ///0x44 - EXTI falling trigger selection register
    pub ftsr3: FTSR3,
    ///0x48 - EXTI software interrupt event register
    pub swier3: SWIER3,
    ///0x4c - EXTI D3 pending mask register
    pub d3pmr3: D3PMR3,
    _reserved16: [u8; 0x04],
    ///0x54 - EXTI D3 pending clear selection register high
    pub d3pcr3h: D3PCR3H,
    _reserved17: [u8; 0x28],
    ///0x80 - EXTI interrupt mask register
    pub cpuimr1: CPUIMR1,
    ///0x84 - EXTI event mask register
    pub cpuemr1: CPUEMR1,
    ///0x88 - EXTI pending register
    pub cpupr1: CPUPR1,
    _reserved20: [u8; 0x04],
    ///0x90 - EXTI interrupt mask register
    pub cpuimr2: CPUIMR2,
    ///0x94 - EXTI event mask register
    pub cpuemr2: CPUEMR2,
    ///0x98 - EXTI pending register
    pub cpupr2: CPUPR2,
    _reserved23: [u8; 0x04],
    ///0xa0 - EXTI interrupt mask register
    pub cpuimr3: CPUIMR3,
    ///0xa4 - EXTI event mask register
    pub cpuemr3: CPUEMR3,
    ///0xa8 - EXTI pending register
    pub cpupr3: CPUPR3,
}
///RTSR1 (rw) register accessor: an alias for `Reg<RTSR1_SPEC>`
pub type RTSR1 = crate::Reg<rtsr1::RTSR1_SPEC>;
///EXTI rising trigger selection register
pub mod rtsr1;
///FTSR1 (rw) register accessor: an alias for `Reg<FTSR1_SPEC>`
pub type FTSR1 = crate::Reg<ftsr1::FTSR1_SPEC>;
///EXTI falling trigger selection register
pub mod ftsr1;
///SWIER1 (rw) register accessor: an alias for `Reg<SWIER1_SPEC>`
pub type SWIER1 = crate::Reg<swier1::SWIER1_SPEC>;
///EXTI software interrupt event register
pub mod swier1;
///D3PMR1 (rw) register accessor: an alias for `Reg<D3PMR1_SPEC>`
pub type D3PMR1 = crate::Reg<d3pmr1::D3PMR1_SPEC>;
///EXTI D3 pending mask register
pub mod d3pmr1;
///D3PCR1L (rw) register accessor: an alias for `Reg<D3PCR1L_SPEC>`
pub type D3PCR1L = crate::Reg<d3pcr1l::D3PCR1L_SPEC>;
///EXTI D3 pending clear selection register low
pub mod d3pcr1l;
///D3PCR1H (rw) register accessor: an alias for `Reg<D3PCR1H_SPEC>`
pub type D3PCR1H = crate::Reg<d3pcr1h::D3PCR1H_SPEC>;
///EXTI D3 pending clear selection register high
pub mod d3pcr1h;
///RTSR2 (rw) register accessor: an alias for `Reg<RTSR2_SPEC>`
pub type RTSR2 = crate::Reg<rtsr2::RTSR2_SPEC>;
///EXTI rising trigger selection register
pub mod rtsr2;
///FTSR2 (rw) register accessor: an alias for `Reg<FTSR2_SPEC>`
pub type FTSR2 = crate::Reg<ftsr2::FTSR2_SPEC>;
///EXTI falling trigger selection register
pub mod ftsr2;
///SWIER2 (rw) register accessor: an alias for `Reg<SWIER2_SPEC>`
pub type SWIER2 = crate::Reg<swier2::SWIER2_SPEC>;
///EXTI software interrupt event register
pub mod swier2;
///D3PMR2 (rw) register accessor: an alias for `Reg<D3PMR2_SPEC>`
pub type D3PMR2 = crate::Reg<d3pmr2::D3PMR2_SPEC>;
///EXTI D3 pending mask register
pub mod d3pmr2;
///D3PCR2L (rw) register accessor: an alias for `Reg<D3PCR2L_SPEC>`
pub type D3PCR2L = crate::Reg<d3pcr2l::D3PCR2L_SPEC>;
///EXTI D3 pending clear selection register low
pub mod d3pcr2l;
///D3PCR2H (rw) register accessor: an alias for `Reg<D3PCR2H_SPEC>`
pub type D3PCR2H = crate::Reg<d3pcr2h::D3PCR2H_SPEC>;
///EXTI D3 pending clear selection register high
pub mod d3pcr2h;
///RTSR3 (rw) register accessor: an alias for `Reg<RTSR3_SPEC>`
pub type RTSR3 = crate::Reg<rtsr3::RTSR3_SPEC>;
///EXTI rising trigger selection register
pub mod rtsr3;
///FTSR3 (rw) register accessor: an alias for `Reg<FTSR3_SPEC>`
pub type FTSR3 = crate::Reg<ftsr3::FTSR3_SPEC>;
///EXTI falling trigger selection register
pub mod ftsr3;
///SWIER3 (rw) register accessor: an alias for `Reg<SWIER3_SPEC>`
pub type SWIER3 = crate::Reg<swier3::SWIER3_SPEC>;
///EXTI software interrupt event register
pub mod swier3;
///D3PMR3 (rw) register accessor: an alias for `Reg<D3PMR3_SPEC>`
pub type D3PMR3 = crate::Reg<d3pmr3::D3PMR3_SPEC>;
///EXTI D3 pending mask register
pub mod d3pmr3;
///D3PCR3H (rw) register accessor: an alias for `Reg<D3PCR3H_SPEC>`
pub type D3PCR3H = crate::Reg<d3pcr3h::D3PCR3H_SPEC>;
///EXTI D3 pending clear selection register high
pub mod d3pcr3h;
///CPUIMR1 (rw) register accessor: an alias for `Reg<CPUIMR1_SPEC>`
pub type CPUIMR1 = crate::Reg<cpuimr1::CPUIMR1_SPEC>;
///EXTI interrupt mask register
pub mod cpuimr1;
///CPUEMR1 (rw) register accessor: an alias for `Reg<CPUEMR1_SPEC>`
pub type CPUEMR1 = crate::Reg<cpuemr1::CPUEMR1_SPEC>;
///EXTI event mask register
pub mod cpuemr1;
///CPUPR1 (rw) register accessor: an alias for `Reg<CPUPR1_SPEC>`
pub type CPUPR1 = crate::Reg<cpupr1::CPUPR1_SPEC>;
///EXTI pending register
pub mod cpupr1;
///CPUIMR2 (rw) register accessor: an alias for `Reg<CPUIMR2_SPEC>`
pub type CPUIMR2 = crate::Reg<cpuimr2::CPUIMR2_SPEC>;
///EXTI interrupt mask register
pub mod cpuimr2;
///CPUEMR2 (rw) register accessor: an alias for `Reg<CPUEMR2_SPEC>`
pub type CPUEMR2 = crate::Reg<cpuemr2::CPUEMR2_SPEC>;
///EXTI event mask register
pub mod cpuemr2;
///CPUPR2 (rw) register accessor: an alias for `Reg<CPUPR2_SPEC>`
pub type CPUPR2 = crate::Reg<cpupr2::CPUPR2_SPEC>;
///EXTI pending register
pub mod cpupr2;
///CPUIMR3 (rw) register accessor: an alias for `Reg<CPUIMR3_SPEC>`
pub type CPUIMR3 = crate::Reg<cpuimr3::CPUIMR3_SPEC>;
///EXTI interrupt mask register
pub mod cpuimr3;
///CPUEMR3 (rw) register accessor: an alias for `Reg<CPUEMR3_SPEC>`
pub type CPUEMR3 = crate::Reg<cpuemr3::CPUEMR3_SPEC>;
///EXTI event mask register
pub mod cpuemr3;
///CPUPR3 (rw) register accessor: an alias for `Reg<CPUPR3_SPEC>`
pub type CPUPR3 = crate::Reg<cpupr3::CPUPR3_SPEC>;
///EXTI pending register
pub mod cpupr3;
