///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RAMECC interrupt enable register
    pub ier: IER,
    _reserved1: [u8; 0x1c],
    ///0x20 - RAMECC monitor 1 configuration register
    pub m1cr: M1CR,
    ///0x24 - RAMECC monitor 1 status register
    pub m1sr: M1SR,
    ///0x28 - RAMECC monitor 1 failing address register
    pub m1far: M1FAR,
    ///0x2c - RAMECC monitor 1 failing data low register
    pub m1fdrl: M1FDRL,
    ///0x30 - RAMECC monitor 1 failing data high register
    pub m1fdrh: M1FDRH,
    ///0x34 - RAMECC monitor 1 failing error code register
    pub m1fecr: M1FECR,
    _reserved7: [u8; 0x08],
    ///0x40 - RAMECC monitor 2 configuration register
    pub m2cr: M2CR,
    ///0x44 - RAMECC monitor 2 status register
    pub m2sr: M2SR,
    ///0x48 - RAMECC monitor 2 failing address register
    pub m2far: M2FAR,
    ///0x4c - RAMECC monitor 2 failing data low register
    pub m2fdrl: M2FDRL,
    ///0x50 - RAMECC monitor 2 failing data high register
    pub m2fdrh: M2FDRH,
    ///0x54 - RAMECC monitor 2 failing error code register
    pub m2fecr: M2FECR,
}
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///RAMECC interrupt enable register
pub mod ier;
///M1CR (rw) register accessor: an alias for `Reg<M1CR_SPEC>`
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
///RAMECC monitor 1 configuration register
pub mod m1cr;
///M1SR (rw) register accessor: an alias for `Reg<M1SR_SPEC>`
pub type M1SR = crate::Reg<m1sr::M1SR_SPEC>;
///RAMECC monitor 1 status register
pub mod m1sr;
///M1FAR (rw) register accessor: an alias for `Reg<M1FAR_SPEC>`
pub type M1FAR = crate::Reg<m1far::M1FAR_SPEC>;
///RAMECC monitor 1 failing address register
pub mod m1far;
///M1FDRL (rw) register accessor: an alias for `Reg<M1FDRL_SPEC>`
pub type M1FDRL = crate::Reg<m1fdrl::M1FDRL_SPEC>;
///RAMECC monitor 1 failing data low register
pub mod m1fdrl;
///M1FDRH (rw) register accessor: an alias for `Reg<M1FDRH_SPEC>`
pub type M1FDRH = crate::Reg<m1fdrh::M1FDRH_SPEC>;
///RAMECC monitor 1 failing data high register
pub mod m1fdrh;
///M1FECR (rw) register accessor: an alias for `Reg<M1FECR_SPEC>`
pub type M1FECR = crate::Reg<m1fecr::M1FECR_SPEC>;
///RAMECC monitor 1 failing error code register
pub mod m1fecr;
///M2CR (rw) register accessor: an alias for `Reg<M2CR_SPEC>`
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
///RAMECC monitor 2 configuration register
pub mod m2cr;
///M2SR (rw) register accessor: an alias for `Reg<M2SR_SPEC>`
pub type M2SR = crate::Reg<m2sr::M2SR_SPEC>;
///RAMECC monitor 2 status register
pub mod m2sr;
///M2FAR (rw) register accessor: an alias for `Reg<M2FAR_SPEC>`
pub type M2FAR = crate::Reg<m2far::M2FAR_SPEC>;
///RAMECC monitor 2 failing address register
pub mod m2far;
///M2FDRL (rw) register accessor: an alias for `Reg<M2FDRL_SPEC>`
pub type M2FDRL = crate::Reg<m2fdrl::M2FDRL_SPEC>;
///RAMECC monitor 2 failing data low register
pub mod m2fdrl;
///M2FDRH (rw) register accessor: an alias for `Reg<M2FDRH_SPEC>`
pub type M2FDRH = crate::Reg<m2fdrh::M2FDRH_SPEC>;
///RAMECC monitor 2 failing data high register
pub mod m2fdrh;
///M2FECR (rw) register accessor: an alias for `Reg<M2FECR_SPEC>`
pub type M2FECR = crate::Reg<m2fecr::M2FECR_SPEC>;
///RAMECC monitor 2 failing error code register
pub mod m2fecr;
