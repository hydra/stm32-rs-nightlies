///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet MAC configuration register
    pub maccr: MACCR,
    ///0x04 - Ethernet MAC frame filter register
    pub macffr: MACFFR,
    ///0x08 - Ethernet MAC hash table high register
    pub machthr: MACHTHR,
    ///0x0c - Ethernet MAC hash table low register
    pub machtlr: MACHTLR,
    ///0x10 - Ethernet MAC MII address register
    pub macmiiar: MACMIIAR,
    ///0x14 - Ethernet MAC MII data register
    pub macmiidr: MACMIIDR,
    ///0x18 - Ethernet MAC flow control register
    pub macfcr: MACFCR,
    ///0x1c - Ethernet MAC VLAN tag register
    pub macvlantr: MACVLANTR,
    _reserved8: [u8; 0x0c],
    ///0x2c - Ethernet MAC PMT control and status register
    pub macpmtcsr: MACPMTCSR,
    _reserved9: [u8; 0x04],
    ///0x34 - Ethernet MAC debug register
    pub macdbgr: MACDBGR,
    ///0x38 - Ethernet MAC interrupt status register
    pub macsr: MACSR,
    ///0x3c - Ethernet MAC interrupt mask register
    pub macimr: MACIMR,
    ///0x40 - Ethernet MAC address 0 high register
    pub maca0hr: MACA0HR,
    ///0x44 - Ethernet MAC address 0 low register
    pub maca0lr: MACA0LR,
    ///0x48 - Ethernet MAC address 1 high register
    pub maca1hr: MACA1HR,
    ///0x4c - Ethernet MAC address1 low register
    pub maca1lr: MACA1LR,
    ///0x50 - Ethernet MAC address 2 high register
    pub maca2hr: MACA2HR,
    ///0x54 - Ethernet MAC address 2 low register
    pub maca2lr: MACA2LR,
    ///0x58 - Ethernet MAC address 3 high register
    pub maca3hr: MACA3HR,
    ///0x5c - Ethernet MAC address 3 low register
    pub maca3lr: MACA3LR,
    ///0x60 - Ethernet MAC remote wakeup frame filter register
    pub macrwuffer: MACRWUFFER,
}
///MACCR (rw) register accessor: an alias for `Reg<MACCR_SPEC>`
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
///Ethernet MAC configuration register
pub mod maccr;
///MACFFR (rw) register accessor: an alias for `Reg<MACFFR_SPEC>`
pub type MACFFR = crate::Reg<macffr::MACFFR_SPEC>;
///Ethernet MAC frame filter register
pub mod macffr;
///MACHTHR (rw) register accessor: an alias for `Reg<MACHTHR_SPEC>`
pub type MACHTHR = crate::Reg<machthr::MACHTHR_SPEC>;
///Ethernet MAC hash table high register
pub mod machthr;
///MACHTLR (rw) register accessor: an alias for `Reg<MACHTLR_SPEC>`
pub type MACHTLR = crate::Reg<machtlr::MACHTLR_SPEC>;
///Ethernet MAC hash table low register
pub mod machtlr;
///MACMIIAR (rw) register accessor: an alias for `Reg<MACMIIAR_SPEC>`
pub type MACMIIAR = crate::Reg<macmiiar::MACMIIAR_SPEC>;
///Ethernet MAC MII address register
pub mod macmiiar;
///MACMIIDR (rw) register accessor: an alias for `Reg<MACMIIDR_SPEC>`
pub type MACMIIDR = crate::Reg<macmiidr::MACMIIDR_SPEC>;
///Ethernet MAC MII data register
pub mod macmiidr;
///MACFCR (rw) register accessor: an alias for `Reg<MACFCR_SPEC>`
pub type MACFCR = crate::Reg<macfcr::MACFCR_SPEC>;
///Ethernet MAC flow control register
pub mod macfcr;
///MACVLANTR (rw) register accessor: an alias for `Reg<MACVLANTR_SPEC>`
pub type MACVLANTR = crate::Reg<macvlantr::MACVLANTR_SPEC>;
///Ethernet MAC VLAN tag register
pub mod macvlantr;
///MACPMTCSR (rw) register accessor: an alias for `Reg<MACPMTCSR_SPEC>`
pub type MACPMTCSR = crate::Reg<macpmtcsr::MACPMTCSR_SPEC>;
///Ethernet MAC PMT control and status register
pub mod macpmtcsr;
///MACDBGR (r) register accessor: an alias for `Reg<MACDBGR_SPEC>`
pub type MACDBGR = crate::Reg<macdbgr::MACDBGR_SPEC>;
///Ethernet MAC debug register
pub mod macdbgr;
///MACSR (rw) register accessor: an alias for `Reg<MACSR_SPEC>`
pub type MACSR = crate::Reg<macsr::MACSR_SPEC>;
///Ethernet MAC interrupt status register
pub mod macsr;
///MACIMR (rw) register accessor: an alias for `Reg<MACIMR_SPEC>`
pub type MACIMR = crate::Reg<macimr::MACIMR_SPEC>;
///Ethernet MAC interrupt mask register
pub mod macimr;
///MACA0HR (rw) register accessor: an alias for `Reg<MACA0HR_SPEC>`
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
///Ethernet MAC address 0 high register
pub mod maca0hr;
///MACA0LR (rw) register accessor: an alias for `Reg<MACA0LR_SPEC>`
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
///Ethernet MAC address 0 low register
pub mod maca0lr;
///MACA1HR (rw) register accessor: an alias for `Reg<MACA1HR_SPEC>`
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
///Ethernet MAC address 1 high register
pub mod maca1hr;
///MACA1LR (rw) register accessor: an alias for `Reg<MACA1LR_SPEC>`
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
///Ethernet MAC address1 low register
pub mod maca1lr;
///MACA2HR (rw) register accessor: an alias for `Reg<MACA2HR_SPEC>`
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
///Ethernet MAC address 2 high register
pub mod maca2hr;
///MACA2LR (rw) register accessor: an alias for `Reg<MACA2LR_SPEC>`
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
///Ethernet MAC address 2 low register
pub mod maca2lr;
///MACA3HR (rw) register accessor: an alias for `Reg<MACA3HR_SPEC>`
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
///Ethernet MAC address 3 high register
pub mod maca3hr;
///MACA3LR (rw) register accessor: an alias for `Reg<MACA3LR_SPEC>`
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
///Ethernet MAC address 3 low register
pub mod maca3lr;
///MACRWUFFER (rw) register accessor: an alias for `Reg<MACRWUFFER_SPEC>`
pub type MACRWUFFER = crate::Reg<macrwuffer::MACRWUFFER_SPEC>;
///Ethernet MAC remote wakeup frame filter register
pub mod macrwuffer;
