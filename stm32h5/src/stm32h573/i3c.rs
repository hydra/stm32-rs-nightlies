///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    ///0x04 - I3C configuration register
    pub cfgr: CFGR,
    _reserved2: [u8; 0x08],
    ///0x10 - I3C receive data byte register
    pub rdr: RDR,
    ///0x14 - I3C receive data word register
    pub rdwr: RDWR,
    ///0x18 - I3C transmit data byte register
    pub tdr: TDR,
    ///0x1c - I3C transmit data word register
    pub tdwr: TDWR,
    ///0x20 - I3C IBI payload data register
    pub ibidr: IBIDR,
    ///0x24 - I3C target transmit configuration register
    pub tgttdr: TGTTDR,
    _reserved8: [u8; 0x08],
    ///0x30 - I3C status register
    pub sr: SR,
    ///0x34 - I3C status error register
    pub ser: SER,
    _reserved10: [u8; 0x08],
    ///0x40 - I3C received message register
    pub rmr: RMR,
    _reserved11: [u8; 0x0c],
    ///0x50 - I3C event register
    pub evr: EVR,
    ///0x54 - I3C interrupt enable register
    pub ier: IER,
    ///0x58 - I3C clear event register
    pub cevr: CEVR,
    _reserved14: [u8; 0x04],
    ///0x60 - I3C own device characteristics register
    pub devr0: DEVR0,
    ///0x64 - I3C device 1 characteristics register
    pub devr1: DEVR1,
    ///0x68 - I3C device 2 characteristics register
    pub devr2: DEVR2,
    ///0x6c - I3C device 3 characteristics register
    pub devr3: DEVR3,
    ///0x70 - I3C device 4 characteristics register
    pub devr4: DEVR4,
    _reserved19: [u8; 0x1c],
    ///0x90 - I3C maximum read length register
    pub maxrlr: MAXRLR,
    ///0x94 - I3C maximum write length register
    pub maxwlr: MAXWLR,
    _reserved21: [u8; 0x08],
    ///0xa0 - I3C timing register 0
    pub timingr0: TIMINGR0,
    ///0xa4 - I3C timing register 1
    pub timingr1: TIMINGR1,
    ///0xa8 - I3C timing register 2
    pub timingr2: TIMINGR2,
    _reserved24: [u8; 0x14],
    ///0xc0 - I3C bus characteristics register
    pub bcr: BCR,
    ///0xc4 - I3C device characteristics register
    pub dcr: DCR,
    ///0xc8 - I3C get capability register
    pub getcapr: GETCAPR,
    ///0xcc - I3C controller-role capability register
    pub crcapr: CRCAPR,
    ///0xd0 - I3C get capability register
    pub getmxdsr: GETMXDSR,
    ///0xd4 - I3C extended provisioned ID register
    pub epidr: EPIDR,
}
impl RegisterBlock {
    ///0x00 - I3C message control register alternate
    #[inline(always)]
    pub const fn cr_alternate(&self) -> &CR_ALTERNATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    ///0x00 - I3C message control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
///CR (w) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///I3C message control register
pub mod cr;
///CR_ALTERNATE (w) register accessor: an alias for `Reg<CR_ALTERNATE_SPEC>`
pub type CR_ALTERNATE = crate::Reg<cr_alternate::CR_ALTERNATE_SPEC>;
///I3C message control register alternate
pub mod cr_alternate;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///I3C configuration register
pub mod cfgr;
///RDR (r) register accessor: an alias for `Reg<RDR_SPEC>`
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
///I3C receive data byte register
pub mod rdr;
///RDWR (r) register accessor: an alias for `Reg<RDWR_SPEC>`
pub type RDWR = crate::Reg<rdwr::RDWR_SPEC>;
///I3C receive data word register
pub mod rdwr;
///TDR (w) register accessor: an alias for `Reg<TDR_SPEC>`
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
///I3C transmit data byte register
pub mod tdr;
///TDWR (w) register accessor: an alias for `Reg<TDWR_SPEC>`
pub type TDWR = crate::Reg<tdwr::TDWR_SPEC>;
///I3C transmit data word register
pub mod tdwr;
///IBIDR (rw) register accessor: an alias for `Reg<IBIDR_SPEC>`
pub type IBIDR = crate::Reg<ibidr::IBIDR_SPEC>;
///I3C IBI payload data register
pub mod ibidr;
///TGTTDR (rw) register accessor: an alias for `Reg<TGTTDR_SPEC>`
pub type TGTTDR = crate::Reg<tgttdr::TGTTDR_SPEC>;
///I3C target transmit configuration register
pub mod tgttdr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///I3C status register
pub mod sr;
///SER (r) register accessor: an alias for `Reg<SER_SPEC>`
pub type SER = crate::Reg<ser::SER_SPEC>;
///I3C status error register
pub mod ser;
///RMR (r) register accessor: an alias for `Reg<RMR_SPEC>`
pub type RMR = crate::Reg<rmr::RMR_SPEC>;
///I3C received message register
pub mod rmr;
///EVR (r) register accessor: an alias for `Reg<EVR_SPEC>`
pub type EVR = crate::Reg<evr::EVR_SPEC>;
///I3C event register
pub mod evr;
///IER (r) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///I3C interrupt enable register
pub mod ier;
///CEVR (w) register accessor: an alias for `Reg<CEVR_SPEC>`
pub type CEVR = crate::Reg<cevr::CEVR_SPEC>;
///I3C clear event register
pub mod cevr;
///DEVR0 (rw) register accessor: an alias for `Reg<DEVR0_SPEC>`
pub type DEVR0 = crate::Reg<devr0::DEVR0_SPEC>;
///I3C own device characteristics register
pub mod devr0;
///DEVR1 (rw) register accessor: an alias for `Reg<DEVR1_SPEC>`
pub type DEVR1 = crate::Reg<devr1::DEVR1_SPEC>;
///I3C device 1 characteristics register
pub mod devr1;
///DEVR2 (rw) register accessor: an alias for `Reg<DEVR2_SPEC>`
pub type DEVR2 = crate::Reg<devr2::DEVR2_SPEC>;
///I3C device 2 characteristics register
pub mod devr2;
///DEVR3 (rw) register accessor: an alias for `Reg<DEVR3_SPEC>`
pub type DEVR3 = crate::Reg<devr3::DEVR3_SPEC>;
///I3C device 3 characteristics register
pub mod devr3;
///DEVR4 (rw) register accessor: an alias for `Reg<DEVR4_SPEC>`
pub type DEVR4 = crate::Reg<devr4::DEVR4_SPEC>;
///I3C device 4 characteristics register
pub mod devr4;
///MAXRLR (rw) register accessor: an alias for `Reg<MAXRLR_SPEC>`
pub type MAXRLR = crate::Reg<maxrlr::MAXRLR_SPEC>;
///I3C maximum read length register
pub mod maxrlr;
///MAXWLR (rw) register accessor: an alias for `Reg<MAXWLR_SPEC>`
pub type MAXWLR = crate::Reg<maxwlr::MAXWLR_SPEC>;
///I3C maximum write length register
pub mod maxwlr;
///TIMINGR0 (rw) register accessor: an alias for `Reg<TIMINGR0_SPEC>`
pub type TIMINGR0 = crate::Reg<timingr0::TIMINGR0_SPEC>;
///I3C timing register 0
pub mod timingr0;
///TIMINGR1 (rw) register accessor: an alias for `Reg<TIMINGR1_SPEC>`
pub type TIMINGR1 = crate::Reg<timingr1::TIMINGR1_SPEC>;
///I3C timing register 1
pub mod timingr1;
///TIMINGR2 (rw) register accessor: an alias for `Reg<TIMINGR2_SPEC>`
pub type TIMINGR2 = crate::Reg<timingr2::TIMINGR2_SPEC>;
///I3C timing register 2
pub mod timingr2;
///BCR (rw) register accessor: an alias for `Reg<BCR_SPEC>`
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
///I3C bus characteristics register
pub mod bcr;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///I3C device characteristics register
pub mod dcr;
///GETCAPR (rw) register accessor: an alias for `Reg<GETCAPR_SPEC>`
pub type GETCAPR = crate::Reg<getcapr::GETCAPR_SPEC>;
///I3C get capability register
pub mod getcapr;
///CRCAPR (rw) register accessor: an alias for `Reg<CRCAPR_SPEC>`
pub type CRCAPR = crate::Reg<crcapr::CRCAPR_SPEC>;
///I3C controller-role capability register
pub mod crcapr;
///GETMXDSR (rw) register accessor: an alias for `Reg<GETMXDSR_SPEC>`
pub type GETMXDSR = crate::Reg<getmxdsr::GETMXDSR_SPEC>;
///I3C get capability register
pub mod getmxdsr;
///EPIDR (rw) register accessor: an alias for `Reg<EPIDR_SPEC>`
pub type EPIDR = crate::Reg<epidr::EPIDR_SPEC>;
///I3C extended provisioned ID register
pub mod epidr;
