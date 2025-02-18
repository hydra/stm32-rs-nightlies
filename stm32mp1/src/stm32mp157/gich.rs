///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GICH hypervisor control register
    pub gich_hcr: GICH_HCR,
    ///0x04 - GICH VGIC type register
    pub gich_vtr: GICH_VTR,
    ///0x08 - GICH virtual machine control register
    pub gich_vmcr: GICH_VMCR,
    _reserved3: [u8; 0x04],
    ///0x10 - GICH maintenance interrupt status register
    pub gich_misr: GICH_MISR,
    _reserved4: [u8; 0x0c],
    ///0x20 - GICH end of interrupt status register
    pub gich_eisr0: GICH_EISR0,
    _reserved5: [u8; 0x0c],
    ///0x30 - GICH empty list status register
    pub gich_elsr0: GICH_ELSR0,
    _reserved6: [u8; 0xbc],
    ///0xf0 - GICH active priority register
    pub gich_apr0: GICH_APR0,
    _reserved7: [u8; 0x0c],
    ///0x100 - GICH list register 0
    pub gich_lr0: GICH_LR0,
    ///0x104 - GICH list register 1
    pub gich_lr1: GICH_LR1,
    ///0x108 - GICH list register 2
    pub gich_lr2: GICH_LR2,
    ///0x10c - GICH list register 3
    pub gich_lr3: GICH_LR3,
}
///GICH_HCR (rw) register accessor: an alias for `Reg<GICH_HCR_SPEC>`
pub type GICH_HCR = crate::Reg<gich_hcr::GICH_HCR_SPEC>;
///GICH hypervisor control register
pub mod gich_hcr;
///GICH_VTR (r) register accessor: an alias for `Reg<GICH_VTR_SPEC>`
pub type GICH_VTR = crate::Reg<gich_vtr::GICH_VTR_SPEC>;
///GICH VGIC type register
pub mod gich_vtr;
///GICH_VMCR (rw) register accessor: an alias for `Reg<GICH_VMCR_SPEC>`
pub type GICH_VMCR = crate::Reg<gich_vmcr::GICH_VMCR_SPEC>;
///GICH virtual machine control register
pub mod gich_vmcr;
///GICH_MISR (r) register accessor: an alias for `Reg<GICH_MISR_SPEC>`
pub type GICH_MISR = crate::Reg<gich_misr::GICH_MISR_SPEC>;
///GICH maintenance interrupt status register
pub mod gich_misr;
///GICH_EISR0 (r) register accessor: an alias for `Reg<GICH_EISR0_SPEC>`
pub type GICH_EISR0 = crate::Reg<gich_eisr0::GICH_EISR0_SPEC>;
///GICH end of interrupt status register
pub mod gich_eisr0;
///GICH_ELSR0 (r) register accessor: an alias for `Reg<GICH_ELSR0_SPEC>`
pub type GICH_ELSR0 = crate::Reg<gich_elsr0::GICH_ELSR0_SPEC>;
///GICH empty list status register
pub mod gich_elsr0;
///GICH_APR0 (rw) register accessor: an alias for `Reg<GICH_APR0_SPEC>`
pub type GICH_APR0 = crate::Reg<gich_apr0::GICH_APR0_SPEC>;
///GICH active priority register
pub mod gich_apr0;
///GICH_LR0 (rw) register accessor: an alias for `Reg<GICH_LR0_SPEC>`
pub type GICH_LR0 = crate::Reg<gich_lr0::GICH_LR0_SPEC>;
///GICH list register 0
pub mod gich_lr0;
///GICH_LR1 (rw) register accessor: an alias for `Reg<GICH_LR1_SPEC>`
pub type GICH_LR1 = crate::Reg<gich_lr1::GICH_LR1_SPEC>;
///GICH list register 1
pub mod gich_lr1;
///GICH_LR2 (rw) register accessor: an alias for `Reg<GICH_LR2_SPEC>`
pub type GICH_LR2 = crate::Reg<gich_lr2::GICH_LR2_SPEC>;
///GICH list register 2
pub mod gich_lr2;
///GICH_LR3 (rw) register accessor: an alias for `Reg<GICH_LR3_SPEC>`
pub type GICH_LR3 = crate::Reg<gich_lr3::GICH_LR3_SPEC>;
///GICH list register 3
pub mod gich_lr3;
