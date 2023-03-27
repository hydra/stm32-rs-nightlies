///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DSI Host version register
    pub vr: VR,
    ///0x04 - DSI Host control register
    pub cr: CR,
    ///0x08 - DSI Host clock control register
    pub ccr: CCR,
    ///0x0c - DSI Host LTDC VCID register
    pub lvcidr: LVCIDR,
    ///0x10 - DSI Host LTDC color coding register
    pub lcolcr: LCOLCR,
    ///0x14 - DSI Host LTDC polarity configuration register
    pub lpcr: LPCR,
    ///0x18 - DSI Host low-power mode configuration register
    pub lpmcr: LPMCR,
    _reserved7: [u8; 0x10],
    ///0x2c - DSI Host protocol configuration register
    pub pcr: PCR,
    ///0x30 - DSI Host generic VCID register
    pub gvcidr: GVCIDR,
    ///0x34 - DSI Host mode configuration register
    pub mcr: MCR,
    ///0x38 - DSI Host video mode configuration register
    pub vmcr: VMCR,
    ///0x3c - DSI Host video packet configuration register
    pub vpcr: VPCR,
    ///0x40 - DSI Host video chunks configuration register
    pub vccr: VCCR,
    ///0x44 - DSI Host video null packet configuration register
    pub vnpcr: VNPCR,
    ///0x48 - DSI Host video HSA configuration register
    pub vhsacr: VHSACR,
    ///0x4c - DSI Host video HBP configuration register
    pub vhbpcr: VHBPCR,
    ///0x50 - DSI Host video line configuration register
    pub vlcr: VLCR,
    ///0x54 - DSI Host video VSA configuration register
    pub vvsacr: VVSACR,
    ///0x58 - DSI Host video VBP configuration register
    pub vvbpcr: VVBPCR,
    ///0x5c - DSI Host video VFP configuration register
    pub vvfpcr: VVFPCR,
    ///0x60 - DSI Host video VA configuration register
    pub vvacr: VVACR,
    ///0x64 - DSI Host LTDC command configuration register
    pub lccr: LCCR,
    ///0x68 - DSI Host command mode configuration register
    pub cmcr: CMCR,
    ///0x6c - DSI Host generic header configuration register
    pub ghcr: GHCR,
    ///0x70 - DSI Host generic payload data register
    pub gpdr: GPDR,
    ///0x74 - DSI Host generic packet status register
    pub gpsr: GPSR,
    ///0x78 - DSI Host timeout counter configuration register 0
    pub tccr0: TCCR0,
    ///0x7c - DSI Host timeout counter configuration register 1
    pub tccr1: TCCR1,
    ///0x80 - DSI Host timeout counter configuration register 2
    pub tccr2: TCCR2,
    ///0x84 - DSI Host timeout counter configuration register 3
    pub tccr3: TCCR3,
    ///0x88 - DSI Host timeout counter configuration register 4
    pub tccr4: TCCR4,
    ///0x8c - DSI Host timeout counter configuration register 5
    pub tccr5: TCCR5,
    _reserved32: [u8; 0x04],
    ///0x94 - DSI Host clock lane configuration register
    pub clcr: CLCR,
    ///0x98 - DSI Host clock lane timer configuration register
    pub cltcr: CLTCR,
    ///0x9c - DSI Host data lane timer configuration register
    pub dltcr: DLTCR,
    ///0xa0 - DSI Host PHY control register
    pub pctlr: PCTLR,
    ///0xa4 - DSI Host PHY configuration register
    pub pconfr: PCONFR,
    ///0xa8 - DSI Host PHY ULPS control register
    pub pucr: PUCR,
    ///0xac - DSI Host PHY TX triggers configuration register
    pub pttcr: PTTCR,
    ///0xb0 - DSI Host PHY status register
    pub psr: PSR,
    _reserved40: [u8; 0x08],
    ///0xbc - DSI Host interrupt and status register 0
    pub isr0: ISR0,
    ///0xc0 - DSI Host interrupt and status register 1
    pub isr1: ISR1,
    ///0xc4 - DSI Host interrupt enable register 0
    pub ier0: IER0,
    ///0xc8 - DSI Host interrupt enable register 1
    pub ier1: IER1,
    _reserved44: [u8; 0x0c],
    ///0xd8 - DSI Host force interrupt register 0
    pub fir0: FIR0,
    ///0xdc - DSI Host force interrupt register 1
    pub fir1: FIR1,
    _reserved46: [u8; 0x14],
    ///0xf4 - DSI Host data lane timer read configuration register
    pub dltrcr: DLTRCR,
    _reserved47: [u8; 0x08],
    ///0x100 - DSI Host video shadow control register
    pub vscr: VSCR,
    _reserved48: [u8; 0x08],
    ///0x10c - DSI Host LTDC current VCID register
    pub lcvcidr: LCVCIDR,
    ///0x110 - DSI Host LTDC current color coding register
    pub lcccr: LCCCR,
    _reserved50: [u8; 0x04],
    ///0x118 - DSI Host low-power mode current configuration register
    pub lpmccr: LPMCCR,
    _reserved51: [u8; 0x1c],
    ///0x138 - DSI Host video mode current configuration register
    pub vmccr: VMCCR,
    ///0x13c - DSI Host video packet current configuration register
    pub vpccr: VPCCR,
    ///0x140 - DSI Host video chunks current configuration register
    pub vcccr: VCCCR,
    ///0x144 - DSI Host video null packet current configuration register
    pub vnpccr: VNPCCR,
    ///0x148 - DSI Host video HSA current configuration register
    pub vhsaccr: VHSACCR,
    ///0x14c - DSI Host video HBP current configuration register
    pub vhbpccr: VHBPCCR,
    ///0x150 - DSI Host video line current configuration register
    pub vlccr: VLCCR,
    ///0x154 - DSI Host video VSA current configuration register
    pub vvsaccr: VVSACCR,
    ///0x158 - DSI Host video VBP current configuration register
    pub vvbpccr: VVBPCCR,
    ///0x15c - DSI Host video VFP current configuration register
    pub vvfpccr: VVFPCCR,
    ///0x160 - DSI Host video VA current configuration register
    pub vvaccr: VVACCR,
    _reserved62: [u8; 0x029c],
    ///0x400 - DSI wrapper configuration register
    pub wcfgr: WCFGR,
    ///0x404 - DSI wrapper control register
    pub wcr: WCR,
    ///0x408 - DSI wrapper interrupt enable register
    pub wier: WIER,
    ///0x40c - DSI wrapper interrupt and status register
    pub wisr: WISR,
    ///0x410 - DSI wrapper interrupt flag clear register
    pub wifcr: WIFCR,
    _reserved67: [u8; 0x04],
    ///0x418 - DSI wrapper PHY configuration register 0
    pub wpcr0: WPCR0,
    ///0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).
    pub wpcr1: WPCR1,
    _reserved69: [u8; 0x10],
    ///0x430 - DSI wrapper regulator and PLL control register
    pub wrpcr: WRPCR,
    _reserved70: [u8; 0x03bc],
    ///0x7f0 - DSI Host hardware configuration register
    pub hwcfgr: HWCFGR,
    ///0x7f4 - DSI Host version register
    pub verr: VERR,
    ///0x7f8 - DSI Host identification register
    pub ipidr: IPIDR,
    ///0x7fc - DSI Host size identification register
    pub sidr: SIDR,
}
///VR (r) register accessor: an alias for `Reg<VR_SPEC>`
pub type VR = crate::Reg<vr::VR_SPEC>;
///DSI Host version register
pub mod vr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DSI Host control register
pub mod cr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///DSI Host clock control register
pub mod ccr;
///LVCIDR (rw) register accessor: an alias for `Reg<LVCIDR_SPEC>`
pub type LVCIDR = crate::Reg<lvcidr::LVCIDR_SPEC>;
///DSI Host LTDC VCID register
pub mod lvcidr;
///LCOLCR (rw) register accessor: an alias for `Reg<LCOLCR_SPEC>`
pub type LCOLCR = crate::Reg<lcolcr::LCOLCR_SPEC>;
///DSI Host LTDC color coding register
pub mod lcolcr;
///LPCR (rw) register accessor: an alias for `Reg<LPCR_SPEC>`
pub type LPCR = crate::Reg<lpcr::LPCR_SPEC>;
///DSI Host LTDC polarity configuration register
pub mod lpcr;
///LPMCR (rw) register accessor: an alias for `Reg<LPMCR_SPEC>`
pub type LPMCR = crate::Reg<lpmcr::LPMCR_SPEC>;
///DSI Host low-power mode configuration register
pub mod lpmcr;
///PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
///DSI Host protocol configuration register
pub mod pcr;
///GVCIDR (r) register accessor: an alias for `Reg<GVCIDR_SPEC>`
pub type GVCIDR = crate::Reg<gvcidr::GVCIDR_SPEC>;
///DSI Host generic VCID register
pub mod gvcidr;
///MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
///DSI Host mode configuration register
pub mod mcr;
///VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
///DSI Host video mode configuration register
pub mod vmcr;
///VPCR (rw) register accessor: an alias for `Reg<VPCR_SPEC>`
pub type VPCR = crate::Reg<vpcr::VPCR_SPEC>;
///DSI Host video packet configuration register
pub mod vpcr;
///VCCR (rw) register accessor: an alias for `Reg<VCCR_SPEC>`
pub type VCCR = crate::Reg<vccr::VCCR_SPEC>;
///DSI Host video chunks configuration register
pub mod vccr;
///VNPCR (rw) register accessor: an alias for `Reg<VNPCR_SPEC>`
pub type VNPCR = crate::Reg<vnpcr::VNPCR_SPEC>;
///DSI Host video null packet configuration register
pub mod vnpcr;
///VHSACR (rw) register accessor: an alias for `Reg<VHSACR_SPEC>`
pub type VHSACR = crate::Reg<vhsacr::VHSACR_SPEC>;
///DSI Host video HSA configuration register
pub mod vhsacr;
///VHBPCR (rw) register accessor: an alias for `Reg<VHBPCR_SPEC>`
pub type VHBPCR = crate::Reg<vhbpcr::VHBPCR_SPEC>;
///DSI Host video HBP configuration register
pub mod vhbpcr;
///VLCR (rw) register accessor: an alias for `Reg<VLCR_SPEC>`
pub type VLCR = crate::Reg<vlcr::VLCR_SPEC>;
///DSI Host video line configuration register
pub mod vlcr;
///VVSACR (rw) register accessor: an alias for `Reg<VVSACR_SPEC>`
pub type VVSACR = crate::Reg<vvsacr::VVSACR_SPEC>;
///DSI Host video VSA configuration register
pub mod vvsacr;
///VVBPCR (rw) register accessor: an alias for `Reg<VVBPCR_SPEC>`
pub type VVBPCR = crate::Reg<vvbpcr::VVBPCR_SPEC>;
///DSI Host video VBP configuration register
pub mod vvbpcr;
///VVFPCR (rw) register accessor: an alias for `Reg<VVFPCR_SPEC>`
pub type VVFPCR = crate::Reg<vvfpcr::VVFPCR_SPEC>;
///DSI Host video VFP configuration register
pub mod vvfpcr;
///VVACR (rw) register accessor: an alias for `Reg<VVACR_SPEC>`
pub type VVACR = crate::Reg<vvacr::VVACR_SPEC>;
///DSI Host video VA configuration register
pub mod vvacr;
///LCCR (rw) register accessor: an alias for `Reg<LCCR_SPEC>`
pub type LCCR = crate::Reg<lccr::LCCR_SPEC>;
///DSI Host LTDC command configuration register
pub mod lccr;
///CMCR (rw) register accessor: an alias for `Reg<CMCR_SPEC>`
pub type CMCR = crate::Reg<cmcr::CMCR_SPEC>;
///DSI Host command mode configuration register
pub mod cmcr;
///GHCR (rw) register accessor: an alias for `Reg<GHCR_SPEC>`
pub type GHCR = crate::Reg<ghcr::GHCR_SPEC>;
///DSI Host generic header configuration register
pub mod ghcr;
///GPDR (rw) register accessor: an alias for `Reg<GPDR_SPEC>`
pub type GPDR = crate::Reg<gpdr::GPDR_SPEC>;
///DSI Host generic payload data register
pub mod gpdr;
///GPSR (r) register accessor: an alias for `Reg<GPSR_SPEC>`
pub type GPSR = crate::Reg<gpsr::GPSR_SPEC>;
///DSI Host generic packet status register
pub mod gpsr;
///TCCR0 (rw) register accessor: an alias for `Reg<TCCR0_SPEC>`
pub type TCCR0 = crate::Reg<tccr0::TCCR0_SPEC>;
///DSI Host timeout counter configuration register 0
pub mod tccr0;
///TCCR1 (rw) register accessor: an alias for `Reg<TCCR1_SPEC>`
pub type TCCR1 = crate::Reg<tccr1::TCCR1_SPEC>;
///DSI Host timeout counter configuration register 1
pub mod tccr1;
///TCCR2 (rw) register accessor: an alias for `Reg<TCCR2_SPEC>`
pub type TCCR2 = crate::Reg<tccr2::TCCR2_SPEC>;
///DSI Host timeout counter configuration register 2
pub mod tccr2;
///TCCR3 (rw) register accessor: an alias for `Reg<TCCR3_SPEC>`
pub type TCCR3 = crate::Reg<tccr3::TCCR3_SPEC>;
///DSI Host timeout counter configuration register 3
pub mod tccr3;
///TCCR4 (rw) register accessor: an alias for `Reg<TCCR4_SPEC>`
pub type TCCR4 = crate::Reg<tccr4::TCCR4_SPEC>;
///DSI Host timeout counter configuration register 4
pub mod tccr4;
///TCCR5 (rw) register accessor: an alias for `Reg<TCCR5_SPEC>`
pub type TCCR5 = crate::Reg<tccr5::TCCR5_SPEC>;
///DSI Host timeout counter configuration register 5
pub mod tccr5;
///CLCR (rw) register accessor: an alias for `Reg<CLCR_SPEC>`
pub type CLCR = crate::Reg<clcr::CLCR_SPEC>;
///DSI Host clock lane configuration register
pub mod clcr;
///CLTCR (rw) register accessor: an alias for `Reg<CLTCR_SPEC>`
pub type CLTCR = crate::Reg<cltcr::CLTCR_SPEC>;
///DSI Host clock lane timer configuration register
pub mod cltcr;
///DLTCR (rw) register accessor: an alias for `Reg<DLTCR_SPEC>`
pub type DLTCR = crate::Reg<dltcr::DLTCR_SPEC>;
///DSI Host data lane timer configuration register
pub mod dltcr;
///PCTLR (rw) register accessor: an alias for `Reg<PCTLR_SPEC>`
pub type PCTLR = crate::Reg<pctlr::PCTLR_SPEC>;
///DSI Host PHY control register
pub mod pctlr;
///PCONFR (rw) register accessor: an alias for `Reg<PCONFR_SPEC>`
pub type PCONFR = crate::Reg<pconfr::PCONFR_SPEC>;
///DSI Host PHY configuration register
pub mod pconfr;
///PUCR (rw) register accessor: an alias for `Reg<PUCR_SPEC>`
pub type PUCR = crate::Reg<pucr::PUCR_SPEC>;
///DSI Host PHY ULPS control register
pub mod pucr;
///PTTCR (rw) register accessor: an alias for `Reg<PTTCR_SPEC>`
pub type PTTCR = crate::Reg<pttcr::PTTCR_SPEC>;
///DSI Host PHY TX triggers configuration register
pub mod pttcr;
///PSR (r) register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///DSI Host PHY status register
pub mod psr;
///ISR0 (r) register accessor: an alias for `Reg<ISR0_SPEC>`
pub type ISR0 = crate::Reg<isr0::ISR0_SPEC>;
///DSI Host interrupt and status register 0
pub mod isr0;
///ISR1 (r) register accessor: an alias for `Reg<ISR1_SPEC>`
pub type ISR1 = crate::Reg<isr1::ISR1_SPEC>;
///DSI Host interrupt and status register 1
pub mod isr1;
///IER0 (rw) register accessor: an alias for `Reg<IER0_SPEC>`
pub type IER0 = crate::Reg<ier0::IER0_SPEC>;
///DSI Host interrupt enable register 0
pub mod ier0;
///IER1 (rw) register accessor: an alias for `Reg<IER1_SPEC>`
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
///DSI Host interrupt enable register 1
pub mod ier1;
///FIR0 (w) register accessor: an alias for `Reg<FIR0_SPEC>`
pub type FIR0 = crate::Reg<fir0::FIR0_SPEC>;
///DSI Host force interrupt register 0
pub mod fir0;
///FIR1 (w) register accessor: an alias for `Reg<FIR1_SPEC>`
pub type FIR1 = crate::Reg<fir1::FIR1_SPEC>;
///DSI Host force interrupt register 1
pub mod fir1;
///DLTRCR (rw) register accessor: an alias for `Reg<DLTRCR_SPEC>`
pub type DLTRCR = crate::Reg<dltrcr::DLTRCR_SPEC>;
///DSI Host data lane timer read configuration register
pub mod dltrcr;
///VSCR (rw) register accessor: an alias for `Reg<VSCR_SPEC>`
pub type VSCR = crate::Reg<vscr::VSCR_SPEC>;
///DSI Host video shadow control register
pub mod vscr;
///LCVCIDR (rw) register accessor: an alias for `Reg<LCVCIDR_SPEC>`
pub type LCVCIDR = crate::Reg<lcvcidr::LCVCIDR_SPEC>;
///DSI Host LTDC current VCID register
pub mod lcvcidr;
///LCCCR (r) register accessor: an alias for `Reg<LCCCR_SPEC>`
pub type LCCCR = crate::Reg<lcccr::LCCCR_SPEC>;
///DSI Host LTDC current color coding register
pub mod lcccr;
///LPMCCR (r) register accessor: an alias for `Reg<LPMCCR_SPEC>`
pub type LPMCCR = crate::Reg<lpmccr::LPMCCR_SPEC>;
///DSI Host low-power mode current configuration register
pub mod lpmccr;
///VMCCR (r) register accessor: an alias for `Reg<VMCCR_SPEC>`
pub type VMCCR = crate::Reg<vmccr::VMCCR_SPEC>;
///DSI Host video mode current configuration register
pub mod vmccr;
///VPCCR (r) register accessor: an alias for `Reg<VPCCR_SPEC>`
pub type VPCCR = crate::Reg<vpccr::VPCCR_SPEC>;
///DSI Host video packet current configuration register
pub mod vpccr;
///VCCCR (r) register accessor: an alias for `Reg<VCCCR_SPEC>`
pub type VCCCR = crate::Reg<vcccr::VCCCR_SPEC>;
///DSI Host video chunks current configuration register
pub mod vcccr;
///VNPCCR (r) register accessor: an alias for `Reg<VNPCCR_SPEC>`
pub type VNPCCR = crate::Reg<vnpccr::VNPCCR_SPEC>;
///DSI Host video null packet current configuration register
pub mod vnpccr;
///VHSACCR (r) register accessor: an alias for `Reg<VHSACCR_SPEC>`
pub type VHSACCR = crate::Reg<vhsaccr::VHSACCR_SPEC>;
///DSI Host video HSA current configuration register
pub mod vhsaccr;
///VHBPCCR (r) register accessor: an alias for `Reg<VHBPCCR_SPEC>`
pub type VHBPCCR = crate::Reg<vhbpccr::VHBPCCR_SPEC>;
///DSI Host video HBP current configuration register
pub mod vhbpccr;
///VLCCR (r) register accessor: an alias for `Reg<VLCCR_SPEC>`
pub type VLCCR = crate::Reg<vlccr::VLCCR_SPEC>;
///DSI Host video line current configuration register
pub mod vlccr;
///VVSACCR (r) register accessor: an alias for `Reg<VVSACCR_SPEC>`
pub type VVSACCR = crate::Reg<vvsaccr::VVSACCR_SPEC>;
///DSI Host video VSA current configuration register
pub mod vvsaccr;
///VVBPCCR (r) register accessor: an alias for `Reg<VVBPCCR_SPEC>`
pub type VVBPCCR = crate::Reg<vvbpccr::VVBPCCR_SPEC>;
///DSI Host video VBP current configuration register
pub mod vvbpccr;
///VVFPCCR (r) register accessor: an alias for `Reg<VVFPCCR_SPEC>`
pub type VVFPCCR = crate::Reg<vvfpccr::VVFPCCR_SPEC>;
///DSI Host video VFP current configuration register
pub mod vvfpccr;
///VVACCR (r) register accessor: an alias for `Reg<VVACCR_SPEC>`
pub type VVACCR = crate::Reg<vvaccr::VVACCR_SPEC>;
///DSI Host video VA current configuration register
pub mod vvaccr;
///WCFGR (rw) register accessor: an alias for `Reg<WCFGR_SPEC>`
pub type WCFGR = crate::Reg<wcfgr::WCFGR_SPEC>;
///DSI wrapper configuration register
pub mod wcfgr;
///WCR (rw) register accessor: an alias for `Reg<WCR_SPEC>`
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
///DSI wrapper control register
pub mod wcr;
///WIER (rw) register accessor: an alias for `Reg<WIER_SPEC>`
pub type WIER = crate::Reg<wier::WIER_SPEC>;
///DSI wrapper interrupt enable register
pub mod wier;
///WISR (r) register accessor: an alias for `Reg<WISR_SPEC>`
pub type WISR = crate::Reg<wisr::WISR_SPEC>;
///DSI wrapper interrupt and status register
pub mod wisr;
///WIFCR (w) register accessor: an alias for `Reg<WIFCR_SPEC>`
pub type WIFCR = crate::Reg<wifcr::WIFCR_SPEC>;
///DSI wrapper interrupt flag clear register
pub mod wifcr;
///WPCR0 (rw) register accessor: an alias for `Reg<WPCR0_SPEC>`
pub type WPCR0 = crate::Reg<wpcr0::WPCR0_SPEC>;
///DSI wrapper PHY configuration register 0
pub mod wpcr0;
///WPCR1 (rw) register accessor: an alias for `Reg<WPCR1_SPEC>`
pub type WPCR1 = crate::Reg<wpcr1::WPCR1_SPEC>;
///This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).
pub mod wpcr1;
///WRPCR (rw) register accessor: an alias for `Reg<WRPCR_SPEC>`
pub type WRPCR = crate::Reg<wrpcr::WRPCR_SPEC>;
///DSI wrapper regulator and PLL control register
pub mod wrpcr;
///HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
///DSI Host hardware configuration register
pub mod hwcfgr;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///DSI Host version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///DSI Host identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///DSI Host size identification register
pub mod sidr;
