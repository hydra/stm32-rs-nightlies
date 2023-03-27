///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr1: [u8; 0x04],
    ///0x04 - USART control register 2
    pub cr2: CR2,
    ///0x08 - USART control register 3
    pub cr3: CR3,
    ///0x0c - USART baud rate register
    pub brr: BRR,
    ///0x10 - USART guard time and prescaler register
    pub gtpr: GTPR,
    ///0x14 - USART receiver timeout register
    pub rtor: RTOR,
    ///0x18 - USART request register
    pub rqr: RQR,
    _reserved_7_isr: [u8; 0x04],
    ///0x20 - USART interrupt flag clear register
    pub icr: ICR,
    ///0x24 - USART receive data register
    pub rdr: RDR,
    ///0x28 - USART transmit data register
    pub tdr: TDR,
    ///0x2c - USART prescaler register
    pub presc: PRESC,
}
impl RegisterBlock {
    ///0x00 - USART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_disabled(&self) -> &CR1_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - USART control register 1 \[alternate\]
    #[inline(always)]
    pub const fn cr1_enabled(&self) -> &CR1_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x1c - USART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_disabled(&self) -> &ISR_DISABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - USART interrupt and status register \[alternate\]
    #[inline(always)]
    pub const fn isr_enabled(&self) -> &ISR_ENABLED {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
///CR1_enabled (rw) register accessor: an alias for `Reg<CR1_ENABLED_SPEC>`
pub type CR1_ENABLED = crate::Reg<cr1_enabled::CR1_ENABLED_SPEC>;
///USART control register 1 \[alternate\]
pub mod cr1_enabled;
///CR1_disabled (rw) register accessor: an alias for `Reg<CR1_DISABLED_SPEC>`
pub type CR1_DISABLED = crate::Reg<cr1_disabled::CR1_DISABLED_SPEC>;
///USART control register 1 \[alternate\]
pub mod cr1_disabled;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///USART control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///USART control register 3
pub mod cr3;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///USART baud rate register
pub mod brr;
///GTPR (rw) register accessor: an alias for `Reg<GTPR_SPEC>`
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///USART guard time and prescaler register
pub mod gtpr;
///RTOR (rw) register accessor: an alias for `Reg<RTOR_SPEC>`
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
///USART receiver timeout register
pub mod rtor;
///RQR (w) register accessor: an alias for `Reg<RQR_SPEC>`
pub type RQR = crate::Reg<rqr::RQR_SPEC>;
///USART request register
pub mod rqr;
///ISR_enabled (r) register accessor: an alias for `Reg<ISR_ENABLED_SPEC>`
pub type ISR_ENABLED = crate::Reg<isr_enabled::ISR_ENABLED_SPEC>;
///USART interrupt and status register \[alternate\]
pub mod isr_enabled;
///ISR_disabled (r) register accessor: an alias for `Reg<ISR_DISABLED_SPEC>`
pub type ISR_DISABLED = crate::Reg<isr_disabled::ISR_DISABLED_SPEC>;
///USART interrupt and status register \[alternate\]
pub mod isr_disabled;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///USART interrupt flag clear register
pub mod icr;
///RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///USART receive data register
pub mod rdr;
///TDR (rw) register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///USART transmit data register
pub mod tdr;
///PRESC (rw) register accessor: an alias for `Reg<PRESC_SPEC>`
pub type PRESC = crate::Reg<presc::PRESC_SPEC>;
///USART prescaler register
pub mod presc;
