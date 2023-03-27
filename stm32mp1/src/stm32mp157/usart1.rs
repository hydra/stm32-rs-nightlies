///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: CR1,
    ///0x04 - Control register 2
    pub cr2: CR2,
    ///0x08 - Control register 3
    pub cr3: CR3,
    ///0x0c - Baud rate register
    pub brr: BRR,
    ///0x10 - Guard time and prescaler register
    pub gtpr: GTPR,
    ///0x14 - Receiver timeout register
    pub rtor: RTOR,
    ///0x18 - Request register
    pub rqr: RQR,
    ///0x1c - Interrupt &amp; status register
    pub isr: ISR,
    ///0x20 - Interrupt flag clear register
    pub icr: ICR,
    ///0x24 - Receive data register
    pub rdr: RDR,
    ///0x28 - Transmit data register
    pub tdr: TDR,
    ///0x2c - Prescaler register
    pub presc: PRESC,
    _reserved12: [u8; 0x03bc],
    ///0x3ec - USART Hardware Configuration register 2
    pub hwcfgr2: HWCFGR2,
    ///0x3f0 - USART Hardware Configuration register 1
    pub hwcfgr1: HWCFGR1,
    ///0x3f4 - EXTI IP Version register
    pub verr: VERR,
    ///0x3f8 - EXTI Identification register
    pub ipidr: IPIDR,
    ///0x3fc - EXTI Size ID register
    pub sidr: SIDR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Control register 3
pub mod cr3;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Baud rate register
pub mod brr;
///GTPR (rw) register accessor: an alias for `Reg<GTPR_SPEC>`
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///Guard time and prescaler register
pub mod gtpr;
///RTOR (rw) register accessor: an alias for `Reg<RTOR_SPEC>`
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
///Receiver timeout register
pub mod rtor;
///RQR (w) register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///Request register
pub mod rqr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt &amp; status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt flag clear register
pub mod icr;
///RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///Receive data register
pub mod rdr;
///TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///Transmit data register
pub mod tdr;
///PRESC (rw) register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///Prescaler register
pub mod presc;
///HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///USART Hardware Configuration register 2
pub mod hwcfgr2;
///HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///USART Hardware Configuration register 1
pub mod hwcfgr1;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///EXTI IP Version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///EXTI Identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///EXTI Size ID register
pub mod sidr;
