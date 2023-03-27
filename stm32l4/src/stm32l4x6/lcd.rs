///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - frame control register
    pub fcr: FCR,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - clear register
    pub clr: CLR,
    _reserved4: [u8; 0x04],
    ///0x14..0x54 - display memory
    pub ram_com: [RAM_COM; 8],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///frame control register
pub mod fcr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`
pub type CLR = crate::Reg<clr::CLR_SPEC>;
///clear register
pub mod clr;
///RAM_COM (rw) register accessor: an alias for `Reg<RAM_COM_SPEC>`
pub type RAM_COM = crate::Reg<ram_com::RAM_COM_SPEC>;
///display memory
pub mod ram_com;
