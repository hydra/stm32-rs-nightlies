///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    ///0x04 - LPUART control register 2
    pub cr2: CR2,
    ///0x08 - LPUART control register 3
    pub cr3: CR3,
    ///0x0c - LPUART baud rate register
    pub brr: BRR,
    _reserved4: [u8; 0x08],
    ///0x18 - LPUART request register
    pub rqr: RQR,
    _reserved_5_isr: [u8; 0x04],
    ///0x20 - LPUART interrupt flag clear register
    pub icr: ICR,
    ///0x24 - LPUART receive data register
    pub rdr: RDR,
    ///0x28 - LPUART transmit data register
    pub tdr: TDR,
    ///0x2c - LPUART prescaler register
    pub presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - LPUART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - LPUART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x1c - LPUART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - LPUART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///CR1_enabled (rw) register accessor: an alias for `Reg<CR1_ENABLED_SPEC>`
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>;
///LPUART control register 1 \[alternate\]
pub mod cr1_enabled;
///CR1_disabled (rw) register accessor: an alias for `Reg<CR1_DISABLED_SPEC>`
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>;
///LPUART control register 1 \[alternate\]
pub mod cr1_disabled;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///LPUART control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///LPUART control register 3
pub mod cr3;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///LPUART baud rate register
pub mod brr;
///RQR (w) register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///LPUART request register
pub mod rqr;
///ISR_enabled (r) register accessor: an alias for `Reg<ISR_ENABLED_SPEC>`
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLED_SPEC>;
///LPUART interrupt and status register \[alternate\]
pub mod isr_enabled;
///ISR_disabled (r) register accessor: an alias for `Reg<ISR_DISABLED_SPEC>`
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLED_SPEC>;
///LPUART interrupt and status register \[alternate\]
pub mod isr_disabled;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///LPUART interrupt flag clear register
pub mod icr;
///RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///LPUART receive data register
pub mod rdr;
///TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///LPUART transmit data register
pub mod tdr;
///PRESC (rw) register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///LPUART prescaler register
pub mod presc;
