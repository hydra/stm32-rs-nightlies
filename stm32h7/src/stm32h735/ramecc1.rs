///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RAMECC interrupt enable register
    pub ier: IER,
    _reserved1: [u8; 0x1c],
    ///0x20 - RAMECC monitor x configuration register
    pub m1cr: M1CR,
    ///0x24 - RAMECC monitor x status register
    pub m1sr: M1SR,
    ///0x28 - RAMECC monitor x failing address register
    pub m1far: M1FAR,
    ///0x2c - RAMECC monitor x failing data low register
    pub m1fdrl: M1FDRL,
    ///0x30 - RAMECC monitor x failing data high register
    pub m1fdrh: M1FDRH,
    ///0x34 - RAMECC monitor x failing ECC error code register
    pub m1fecr: M1FECR,
    _reserved7: [u8; 0x08],
    ///0x40 - RAMECC monitor x configuration register
    pub m2cr: M2CR,
    ///0x44 - RAMECC monitor x status register
    pub m2sr: M2SR,
    ///0x48 - RAMECC monitor x failing address register
    pub m2far: M2FAR,
    ///0x4c - RAMECC monitor x failing data low register
    pub m2fdrl: M2FDRL,
    ///0x50 - RAMECC monitor x failing data high register
    pub m2fdrh: M2FDRH,
    _reserved12: [u8; 0x04],
    ///0x58 - RAMECC monitor x failing ECC error code register
    pub m2fecr: M2FECR,
    _reserved13: [u8; 0x04],
    ///0x60 - RAMECC monitor x configuration register
    pub m3cr: M3CR,
    ///0x64 - RAMECC monitor x status register
    pub m3sr: M3SR,
    ///0x68 - RAMECC monitor x failing address register
    pub m3far: M3FAR,
    ///0x6c - RAMECC monitor x failing data low register
    pub m3fdrl: M3FDRL,
    ///0x70 - RAMECC monitor x failing data high register
    pub m3fdrh: M3FDRH,
    _reserved18: [u8; 0x08],
    ///0x7c - RAMECC monitor x failing ECC error code register
    pub m3fecr: M3FECR,
    ///0x80 - RAMECC monitor x configuration register
    pub m4cr: M4CR,
    ///0x84 - RAMECC monitor x status register
    pub m4sr: M4SR,
    ///0x88 - RAMECC monitor x failing address register
    pub m4far: M4FAR,
    ///0x8c - RAMECC monitor x failing data low register
    pub m4fdrl: M4FDRL,
    _reserved_23_m: [u8; 0x04],
    _reserved24: [u8; 0x0c],
    ///0xa0 - RAMECC monitor x configuration register
    pub m5cr: M5CR,
    ///0xa4 - RAMECC monitor x status register
    pub m5sr: M5SR,
    ///0xa8 - RAMECC monitor x failing address register
    pub m5far: M5FAR,
    ///0xac - RAMECC monitor x failing data low register
    pub m5fdrl: M5FDRL,
    ///0xb0 - RAMECC monitor x failing data high register
    pub m5fdrh: M5FDRH,
    ///0xb4 - RAMECC monitor x failing ECC error code register
    pub m5fecr: M5FECR,
}
impl RegisterBlock {
    ///0x90 - RAMECC monitor x failing ECC error code register
    #[inline(always)]
    pub const fn m4fecr(&self) -> &M4FECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
    ///0x90 - RAMECC monitor x failing data high register
    #[inline(always)]
    pub const fn m4fdrh(&self) -> &M4FDRH {
        unsafe { &*(self as *const Self).cast::<u8>().add(144usize).cast() }
    }
}
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///RAMECC interrupt enable register
pub mod ier;
///M1CR (rw) register accessor: an alias for `Reg<M1CR_SPEC>`
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
///RAMECC monitor x configuration register
pub mod m1cr;
///M1SR (rw) register accessor: an alias for `Reg<M1SR_SPEC>`
pub type M1SR = crate::Reg<m1sr::M1SR_SPEC>;
///RAMECC monitor x status register
pub mod m1sr;
///M1FAR (rw) register accessor: an alias for `Reg<M1FAR_SPEC>`
pub type M1FAR = crate::Reg<m1far::M1FAR_SPEC>;
///RAMECC monitor x failing address register
pub mod m1far;
///M1FDRL (rw) register accessor: an alias for `Reg<M1FDRL_SPEC>`
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRL_SPEC>;
///RAMECC monitor x failing data low register
pub mod m1fdrl;
///M1FDRH (rw) register accessor: an alias for `Reg<M1FDRH_SPEC>`
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRH_SPEC>;
///RAMECC monitor x failing data high register
pub mod m1fdrh;
///M1FECR (rw) register accessor: an alias for `Reg<M1FECR_SPEC>`
pub type M1FECR = crate::Reg<m1fecr::M1FECR_SPEC>;
///RAMECC monitor x failing ECC error code register
pub mod m1fecr;
///M2CR (rw) register accessor: an alias for `Reg<M2CR_SPEC>`
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
///RAMECC monitor x configuration register
pub mod m2cr;
///M2SR (rw) register accessor: an alias for `Reg<M2SR_SPEC>`
pub type M2SR = crate::Reg<m2sr::M2SR_SPEC>;
///RAMECC monitor x status register
pub mod m2sr;
///M2FAR (rw) register accessor: an alias for `Reg<M2FAR_SPEC>`
pub type M2FAR = crate::Reg<m2far::M2FAR_SPEC>;
///RAMECC monitor x failing address register
pub mod m2far;
///M2FDRL (rw) register accessor: an alias for `Reg<M2FDRL_SPEC>`
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRL_SPEC>;
///RAMECC monitor x failing data low register
pub mod m2fdrl;
///M2FDRH (r) register accessor: an alias for `Reg<M2FDRH_SPEC>`
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRH_SPEC>;
///RAMECC monitor x failing data high register
pub mod m2fdrh;
///M2FECR (r) register accessor: an alias for `Reg<M2FECR_SPEC>`
pub type M2FECR = crate::Reg<m2fecr::M2FECR_SPEC>;
///RAMECC monitor x failing ECC error code register
pub mod m2fecr;
///M3CR (r) register accessor: an alias for `Reg<M3CR_SPEC>`
pub type M3CR = crate::Reg<m3cr::M3CR_SPEC>;
///RAMECC monitor x configuration register
pub mod m3cr;
///M3SR (r) register accessor: an alias for `Reg<M3SR_SPEC>`
pub type M3SR = crate::Reg<m3sr::M3SR_SPEC>;
///RAMECC monitor x status register
pub mod m3sr;
///M3FAR (rw) register accessor: an alias for `Reg<M3FAR_SPEC>`
pub type M3FAR = crate::Reg<m3far::M3FAR_SPEC>;
///RAMECC monitor x failing address register
pub mod m3far;
///M3FDRL (r) register accessor: an alias for `Reg<M3FDRL_SPEC>`
pub type M3FDRL = crate::Reg<m3fdrl::M3FDRL_SPEC>;
///RAMECC monitor x failing data low register
pub mod m3fdrl;
///M3FDRH (r) register accessor: an alias for `Reg<M3FDRH_SPEC>`
pub type M3FDRH = crate::Reg<m3fdrh::M3FDRH_SPEC>;
///RAMECC monitor x failing data high register
pub mod m3fdrh;
///M3FECR (r) register accessor: an alias for `Reg<M3FECR_SPEC>`
pub type M3FECR = crate::Reg<m3fecr::M3FECR_SPEC>;
///RAMECC monitor x failing ECC error code register
pub mod m3fecr;
///M4CR (r) register accessor: an alias for `Reg<M4CR_SPEC>`
pub type M4CR = crate::Reg<m4cr::M4CR_SPEC>;
///RAMECC monitor x configuration register
pub mod m4cr;
///M4SR (r) register accessor: an alias for `Reg<M4SR_SPEC>`
pub type M4SR = crate::Reg<m4sr::M4SR_SPEC>;
///RAMECC monitor x status register
pub mod m4sr;
///M4FAR (r) register accessor: an alias for `Reg<M4FAR_SPEC>`
pub type M4FAR = crate::Reg<m4far::M4FAR_SPEC>;
///RAMECC monitor x failing address register
pub mod m4far;
///M4FDRL (rw) register accessor: an alias for `Reg<M4FDRL_SPEC>`
pub type M4FDRL = crate::Reg<m4fdrl::M4FDRL_SPEC>;
///RAMECC monitor x failing data low register
pub mod m4fdrl;
///M4FDRH (r) register accessor: an alias for `Reg<M4FDRH_SPEC>`
pub type M4FDRH = crate::Reg<m4fdrh::M4FDRH_SPEC>;
///RAMECC monitor x failing data high register
pub mod m4fdrh;
///M4FECR (r) register accessor: an alias for `Reg<M4FECR_SPEC>`
pub type M4FECR = crate::Reg<m4fecr::M4FECR_SPEC>;
///RAMECC monitor x failing ECC error code register
pub mod m4fecr;
///M5CR (r) register accessor: an alias for `Reg<M5CR_SPEC>`
pub type M5CR = crate::Reg<m5cr::M5CR_SPEC>;
///RAMECC monitor x configuration register
pub mod m5cr;
///M5SR (rw) register accessor: an alias for `Reg<M5SR_SPEC>`
pub type M5SR = crate::Reg<m5sr::M5SR_SPEC>;
///RAMECC monitor x status register
pub mod m5sr;
///M5FAR (rw) register accessor: an alias for `Reg<M5FAR_SPEC>`
pub type M5FAR = crate::Reg<m5far::M5FAR_SPEC>;
///RAMECC monitor x failing address register
pub mod m5far;
///M5FDRL (r) register accessor: an alias for `Reg<M5FDRL_SPEC>`
pub type M5FDRL = crate::Reg<m5fdrl::M5FDRL_SPEC>;
///RAMECC monitor x failing data low register
pub mod m5fdrl;
///M5FDRH (r) register accessor: an alias for `Reg<M5FDRH_SPEC>`
pub type M5FDRH = crate::Reg<m5fdrh::M5FDRH_SPEC>;
///RAMECC monitor x failing data high register
pub mod m5fdrh;
///M5FECR (r) register accessor: an alias for `Reg<M5FECR_SPEC>`
pub type M5FECR = crate::Reg<m5fecr::M5FECR_SPEC>;
///RAMECC monitor x failing ECC error code register
pub mod m5fecr;
