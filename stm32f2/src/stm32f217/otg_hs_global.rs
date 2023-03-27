///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_HS control and status register
    pub gotgctl: GOTGCTL,
    ///0x04 - OTG_HS interrupt register
    pub gotgint: GOTGINT,
    ///0x08 - OTG_HS AHB configuration register
    pub gahbcfg: GAHBCFG,
    ///0x0c - OTG_HS USB configuration register
    pub gusbcfg: GUSBCFG,
    ///0x10 - OTG_HS reset register
    pub grstctl: GRSTCTL,
    ///0x14 - OTG_HS core interrupt register
    pub gintsts: GINTSTS,
    ///0x18 - OTG_HS interrupt mask register
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    ///0x24 - OTG_HS Receive FIFO size register
    pub grxfsiz: GRXFSIZ,
    _reserved_10_dieptxf0: [u8; 0x04],
    ///0x2c - OTG_HS nonperiodic transmit FIFO/queue status register
    pub hnptxsts: HNPTXSTS,
    _reserved12: [u8; 0x08],
    ///0x38 - OTG_HS general core configuration register
    pub gccfg: GCCFG,
    ///0x3c - OTG_HS core ID register
    pub cid: CID,
    _reserved14: [u8; 0xc0],
    ///0x100 - OTG_HS Host periodic transmit FIFO size register
    pub hptxfsiz: HPTXFSIZ,
    ///0x104..0x118 - OTG_HS device IN endpoint transmit FIFO size register
    pub dieptxf: [DIEPTXF; 5],
}
impl RegisterBlock {
    ///0x1c - OTG_HS Receive status debug read register (peripheral mode mode)
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - OTG_HS Receive status debug read register (host mode)
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (peripheral mode)
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x20 - OTG_HS status read and pop register (host mode)
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x28 - Endpoint 0 transmit FIFO size (peripheral mode)
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x104 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf1(&self) -> &DIEPTXF {
        &self.dieptxf[0]
    }
    ///0x108 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf2(&self) -> &DIEPTXF {
        &self.dieptxf[1]
    }
    ///0x10c - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf3(&self) -> &DIEPTXF {
        &self.dieptxf[2]
    }
    ///0x110 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf4(&self) -> &DIEPTXF {
        &self.dieptxf[3]
    }
    ///0x114 - OTG_HS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf5(&self) -> &DIEPTXF {
        &self.dieptxf[4]
    }
}
///GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
///OTG_HS control and status register
pub mod gotgctl;
///GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
///OTG_HS interrupt register
pub mod gotgint;
///GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
///OTG_HS AHB configuration register
pub mod gahbcfg;
///GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
///OTG_HS USB configuration register
pub mod gusbcfg;
///GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
///OTG_HS reset register
pub mod grstctl;
///GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
///OTG_HS core interrupt register
pub mod gintsts;
///GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
///OTG_HS interrupt mask register
pub mod gintmsk;
///GRXSTSR_Host (r) register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
///OTG_HS Receive status debug read register (host mode)
pub mod grxstsr_host;
///GRXSTSP_Host (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
///OTG_HS status read and pop register (host mode)
pub mod grxstsp_host;
///GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
///OTG_HS Receive FIFO size register
pub mod grxfsiz;
///HNPTXFSIZ (rw) register accessor: an alias for `Reg<HNPTXFSIZ_SPEC>`
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZ_SPEC>;
///OTG_HS nonperiodic transmit FIFO size register (host mode)
pub mod hnptxfsiz;
///DIEPTXF0 (rw) register accessor: an alias for `Reg<DIEPTXF0_SPEC>`
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0_SPEC>;
///Endpoint 0 transmit FIFO size (peripheral mode)
pub mod dieptxf0;
///HNPTXSTS (r) register accessor: an alias for `Reg<HNPTXSTS_SPEC>`
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTS_SPEC>;
///OTG_HS nonperiodic transmit FIFO/queue status register
pub mod hnptxsts;
///GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
///OTG_HS general core configuration register
pub mod gccfg;
///CID (rw) register accessor: an alias for `Reg<CID_SPEC>`
pub type CID = crate::Reg<cid::CID_SPEC>;
///OTG_HS core ID register
pub mod cid;
///HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
///OTG_HS Host periodic transmit FIFO size register
pub mod hptxfsiz;
///DIEPTXF (rw) register accessor: an alias for `Reg<DIEPTXF_SPEC>`
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXF_SPEC>;
///OTG_HS device IN endpoint transmit FIFO size register
pub mod dieptxf;
///GRXSTSR_Device (r) register accessor: an alias for `Reg<GRXSTSR_DEVICE_SPEC>`
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICE_SPEC>;
///OTG_HS Receive status debug read register (peripheral mode mode)
pub mod grxstsr_device;
///GRXSTSP_Device (r) register accessor: an alias for `Reg<GRXSTSP_DEVICE_SPEC>`
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICE_SPEC>;
///OTG_HS status read and pop register (peripheral mode)
pub mod grxstsp_device;
