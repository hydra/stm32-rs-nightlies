///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DBGMCU identity code register
    pub idcode: IDCODE,
    ///0x04 - DBGMCU configuration register
    pub cr: CR,
    ///0x08 - DBGMCU APB1L peripheral freeze register
    pub apb1lfzr: APB1LFZR,
    ///0x0c - DBGMCU APB1H peripheral freeze register
    pub apb1hfzr: APB1HFZR,
    ///0x10 - DBGMCU APB2 peripheral freeze register
    pub apb2fzr: APB2FZR,
    ///0x14 - DBGMCU APB3 peripheral freeze register
    pub apb3fzr: APB3FZR,
    _reserved6: [u8; 0x08],
    ///0x20 - DBGMCU AHB1 peripheral freeze register
    pub ahb1fzr: AHB1FZR,
    _reserved7: [u8; 0xd8],
    ///0xfc - DBGMCU status register
    pub sr: SR,
    ///0x100 - DBGMCU debug authentication mailbox host register
    pub dbg_auth_host: DBG_AUTH_HOST,
    ///0x104 - DBGMCU debug authentication mailbox device register
    pub dbg_auth_device: DBG_AUTH_DEVICE,
    ///0x108 - DBGMCU debug authentication mailbox acknowledge register
    pub dbg_auth_ack: DBG_AUTH_ACK,
    _reserved11: [u8; 0x0ec4],
    ///0xfd0 - DBGMCU CoreSight peripheral identity register 4
    pub pidr4: PIDR4,
    _reserved12: [u8; 0x0c],
    ///0xfe0 - DBGMCU CoreSight peripheral identity register 0
    pub pidr0: PIDR0,
    ///0xfe4 - DBGMCU CoreSight peripheral identity register 1
    pub pidr1: PIDR1,
    ///0xfe8 - DBGMCU CoreSight peripheral identity register 2
    pub pidr2: PIDR2,
    ///0xfec - DBGMCU CoreSight peripheral identity register 3
    pub pidr3: PIDR3,
    ///0xff0 - DBGMCU CoreSight component identity register 0
    pub cidr0: CIDR0,
    ///0xff4 - DBGMCU CoreSight component identity register 1
    pub cidr1: CIDR1,
    ///0xff8 - DBGMCU CoreSight component identity register 2
    pub cidr2: CIDR2,
    ///0xffc - DBGMCU CoreSight component identity register 3
    pub cidr3: CIDR3,
}
///IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
///DBGMCU identity code register
pub mod idcode;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DBGMCU configuration register
pub mod cr;
///APB1LFZR (rw) register accessor: an alias for `Reg<APB1LFZR_SPEC>`
pub type APB1LFZR = crate::Reg<apb1lfzr::APB1LFZR_SPEC>;
///DBGMCU APB1L peripheral freeze register
pub mod apb1lfzr;
///APB1HFZR (rw) register accessor: an alias for `Reg<APB1HFZR_SPEC>`
pub type APB1HFZR = crate::Reg<apb1hfzr::APB1HFZR_SPEC>;
///DBGMCU APB1H peripheral freeze register
pub mod apb1hfzr;
///APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
///DBGMCU APB2 peripheral freeze register
pub mod apb2fzr;
///APB3FZR (rw) register accessor: an alias for `Reg<APB3FZR_SPEC>`
pub type APB3FZR = crate::Reg<apb3fzr::APB3FZR_SPEC>;
///DBGMCU APB3 peripheral freeze register
pub mod apb3fzr;
///AHB1FZR (rw) register accessor: an alias for `Reg<AHB1FZR_SPEC>`
pub type AHB1FZR = crate::Reg<ahb1fzr::AHB1FZR_SPEC>;
///DBGMCU AHB1 peripheral freeze register
pub mod ahb1fzr;
///SR (w) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///DBGMCU status register
pub mod sr;
///DBG_AUTH_HOST (rw) register accessor: an alias for `Reg<DBG_AUTH_HOST_SPEC>`
pub type DBG_AUTH_HOST = crate::Reg<dbg_auth_host::DBG_AUTH_HOST_SPEC>;
///DBGMCU debug authentication mailbox host register
pub mod dbg_auth_host;
///DBG_AUTH_DEVICE (r) register accessor: an alias for `Reg<DBG_AUTH_DEVICE_SPEC>`
pub type DBG_AUTH_DEVICE = crate::Reg<dbg_auth_device::DBG_AUTH_DEVICE_SPEC>;
///DBGMCU debug authentication mailbox device register
pub mod dbg_auth_device;
///DBG_AUTH_ACK (rw) register accessor: an alias for `Reg<DBG_AUTH_ACK_SPEC>`
pub type DBG_AUTH_ACK = crate::Reg<dbg_auth_ack::DBG_AUTH_ACK_SPEC>;
///DBGMCU debug authentication mailbox acknowledge register
pub mod dbg_auth_ack;
///PIDR4 (r) register accessor: an alias for `Reg<PIDR4_SPEC>`
pub type PIDR4 = crate::Reg<pidr4::PIDR4_SPEC>;
///DBGMCU CoreSight peripheral identity register 4
pub mod pidr4;
///PIDR0 (r) register accessor: an alias for `Reg<PIDR0_SPEC>`
pub type PIDR0 = crate::Reg<pidr0::PIDR0_SPEC>;
///DBGMCU CoreSight peripheral identity register 0
pub mod pidr0;
///PIDR1 (r) register accessor: an alias for `Reg<PIDR1_SPEC>`
pub type PIDR1 = crate::Reg<pidr1::PIDR1_SPEC>;
///DBGMCU CoreSight peripheral identity register 1
pub mod pidr1;
///PIDR2 (r) register accessor: an alias for `Reg<PIDR2_SPEC>`
pub type PIDR2 = crate::Reg<pidr2::PIDR2_SPEC>;
///DBGMCU CoreSight peripheral identity register 2
pub mod pidr2;
///PIDR3 (r) register accessor: an alias for `Reg<PIDR3_SPEC>`
pub type PIDR3 = crate::Reg<pidr3::PIDR3_SPEC>;
///DBGMCU CoreSight peripheral identity register 3
pub mod pidr3;
///CIDR0 (r) register accessor: an alias for `Reg<CIDR0_SPEC>`
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
///DBGMCU CoreSight component identity register 0
pub mod cidr0;
///CIDR1 (r) register accessor: an alias for `Reg<CIDR1_SPEC>`
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
///DBGMCU CoreSight component identity register 1
pub mod cidr1;
///CIDR2 (r) register accessor: an alias for `Reg<CIDR2_SPEC>`
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
///DBGMCU CoreSight component identity register 2
pub mod cidr2;
///CIDR3 (r) register accessor: an alias for `Reg<CIDR3_SPEC>`
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
///DBGMCU CoreSight component identity register 3
pub mod cidr3;
