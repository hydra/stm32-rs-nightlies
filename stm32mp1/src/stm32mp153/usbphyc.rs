///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - This register is used to control the PLL of the HS PHY.
    pub usbphyc_pll: USBPHYC_PLL,
    _reserved1: [u8; 0x04],
    ///0x08 - This register is used to control the switch between controllers for the HS PHY.
    pub usbphyc_misc: USBPHYC_MISC,
    _reserved2: [u8; 0x0100],
    ///0x10c - This register is used to control the tune interface of the HS PHY, port #x.
    pub usbphyc_tune1: USBPHYC_TUNE1,
    _reserved3: [u8; 0xfc],
    ///0x20c - This register is used to control the tune interface of the HS PHY, port #x.
    pub usbphyc_tune2: USBPHYC_TUNE2,
    _reserved4: [u8; 0x0dec],
    ///0xffc - This register defines the version of this IP.
    pub usbphyc_verr: USBPHYC_VERR,
}
///USBPHYC_PLL (rw) register accessor: an alias for `Reg<USBPHYC_PLL_SPEC>`
pub type USBPHYC_PLL = crate::Reg<usbphyc_pll::USBPHYC_PLL_SPEC>;
///This register is used to control the PLL of the HS PHY.
pub mod usbphyc_pll;
///USBPHYC_MISC (rw) register accessor: an alias for `Reg<USBPHYC_MISC_SPEC>`
pub type USBPHYC_MISC = crate::Reg<usbphyc_misc::USBPHYC_MISC_SPEC>;
///This register is used to control the switch between controllers for the HS PHY.
pub mod usbphyc_misc;
///USBPHYC_TUNE1 (rw) register accessor: an alias for `Reg<USBPHYC_TUNE1_SPEC>`
pub type USBPHYC_TUNE1 = crate::Reg<usbphyc_tune1::USBPHYC_TUNE1_SPEC>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod usbphyc_tune1;
///USBPHYC_TUNE2 (rw) register accessor: an alias for `Reg<USBPHYC_TUNE2_SPEC>`
pub type USBPHYC_TUNE2 = crate::Reg<usbphyc_tune2::USBPHYC_TUNE2_SPEC>;
///This register is used to control the tune interface of the HS PHY, port #x.
pub mod usbphyc_tune2;
///USBPHYC_VERR (r) register accessor: an alias for `Reg<USBPHYC_VERR_SPEC>`
pub type USBPHYC_VERR = crate::Reg<usbphyc_verr::USBPHYC_VERR_SPEC>;
///This register defines the version of this IP.
pub mod usbphyc_verr;
