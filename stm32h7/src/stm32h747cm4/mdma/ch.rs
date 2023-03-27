///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - MDMA channel x interrupt/status register
    pub isr: ISR,
    ///0x04 - MDMA channel x interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08 - MDMA Channel x error status register
    pub esr: ESR,
    ///0x0c - This register is used to control the concerned channel.
    pub cr: CR,
    ///0x10 - This register is used to configure the concerned channel.
    pub tcr: TCR,
    ///0x14 - MDMA Channel x block number of data register
    pub bndtr: BNDTR,
    ///0x18 - MDMA channel x source address register
    pub sar: SAR,
    ///0x1c - MDMA channel x destination address register
    pub dar: DAR,
    ///0x20 - MDMA channel x Block Repeat address Update register
    pub brur: BRUR,
    ///0x24 - MDMA channel x Link Address register
    pub lar: LAR,
    ///0x28 - MDMA channel x Trigger and Bus selection Register
    pub tbr: TBR,
    _reserved11: [u8; 0x04],
    ///0x30 - MDMA channel x Mask address register
    pub mar: MAR,
    ///0x34 - MDMA channel x Mask Data register
    pub mdr: MDR,
    _reserved_end: [u8; 0x08],
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///MDMA channel x interrupt/status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///MDMA channel x interrupt flag clear register
pub mod ifcr;
///ESR (r) register accessor: an alias for `Reg<ESR_SPEC>`
pub type ESR = crate::Reg<esr::ESR_SPEC>;
///MDMA Channel x error status register
pub mod esr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///This register is used to control the concerned channel.
pub mod cr;
///TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
///This register is used to configure the concerned channel.
pub mod tcr;
///BNDTR (rw) register accessor: an alias for `Reg<BNDTR_SPEC>`
pub type BNDTR = crate::Reg<bndtr::BNDTR_SPEC>;
///MDMA Channel x block number of data register
pub mod bndtr;
///SAR (rw) register accessor: an alias for `Reg<SAR_SPEC>`
pub type SAR = crate::Reg<sar::SAR_SPEC>;
///MDMA channel x source address register
pub mod sar;
///DAR (rw) register accessor: an alias for `Reg<DAR_SPEC>`
pub type DAR = crate::Reg<dar::DAR_SPEC>;
///MDMA channel x destination address register
pub mod dar;
///BRUR (rw) register accessor: an alias for `Reg<BRUR_SPEC>`
pub type BRUR = crate::Reg<brur::BRUR_SPEC>;
///MDMA channel x Block Repeat address Update register
pub mod brur;
///LAR (rw) register accessor: an alias for `Reg<LAR_SPEC>`
pub type LAR = crate::Reg<lar::LAR_SPEC>;
///MDMA channel x Link Address register
pub mod lar;
///TBR (rw) register accessor: an alias for `Reg<TBR_SPEC>`
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
///MDMA channel x Trigger and Bus selection Register
pub mod tbr;
///MAR (rw) register accessor: an alias for `Reg<MAR_SPEC>`
pub type MAR = crate::Reg<mar::MAR_SPEC>;
///MDMA channel x Mask address register
pub mod mar;
///MDR (rw) register accessor: an alias for `Reg<MDR_SPEC>`
pub type MDR = crate::Reg<mdr::MDR_SPEC>;
///MDMA channel x Mask Data register
pub mod mdr;
