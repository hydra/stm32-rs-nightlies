///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)
    pub gotgctl: GOTGCTL,
    ///0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)
    pub gotgint: GOTGINT,
    ///0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
    pub gahbcfg: GAHBCFG,
    ///0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)
    pub gusbcfg: GUSBCFG,
    ///0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)
    pub grstctl: GRSTCTL,
    ///0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)
    pub gintsts: GINTSTS,
    ///0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)
    pub gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    ///0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
    pub grxfsiz: GRXFSIZ,
    _reserved_10_dieptxf0: [u8; 0x04],
    ///0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
    pub gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x08],
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    pub gccfg: GCCFG,
    ///0x3c - core ID register
    pub cid: CID,
    _reserved14: [u8; 0xc0],
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    pub hptxfsiz: HPTXFSIZ,
    ///0x104..0x118 - OTF_FS device IN endpoint transmit FIFO size register
    pub dieptxf: [DIEPTXF; 5],
}
impl RegisterBlock {
    ///0x1c - OTG status debug read (host mode)
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x20 - OTG status read and pop (host mode)
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x20 - OTG status read and pop (device mode)
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x104 - OTF_FS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf1(&self) -> &DIEPTXF {
        &self.dieptxf[0]
    }
    ///0x108 - OTF_FS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf2(&self) -> &DIEPTXF {
        &self.dieptxf[1]
    }
    ///0x10c - OTF_FS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf3(&self) -> &DIEPTXF {
        &self.dieptxf[2]
    }
    ///0x110 - OTF_FS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf4(&self) -> &DIEPTXF {
        &self.dieptxf[3]
    }
    ///0x114 - OTF_FS device IN endpoint transmit FIFO size register
    #[inline(always)]
    pub fn dieptxf5(&self) -> &DIEPTXF {
        &self.dieptxf[4]
    }
}
///GOTGCTL (rw) register accessor: an alias for `Reg<GOTGCTL_SPEC>`
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
///OTG_FS control and status register (OTG_FS_GOTGCTL)
pub mod gotgctl;
///GOTGINT (rw) register accessor: an alias for `Reg<GOTGINT_SPEC>`
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
///OTG_FS interrupt register (OTG_FS_GOTGINT)
pub mod gotgint;
///GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
///OTG_FS AHB configuration register (OTG_FS_GAHBCFG)
pub mod gahbcfg;
///GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
///OTG_FS USB configuration register (OTG_FS_GUSBCFG)
pub mod gusbcfg;
///GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
///OTG_FS reset register (OTG_FS_GRSTCTL)
pub mod grstctl;
///GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
///OTG_FS core interrupt register (OTG_FS_GINTSTS)
pub mod gintsts;
///GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
///OTG_FS interrupt mask register (OTG_FS_GINTMSK)
pub mod gintmsk;
///GRXSTSR_Device (r) register accessor: an alias for `Reg<GRXSTSR_DEVICE_SPEC>`
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICE_SPEC>;
///OTG_FS Receive status debug read(Device mode)
pub mod grxstsr_device;
///GRXSTSR_Host (r) register accessor: an alias for `Reg<GRXSTSR_HOST_SPEC>`
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOST_SPEC>;
///OTG status debug read (host mode)
pub mod grxstsr_host;
///GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod grxfsiz;
///DIEPTXF0 (rw) register accessor: an alias for `Reg<DIEPTXF0_SPEC>`
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0_SPEC>;
///OTG_FS non-periodic transmit FIFO size register (Device mode)
pub mod dieptxf0;
///HNPTXFSIZ (rw) register accessor: an alias for `Reg<HNPTXFSIZ_SPEC>`
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZ_SPEC>;
///OTG_FS non-periodic transmit FIFO size register (Host mode)
pub mod hnptxfsiz;
///GNPTXSTS (r) register accessor: an alias for `Reg<GNPTXSTS_SPEC>`
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod gnptxsts;
///GCCFG (rw) register accessor: an alias for `Reg<GCCFG_SPEC>`
pub type GCCFG = crate::Reg<gccfg::GCCFG_SPEC>;
///OTG_FS general core configuration register (OTG_FS_GCCFG)
pub mod gccfg;
///CID (rw) register accessor: an alias for `Reg<CID_SPEC>`
pub type CID = crate::Reg<cid::CID_SPEC>;
///core ID register
pub mod cid;
///HPTXFSIZ (rw) register accessor: an alias for `Reg<HPTXFSIZ_SPEC>`
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
///OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
pub mod hptxfsiz;
///DIEPTXF (rw) register accessor: an alias for `Reg<DIEPTXF_SPEC>`
pub type DIEPTXF = crate::Reg<dieptxf::DIEPTXF_SPEC>;
///OTF_FS device IN endpoint transmit FIFO size register
pub mod dieptxf;
///GRXSTSP_Device (r) register accessor: an alias for `Reg<GRXSTSP_DEVICE_SPEC>`
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICE_SPEC>;
///OTG status read and pop (device mode)
pub mod grxstsp_device;
///GRXSTSP_Host (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
///OTG status read and pop (host mode)
pub mod grxstsp_host;
