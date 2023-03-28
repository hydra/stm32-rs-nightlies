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
    pub hnptxsts: HNPTXSTS,
    ///0x30 - OTG I2C access register
    pub gi2cctl: GI2CCTL,
    _reserved13: [u8; 0x04],
    ///0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)
    pub gccfg: GCCFG,
    ///0x3c - core ID register
    pub cid: CID,
    _reserved15: [u8; 0x14],
    ///0x54 - OTG core LPM configuration register
    pub glpmcfg: GLPMCFG,
    ///0x58 - OTG power down register
    pub gpwrdn: GPWRDN,
    _reserved17: [u8; 0x04],
    ///0x60 - OTG ADP timer, control and status register
    pub gadpctl: GADPCTL,
    _reserved18: [u8; 0x9c],
    ///0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)
    pub hptxfsiz: HPTXFSIZ,
    ///0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)
    pub dieptxf1: DIEPTXF1,
    ///0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
    pub dieptxf2: DIEPTXF2,
    ///0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
    pub dieptxf3: DIEPTXF3,
    ///0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
    pub dieptxf4: DIEPTXF4,
    ///0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
    pub dieptxf5: DIEPTXF5,
}
impl RegisterBlock {
    ///0x1c - OTG_FS Receive status debug read(Host mode)
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x1c - OTG_FS Receive status debug read(Device mode)
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x20 - OTG status read and pop register (Host mode)
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x20 - OTG status read and pop register (Device mode)
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x28 - OTG_FS Host non-periodic transmit FIFO size register
    #[inline(always)]
    pub const fn hnptxfsiz_host(&self) -> &HNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x28 - OTG_FS Endpoint 0 Transmit FIFO size
    #[inline(always)]
    pub const fn dieptxf0(&self) -> &DIEPTXF0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
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
///OTG_FS Receive status debug read(Host mode)
pub mod grxstsr_host;
///GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
///OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)
pub mod grxfsiz;
///DIEPTXF0 (rw) register accessor: an alias for `Reg<DIEPTXF0_SPEC>`
pub type DIEPTXF0 = crate::Reg<dieptxf0::DIEPTXF0_SPEC>;
///OTG_FS Endpoint 0 Transmit FIFO size
pub mod dieptxf0;
///HNPTXFSIZ_Host (rw) register accessor: an alias for `Reg<HNPTXFSIZ_HOST_SPEC>`
pub type HNPTXFSIZ_HOST = crate::Reg<hnptxfsiz_host::HNPTXFSIZ_HOST_SPEC>;
///OTG_FS Host non-periodic transmit FIFO size register
pub mod hnptxfsiz_host;
///HNPTXSTS (r) register accessor: an alias for `Reg<HNPTXSTS_SPEC>`
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTS_SPEC>;
///OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)
pub mod hnptxsts;
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
///DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)
pub mod dieptxf1;
///DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)
pub mod dieptxf2;
///DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)
pub mod dieptxf3;
///GRXSTSP_Device (r) register accessor: an alias for `Reg<GRXSTSP_DEVICE_SPEC>`
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICE_SPEC>;
///OTG status read and pop register (Device mode)
pub mod grxstsp_device;
///GRXSTSP_Host (r) register accessor: an alias for `Reg<GRXSTSP_HOST_SPEC>`
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOST_SPEC>;
///OTG status read and pop register (Host mode)
pub mod grxstsp_host;
///GI2CCTL (rw) register accessor: an alias for `Reg<GI2CCTL_SPEC>`
pub type GI2CCTL = crate::Reg<gi2cctl::GI2CCTL_SPEC>;
///OTG I2C access register
pub mod gi2cctl;
///GPWRDN (rw) register accessor: an alias for `Reg<GPWRDN_SPEC>`
pub type GPWRDN = crate::Reg<gpwrdn::GPWRDN_SPEC>;
///OTG power down register
pub mod gpwrdn;
///GADPCTL (rw) register accessor: an alias for `Reg<GADPCTL_SPEC>`
pub type GADPCTL = crate::Reg<gadpctl::GADPCTL_SPEC>;
///OTG ADP timer, control and status register
pub mod gadpctl;
///DIEPTXF4 (rw) register accessor: an alias for `Reg<DIEPTXF4_SPEC>`
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)
pub mod dieptxf4;
///DIEPTXF5 (rw) register accessor: an alias for `Reg<DIEPTXF5_SPEC>`
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
///OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)
pub mod dieptxf5;
///GLPMCFG (rw) register accessor: an alias for `Reg<GLPMCFG_SPEC>`
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFG_SPEC>;
///OTG core LPM configuration register
pub mod glpmcfg;
