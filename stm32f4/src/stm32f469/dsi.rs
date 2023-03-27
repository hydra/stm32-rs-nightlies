///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DSI Host Version Register
    pub vr: VR,
    ///0x04 - DSI Host Control Register
    pub cr: CR,
    ///0x08 - DSI HOST Clock Control Register
    pub ccr: CCR,
    ///0x0c - DSI Host LTDC VCID Register
    pub lvcidr: LVCIDR,
    ///0x10 - DSI Host LTDC Color Coding Register
    pub lcolcr: LCOLCR,
    ///0x14 - DSI Host LTDC Polarity Configuration Register
    pub lpcr: LPCR,
    ///0x18 - DSI Host Low-Power Mode Configuration Register
    pub lpmcr: LPMCR,
    _reserved7: [u8; 0x10],
    ///0x2c - DSI Host Protocol Configuration Register
    pub pcr: PCR,
    ///0x30 - DSI Host Generic VCID Register
    pub gvcidr: GVCIDR,
    ///0x34 - DSI Host Mode Configuration Register
    pub mcr: MCR,
    ///0x38 - DSI Host Video mode Configuration Register
    pub vmcr: VMCR,
    ///0x3c - DSI Host Video Packet Configuration Register
    pub vpcr: VPCR,
    ///0x40 - DSI Host Video Chunks Configuration Register
    pub vccr: VCCR,
    ///0x44 - DSI Host Video Null Packet Configuration Register
    pub vnpcr: VNPCR,
    ///0x48 - DSI Host Video HSA Configuration Register
    pub vhsacr: VHSACR,
    ///0x4c - DSI Host Video HBP Configuration Register
    pub vhbpcr: VHBPCR,
    ///0x50 - DSI Host Video Line Configuration Register
    pub vlcr: VLCR,
    ///0x54 - DSI Host Video VSA Configuration Register
    pub vvsacr: VVSACR,
    ///0x58 - DSI Host Video VBP Configuration Register
    pub vvbpcr: VVBPCR,
    ///0x5c - DSI Host Video VFP Configuration Register
    pub vvfpcr: VVFPCR,
    ///0x60 - DSI Host Video VA Configuration Register
    pub vvacr: VVACR,
    ///0x64 - DSI Host LTDC Command Configuration Register
    pub lccr: LCCR,
    ///0x68 - DSI Host Command mode Configuration Register
    pub cmcr: CMCR,
    ///0x6c - DSI Host Generic Header Configuration Register
    pub ghcr: GHCR,
    ///0x70 - DSI Host Generic Payload Data Register
    pub gpdr: GPDR,
    ///0x74 - DSI Host Generic Packet Status Register
    pub gpsr: GPSR,
    ///0x78 - DSI Host Timeout Counter Configuration Register1
    pub tccr1: TCCR1,
    ///0x7c - DSI Host Timeout Counter Configuration Register2
    pub tccr2: TCCR2,
    ///0x80 - DSI Host Timeout Counter Configuration Register3
    pub tccr3: TCCR3,
    ///0x84 - DSI Host Timeout Counter Configuration Register4
    pub tccr4: TCCR4,
    ///0x88 - DSI Host Timeout Counter Configuration Register5
    pub tccr5: TCCR5,
    ///0x8c - DSI Host Timeout Counter Configuration Register6
    pub tccr6: TCCR6,
    _reserved32: [u8; 0x04],
    ///0x94 - DSI Host Clock Lane Configuration Register
    pub clcr: CLCR,
    ///0x98 - DSI Host Clock Lane Timer Configuration Register
    pub cltcr: CLTCR,
    ///0x9c - DSI Host Data Lane Timer Configuration Register
    pub dltcr: DLTCR,
    ///0xa0 - DSI Host PHY Control Register
    pub pctlr: PCTLR,
    ///0xa4 - DSI Host PHY Configuration Register
    pub pcconfr: PCCONFR,
    ///0xa8 - DSI Host PHY ULPS Control Register
    pub pucr: PUCR,
    ///0xac - DSI Host PHY TX Triggers Configuration Register
    pub pttcr: PTTCR,
    ///0xb0 - DSI Host PHY Status Register
    pub psr: PSR,
    _reserved40: [u8; 0x08],
    ///0xbc - DSI Host Interrupt &amp; Status Register 0
    pub isr0: ISR0,
    ///0xc0 - DSI Host Interrupt &amp; Status Register 1
    pub isr1: ISR1,
    ///0xc4 - DSI Host Interrupt Enable Register 0
    pub ier0: IER0,
    ///0xc8 - DSI Host Interrupt Enable Register 1
    pub ier1: IER1,
    _reserved44: [u8; 0x0c],
    ///0xd8 - DSI Host Force Interrupt Register 0
    pub fir0: FIR0,
    ///0xdc - DSI Host Force Interrupt Register 1
    pub fir1: FIR1,
    _reserved46: [u8; 0x20],
    ///0x100 - DSI Host Video Shadow Control Register
    pub vscr: VSCR,
    _reserved47: [u8; 0x08],
    ///0x10c - DSI Host LTDC Current VCID Register
    pub lcvcidr: LCVCIDR,
    ///0x110 - DSI Host LTDC Current Color Coding Register
    pub lcccr: LCCCR,
    _reserved49: [u8; 0x04],
    ///0x118 - DSI Host Low-power Mode Current Configuration Register
    pub lpmccr: LPMCCR,
    _reserved50: [u8; 0x1c],
    ///0x138 - DSI Host Video mode Current Configuration Register
    pub vmccr: VMCCR,
    ///0x13c - DSI Host Video Packet Current Configuration Register
    pub vpccr: VPCCR,
    ///0x140 - DSI Host Video Chunks Current Configuration Register
    pub vcccr: VCCCR,
    ///0x144 - DSI Host Video Null Packet Current Configuration Register
    pub vnpccr: VNPCCR,
    ///0x148 - DSI Host Video HSA Current Configuration Register
    pub vhsaccr: VHSACCR,
    ///0x14c - DSI Host Video HBP Current Configuration Register
    pub vhbpccr: VHBPCCR,
    ///0x150 - DSI Host Video Line Current Configuration Register
    pub vlccr: VLCCR,
    ///0x154 - DSI Host Video VSA Current Configuration Register
    pub vvsaccr: VVSACCR,
    ///0x158 - DSI Host Video VBP Current Configuration Register
    pub vvbpccr: VVBPCCR,
    ///0x15c - DSI Host Video VFP Current Configuration Register
    pub vvfpccr: VVFPCCR,
    ///0x160 - DSI Host Video VA Current Configuration Register
    pub vvaccr: VVACCR,
    _reserved61: [u8; 0x029c],
    ///0x400 - DSI Wrapper Configuration Register
    pub wcfgr: WCFGR,
    ///0x404 - DSI Wrapper Control Register
    pub wcr: WCR,
    ///0x408 - DSI Wrapper Interrupt Enable Register
    pub wier: WIER,
    ///0x40c - DSI Wrapper Interrupt &amp; Status Register
    pub wisr: WISR,
    ///0x410 - DSI Wrapper Interrupt Flag Clear Register
    pub wifcr: WIFCR,
    _reserved66: [u8; 0x04],
    ///0x418 - DSI Wrapper PHY Configuration Register 1
    pub wpcr1: WPCR1,
    ///0x41c - DSI Wrapper PHY Configuration Register 2
    pub wpcr2: WPCR2,
    ///0x420 - DSI Wrapper PHY Configuration Register 3
    pub wpcr3: WPCR3,
    ///0x424 - DSI_WPCR4
    pub wpcr4: WPCR4,
    ///0x428 - DSI Wrapper PHY Configuration Register 5
    pub wpcr5: WPCR5,
    _reserved71: [u8; 0x04],
    ///0x430 - DSI Wrapper Regulator and PLL Control Register
    pub wrpcr: WRPCR,
}
///VR (rw) register accessor: an alias for `Reg<VR_SPEC>`
pub type VR = crate::Reg<vr::VR_SPEC>;
///DSI Host Version Register
pub mod vr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DSI Host Control Register
pub mod cr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///DSI HOST Clock Control Register
pub mod ccr;
///LVCIDR (rw) register accessor: an alias for `Reg<LVCIDR_SPEC>`
pub type LVCIDR = crate::Reg<lvcidr::LVCIDR_SPEC>;
///DSI Host LTDC VCID Register
pub mod lvcidr;
///LCOLCR (rw) register accessor: an alias for `Reg<LCOLCR_SPEC>`
pub type LCOLCR = crate::Reg<lcolcr::LCOLCR_SPEC>;
///DSI Host LTDC Color Coding Register
pub mod lcolcr;
///LPCR (rw) register accessor: an alias for `Reg<LPCR_SPEC>`
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
///DSI Host LTDC Polarity Configuration Register
pub mod lpcr;
///LPMCR (rw) register accessor: an alias for `Reg<LPMCR_SPEC>`
pub type LPMCR = crate::Reg<lpmcr::LPMCR_SPEC>;
///DSI Host Low-Power Mode Configuration Register
pub mod lpmcr;
///PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
///DSI Host Protocol Configuration Register
pub mod pcr;
///GVCIDR (rw) register accessor: an alias for `Reg<GVCIDR_SPEC>`
pub type GVCIDR = crate::Reg<gvcidr::GVCIDR_SPEC>;
///DSI Host Generic VCID Register
pub mod gvcidr;
///MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///DSI Host Mode Configuration Register
pub mod mcr;
///VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
///DSI Host Video mode Configuration Register
pub mod vmcr;
///VPCR (rw) register accessor: an alias for `Reg<VPCR_SPEC>`
pub type VPCR = crate::Reg<vpcr::VPCR_SPEC>;
///DSI Host Video Packet Configuration Register
pub mod vpcr;
///VCCR (rw) register accessor: an alias for `Reg<VCCR_SPEC>`
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
///DSI Host Video Chunks Configuration Register
pub mod vccr;
///VNPCR (rw) register accessor: an alias for `Reg<VNPCR_SPEC>`
pub type VNPCR = crate::Reg<vnpcr::VNPCR_SPEC>;
///DSI Host Video Null Packet Configuration Register
pub mod vnpcr;
///VHSACR (rw) register accessor: an alias for `Reg<VHSACR_SPEC>`
pub type VHSACR = crate::Reg<vhsacr::VHSACR_SPEC>;
///DSI Host Video HSA Configuration Register
pub mod vhsacr;
///VHBPCR (rw) register accessor: an alias for `Reg<VHBPCR_SPEC>`
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCR_SPEC>;
///DSI Host Video HBP Configuration Register
pub mod vhbpcr;
///VLCR (rw) register accessor: an alias for `Reg<VLCR_SPEC>`
pub type VLCR = crate::Reg<vlcr::VLCR_SPEC>;
///DSI Host Video Line Configuration Register
pub mod vlcr;
///VVSACR (rw) register accessor: an alias for `Reg<VVSACR_SPEC>`
pub type VVSACR = crate::Reg<vvsacr::VVSACR_SPEC>;
///DSI Host Video VSA Configuration Register
pub mod vvsacr;
///VVBPCR (rw) register accessor: an alias for `Reg<VVBPCR_SPEC>`
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCR_SPEC>;
///DSI Host Video VBP Configuration Register
pub mod vvbpcr;
///VVFPCR (rw) register accessor: an alias for `Reg<VVFPCR_SPEC>`
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCR_SPEC>;
///DSI Host Video VFP Configuration Register
pub mod vvfpcr;
///VVACR (rw) register accessor: an alias for `Reg<VVACR_SPEC>`
pub type VVACR = crate::Reg<vvacr::VVACR_SPEC>;
///DSI Host Video VA Configuration Register
pub mod vvacr;
///LCCR (rw) register accessor: an alias for `Reg<LCCR_SPEC>`
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
///DSI Host LTDC Command Configuration Register
pub mod lccr;
///CMCR (rw) register accessor: an alias for `Reg<CMCR_SPEC>`
pub type CMCR = crate::Reg<cmcr::CMCR_SPEC>;
///DSI Host Command mode Configuration Register
pub mod cmcr;
///GHCR (rw) register accessor: an alias for `Reg<GHCR_SPEC>`
pub type GHCR = crate::Reg<ghcr::GHCR_SPEC>;
///DSI Host Generic Header Configuration Register
pub mod ghcr;
///GPDR (rw) register accessor: an alias for `Reg<GPDR_SPEC>`
pub type GPDR = crate::Reg<gpdr::GPDR_SPEC>;
///DSI Host Generic Payload Data Register
pub mod gpdr;
///GPSR (rw) register accessor: an alias for `Reg<GPSR_SPEC>`
pub type GPSR = crate::Reg<gpsr::GPSR_SPEC>;
///DSI Host Generic Packet Status Register
pub mod gpsr;
///TCCR1 (rw) register accessor: an alias for `Reg<TCCR1_SPEC>`
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
///DSI Host Timeout Counter Configuration Register1
pub mod tccr1;
///TCCR2 (rw) register accessor: an alias for `Reg<TCCR2_SPEC>`
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
///DSI Host Timeout Counter Configuration Register2
pub mod tccr2;
///TCCR3 (rw) register accessor: an alias for `Reg<TCCR3_SPEC>`
pub type TCCR3 = crate::Reg<tccr3::TCCR3_SPEC>;
///DSI Host Timeout Counter Configuration Register3
pub mod tccr3;
///TCCR4 (rw) register accessor: an alias for `Reg<TCCR4_SPEC>`
pub type TCCR4 = crate::Reg<tccr4::TCCR4_SPEC>;
///DSI Host Timeout Counter Configuration Register4
pub mod tccr4;
///TCCR5 (rw) register accessor: an alias for `Reg<TCCR5_SPEC>`
pub type TCCR5 = crate::Reg<tccr5::TCCR5_SPEC>;
///DSI Host Timeout Counter Configuration Register5
pub mod tccr5;
///TCCR6 (rw) register accessor: an alias for `Reg<TCCR6_SPEC>`
pub type TCCR6 = crate::Reg<tccr6::TCCR6_SPEC>;
///DSI Host Timeout Counter Configuration Register6
pub mod tccr6;
///CLCR (rw) register accessor: an alias for `Reg<CLCR_SPEC>`
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
///DSI Host Clock Lane Configuration Register
pub mod clcr;
///CLTCR (rw) register accessor: an alias for `Reg<CLTCR_SPEC>`
pub type CLTCR = crate::Reg<cltcr::CLTCR_SPEC>;
///DSI Host Clock Lane Timer Configuration Register
pub mod cltcr;
///DLTCR (rw) register accessor: an alias for `Reg<DLTCR_SPEC>`
pub type DLTCR = crate::Reg<dltcr::DLTCR_SPEC>;
///DSI Host Data Lane Timer Configuration Register
pub mod dltcr;
///PCTLR (rw) register accessor: an alias for `Reg<PCTLR_SPEC>`
pub type PCTLR = crate::Reg<pctlr::PCTLR_SPEC>;
///DSI Host PHY Control Register
pub mod pctlr;
///PCCONFR (rw) register accessor: an alias for `Reg<PCCONFR_SPEC>`
pub type PCCONFR = crate::Reg<pcconfr::PCCONFR_SPEC>;
///DSI Host PHY Configuration Register
pub mod pcconfr;
///PUCR (rw) register accessor: an alias for `Reg<PUCR_SPEC>`
pub type PUCR = crate::Reg<pucr::PUCR_SPEC>;
///DSI Host PHY ULPS Control Register
pub mod pucr;
///PTTCR (rw) register accessor: an alias for `Reg<PTTCR_SPEC>`
pub type PTTCR = crate::Reg<pttcr::PTTCR_SPEC>;
///DSI Host PHY TX Triggers Configuration Register
pub mod pttcr;
///PSR (rw) register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///DSI Host PHY Status Register
pub mod psr;
///ISR0 (r) register accessor: an alias for `Reg<ISR0_SPEC>`
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
///DSI Host Interrupt &amp; Status Register 0
pub mod isr0;
///ISR1 (r) register accessor: an alias for `Reg<ISR1_SPEC>`
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
///DSI Host Interrupt &amp; Status Register 1
pub mod isr1;
///IER0 (rw) register accessor: an alias for `Reg<IER0_SPEC>`
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
///DSI Host Interrupt Enable Register 0
pub mod ier0;
///IER1 (rw) register accessor: an alias for `Reg<IER1_SPEC>`
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
///DSI Host Interrupt Enable Register 1
pub mod ier1;
///FIR0 (rw) register accessor: an alias for `Reg<FIR0_SPEC>`
pub type FIR0 = crate::Reg<fir0::FIR0_SPEC>;
///DSI Host Force Interrupt Register 0
pub mod fir0;
///FIR1 (rw) register accessor: an alias for `Reg<FIR1_SPEC>`
pub type FIR1 = crate::Reg<fir1::FIR1_SPEC>;
///DSI Host Force Interrupt Register 1
pub mod fir1;
///VSCR (rw) register accessor: an alias for `Reg<VSCR_SPEC>`
pub type VSCR = crate::Reg<vscr::VSCR_SPEC>;
///DSI Host Video Shadow Control Register
pub mod vscr;
///LCVCIDR (rw) register accessor: an alias for `Reg<LCVCIDR_SPEC>`
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDR_SPEC>;
///DSI Host LTDC Current VCID Register
pub mod lcvcidr;
///LCCCR (rw) register accessor: an alias for `Reg<LCCCR_SPEC>`
pub type LCCCR = crate::Reg<lcccr::LCCCR_SPEC>;
///DSI Host LTDC Current Color Coding Register
pub mod lcccr;
///LPMCCR (rw) register accessor: an alias for `Reg<LPMCCR_SPEC>`
pub type LPMCCR = crate::Reg<lpmccr::LPMCCR_SPEC>;
///DSI Host Low-power Mode Current Configuration Register
pub mod lpmccr;
///VMCCR (rw) register accessor: an alias for `Reg<VMCCR_SPEC>`
pub type VMCCR = crate::Reg<vmccr::VMCCR_SPEC>;
///DSI Host Video mode Current Configuration Register
pub mod vmccr;
///VPCCR (rw) register accessor: an alias for `Reg<VPCCR_SPEC>`
pub type VPCCR = crate::Reg<vpccr::VPCCR_SPEC>;
///DSI Host Video Packet Current Configuration Register
pub mod vpccr;
///VCCCR (rw) register accessor: an alias for `Reg<VCCCR_SPEC>`
pub type VCCCR = crate::Reg<vcccr::VCCCR_SPEC>;
///DSI Host Video Chunks Current Configuration Register
pub mod vcccr;
///VNPCCR (rw) register accessor: an alias for `Reg<VNPCCR_SPEC>`
pub type VNPCCR = crate::Reg<vnpccr::VNPCCR_SPEC>;
///DSI Host Video Null Packet Current Configuration Register
pub mod vnpccr;
///VHSACCR (rw) register accessor: an alias for `Reg<VHSACCR_SPEC>`
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCR_SPEC>;
///DSI Host Video HSA Current Configuration Register
pub mod vhsaccr;
///VHBPCCR (rw) register accessor: an alias for `Reg<VHBPCCR_SPEC>`
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCR_SPEC>;
///DSI Host Video HBP Current Configuration Register
pub mod vhbpccr;
///VLCCR (rw) register accessor: an alias for `Reg<VLCCR_SPEC>`
pub type VLCCR = crate::Reg<vlccr::VLCCR_SPEC>;
///DSI Host Video Line Current Configuration Register
pub mod vlccr;
///VVSACCR (rw) register accessor: an alias for `Reg<VVSACCR_SPEC>`
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCR_SPEC>;
///DSI Host Video VSA Current Configuration Register
pub mod vvsaccr;
///VVBPCCR (rw) register accessor: an alias for `Reg<VVBPCCR_SPEC>`
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCR_SPEC>;
///DSI Host Video VBP Current Configuration Register
pub mod vvbpccr;
///VVFPCCR (rw) register accessor: an alias for `Reg<VVFPCCR_SPEC>`
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCR_SPEC>;
///DSI Host Video VFP Current Configuration Register
pub mod vvfpccr;
///VVACCR (rw) register accessor: an alias for `Reg<VVACCR_SPEC>`
pub type VVACCR = crate::Reg<vvaccr::VVACCR_SPEC>;
///DSI Host Video VA Current Configuration Register
pub mod vvaccr;
///WCFGR (rw) register accessor: an alias for `Reg<WCFGR_SPEC>`
pub type WCFGR = crate::Reg<wcfgr::WCFGR_SPEC>;
///DSI Wrapper Configuration Register
pub mod wcfgr;
///WCR (rw) register accessor: an alias for `Reg<WCR_SPEC>`
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
///DSI Wrapper Control Register
pub mod wcr;
///WIER (rw) register accessor: an alias for `Reg<WIER_SPEC>`
pub type WIER = crate::Reg<wier::WIER_SPEC>;
///DSI Wrapper Interrupt Enable Register
pub mod wier;
///WISR (r) register accessor: an alias for `Reg<WISR_SPEC>`
pub type WISR = crate::Reg<wisr::WISR_SPEC>;
///DSI Wrapper Interrupt &amp; Status Register
pub mod wisr;
///WIFCR (rw) register accessor: an alias for `Reg<WIFCR_SPEC>`
pub type WIFCR = crate::Reg<wifcr::WIFCR_SPEC>;
///DSI Wrapper Interrupt Flag Clear Register
pub mod wifcr;
///WPCR1 (rw) register accessor: an alias for `Reg<WPCR1_SPEC>`
pub type WPCR1 = crate::Reg<wpcr1::WPCR1_SPEC>;
///DSI Wrapper PHY Configuration Register 1
pub mod wpcr1;
///WPCR2 (rw) register accessor: an alias for `Reg<WPCR2_SPEC>`
pub type WPCR2 = crate::Reg<wpcr2::WPCR2_SPEC>;
///DSI Wrapper PHY Configuration Register 2
pub mod wpcr2;
///WPCR3 (rw) register accessor: an alias for `Reg<WPCR3_SPEC>`
pub type WPCR3 = crate::Reg<wpcr3::WPCR3_SPEC>;
///DSI Wrapper PHY Configuration Register 3
pub mod wpcr3;
///WPCR4 (rw) register accessor: an alias for `Reg<WPCR4_SPEC>`
pub type WPCR4 = crate::Reg<wpcr4::WPCR4_SPEC>;
///DSI_WPCR4
pub mod wpcr4;
///WPCR5 (rw) register accessor: an alias for `Reg<WPCR5_SPEC>`
pub type WPCR5 = crate::Reg<wpcr5::WPCR5_SPEC>;
///DSI Wrapper PHY Configuration Register 5
pub mod wpcr5;
///WRPCR (rw) register accessor: an alias for `Reg<WRPCR_SPEC>`
pub type WRPCR = crate::Reg<wrpcr::WRPCR_SPEC>;
///DSI Wrapper Regulator and PLL Control Register
pub mod wrpcr;
