///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Operating mode configuration register
    pub maccr: MACCR,
    ///0x04 - Extended operating mode configuration register
    pub macecr: MACECR,
    ///0x08 - Packet filtering control register
    pub macpfr: MACPFR,
    ///0x0c - Watchdog timeout register
    pub macwtr: MACWTR,
    ///0x10 - Hash Table 0 register
    pub macht0r: MACHT0R,
    ///0x14 - Hash Table 1 register
    pub macht1r: MACHT1R,
    _reserved6: [u8; 0x38],
    ///0x50 - VLAN tag register
    pub macvtr: MACVTR,
    _reserved7: [u8; 0x04],
    ///0x58 - VLAN Hash table register
    pub macvhtr: MACVHTR,
    _reserved8: [u8; 0x04],
    ///0x60 - VLAN inclusion register
    pub macvir: MACVIR,
    ///0x64 - Inner VLAN inclusion register
    pub macivir: MACIVIR,
    _reserved10: [u8; 0x08],
    ///0x70 - Tx Queue flow control register
    pub macqtxfcr: MACQTXFCR,
    _reserved11: [u8; 0x1c],
    ///0x90 - Rx flow control register
    pub macrxfcr: MACRXFCR,
    _reserved12: [u8; 0x1c],
    ///0xb0 - Interrupt status register
    pub macisr: MACISR,
    ///0xb4 - Interrupt enable register
    pub macier: MACIER,
    ///0xb8 - Rx Tx status register
    pub macrxtxsr: MACRXTXSR,
    _reserved15: [u8; 0x04],
    ///0xc0 - PMT control status register
    pub macpcsr: MACPCSR,
    ///0xc4 - Remote wakeup packet filter register
    pub macrwkpfr: MACRWKPFR,
    _reserved17: [u8; 0x08],
    ///0xd0 - LPI control and status register
    pub maclcsr: MACLCSR,
    ///0xd4 - LPI timers control register
    pub macltcr: MACLTCR,
    ///0xd8 - LPI entry timer register
    pub macletr: MACLETR,
    ///0xdc - One-microsecond-tick counter register
    pub mac1ustcr: MAC1USTCR,
    _reserved21: [u8; 0x30],
    ///0x110 - Version register
    pub macvr: MACVR,
    ///0x114 - Debug register
    pub macdr: MACDR,
    _reserved23: [u8; 0x04],
    ///0x11c - HW feature 0 register
    pub machwf0r: MACHWF0R,
    ///0x120 - HW feature 1 register
    pub machwf1r: MACHWF1R,
    ///0x124 - HW feature 2 register
    pub machwf2r: MACHWF2R,
    ///0x128 - HW feature 3 register
    pub machwf3r: MACHWF3R,
    _reserved27: [u8; 0xd4],
    ///0x200 - MDIO address register
    pub macmdioar: MACMDIOAR,
    ///0x204 - MDIO data register
    pub macmdiodr: MACMDIODR,
    _reserved29: [u8; 0x08],
    ///0x210 - ARP address register
    pub macarpar: MACARPAR,
    _reserved30: [u8; 0x1c],
    ///0x230 - CSR software control register
    pub maccsrswcr: MACCSRSWCR,
    _reserved31: [u8; 0xcc],
    ///0x300 - MAC Address 0 high register
    pub maca0hr: MACA0HR,
    ///0x304 - MAC Address 0 low register
    pub maca0lr: MACA0LR,
    ///0x308 - MAC Address 1 high register
    pub maca1hr: MACA1HR,
    ///0x30c - MAC Address 1 low register
    pub maca1lr: MACA1LR,
    ///0x310 - MAC Address 2 high register
    pub maca2hr: MACA2HR,
    ///0x314 - MAC Address 2 low register
    pub maca2lr: MACA2LR,
    ///0x318 - MAC Address 3 high register
    pub maca3hr: MACA3HR,
    ///0x31c - MAC Address 3 low register
    pub maca3lr: MACA3LR,
    _reserved39: [u8; 0x03e0],
    ///0x700 - MMC control register
    pub mmc_control: MMC_CONTROL,
    ///0x704 - MMC Rx interrupt register
    pub mmc_rx_interrupt: MMC_RX_INTERRUPT,
    ///0x708 - MMC Tx interrupt register
    pub mmc_tx_interrupt: MMC_TX_INTERRUPT,
    ///0x70c - MMC Rx interrupt mask register
    pub mmc_rx_interrupt_mask: MMC_RX_INTERRUPT_MASK,
    ///0x710 - MMC Tx interrupt mask register
    pub mmc_tx_interrupt_mask: MMC_TX_INTERRUPT_MASK,
    _reserved44: [u8; 0x38],
    ///0x74c - Tx single collision good packets register
    pub tx_single_collision_good_packets: TX_SINGLE_COLLISION_GOOD_PACKETS,
    ///0x750 - Tx multiple collision good packets register
    pub tx_multiple_collision_good_packets: TX_MULTIPLE_COLLISION_GOOD_PACKETS,
    _reserved46: [u8; 0x14],
    ///0x768 - Tx packet count good register
    pub tx_packet_count_good: TX_PACKET_COUNT_GOOD,
    _reserved47: [u8; 0x28],
    ///0x794 - Rx CRC error packets register
    pub rx_crc_error_packets: RX_CRC_ERROR_PACKETS,
    ///0x798 - Rx alignment error packets register
    pub rx_alignment_error_packets: RX_ALIGNMENT_ERROR_PACKETS,
    _reserved49: [u8; 0x28],
    ///0x7c4 - Rx unicast packets good register
    pub rx_unicast_packets_good: RX_UNICAST_PACKETS_GOOD,
    _reserved50: [u8; 0x24],
    ///0x7ec - Tx LPI microsecond timer register
    pub tx_lpi_usec_cntr: TX_LPI_USEC_CNTR,
    ///0x7f0 - Tx LPI transition counter register
    pub tx_lpi_tran_cntr: TX_LPI_TRAN_CNTR,
    ///0x7f4 - Rx LPI microsecond counter register
    pub rx_lpi_usec_cntr: RX_LPI_USEC_CNTR,
    ///0x7f8 - Rx LPI transition counter register
    pub rx_lpi_tran_cntr: RX_LPI_TRAN_CNTR,
    _reserved54: [u8; 0x0104],
    ///0x900 - L3 and L4 control 0 register
    pub macl3l4c0r: MACL3L4C0R,
    ///0x904 - Layer4 Address filter 0 register
    pub macl4a0r: MACL4A0R,
    _reserved56: [u8; 0x08],
    ///0x910 - Layer3 Address 0 filter 0 register
    pub macl3a00r: MACL3A00R,
    ///0x914 - Layer3 Address 1 filter 0 register
    pub macl3a10r: MACL3A10R,
    ///0x918 - Layer3 Address 2 filter 0 register
    pub macl3a20r: MACL3A20R,
    ///0x91c - Layer3 Address 3 filter 0 register
    pub macl3a30r: MACL3A30R,
    _reserved60: [u8; 0x10],
    ///0x930 - L3 and L4 control 1 register
    pub macl3l4c1r: MACL3L4C1R,
    ///0x934 - Layer 4 address filter 1 register
    pub macl4a1r: MACL4A1R,
    _reserved62: [u8; 0x08],
    ///0x940 - Layer3 address 0 filter 1 Register
    pub macl3a01r: MACL3A01R,
    ///0x944 - Layer3 address 1 filter 1 register
    pub macl3a11r: MACL3A11R,
    ///0x948 - Layer3 address 2 filter 1 Register
    pub macl3a21r: MACL3A21R,
    ///0x94c - Layer3 address 3 filter 1 register
    pub macl3a31r: MACL3A31R,
    _reserved66: [u8; 0x01b0],
    ///0xb00 - Timestamp control Register
    pub mactscr: MACTSCR,
    ///0xb04 - Subsecond increment register
    pub macssir: MACSSIR,
    ///0xb08 - System time seconds register
    pub macstsr: MACSTSR,
    ///0xb0c - System time nanoseconds register
    pub macstnr: MACSTNR,
    ///0xb10 - System time seconds update register
    pub macstsur: MACSTSUR,
    ///0xb14 - System time nanoseconds update register
    pub macstnur: MACSTNUR,
    ///0xb18 - Timestamp addend register
    pub mactsar: MACTSAR,
    _reserved73: [u8; 0x04],
    ///0xb20 - Timestamp status register
    pub mactssr: MACTSSR,
    _reserved74: [u8; 0x0c],
    ///0xb30 - Tx timestamp status nanoseconds register
    pub mactxtssnr: MACTXTSSNR,
    ///0xb34 - Tx timestamp status seconds register
    pub mactxtsssr: MACTXTSSSR,
    _reserved76: [u8; 0x08],
    ///0xb40 - Auxiliary control register
    pub macacr: MACACR,
    _reserved77: [u8; 0x04],
    ///0xb48 - Auxiliary timestamp nanoseconds register
    pub macatsnr: MACATSNR,
    ///0xb4c - Auxiliary timestamp seconds register
    pub macatssr: MACATSSR,
    ///0xb50 - Timestamp Ingress asymmetric correction register
    pub mactsiacr: MACTSIACR,
    ///0xb54 - Timestamp Egress asymmetric correction register
    pub mactseacr: MACTSEACR,
    ///0xb58 - Timestamp Ingress correction nanosecond register
    pub mactsicnr: MACTSICNR,
    ///0xb5c - Timestamp Egress correction nanosecond register
    pub mactsecnr: MACTSECNR,
    _reserved83: [u8; 0x10],
    _reserved_83_macppscr: [u8; 0x04],
    _reserved84: [u8; 0x0c],
    ///0xb80 - PPS target time seconds register
    pub macppsttsr: MACPPSTTSR,
    ///0xb84 - PPS target time nanoseconds register
    pub macppsttnr: MACPPSTTNR,
    ///0xb88 - PPS interval register
    pub macppsir: MACPPSIR,
    ///0xb8c - PPS width register
    pub macppswr: MACPPSWR,
    _reserved88: [u8; 0x30],
    ///0xbc0 - PTP Offload control register
    pub macpocr: MACPOCR,
    ///0xbc4 - PTP Source Port Identity 0 Register
    pub macspi0r: MACSPI0R,
    ///0xbc8 - PTP Source port identity 1 register
    pub macspi1r: MACSPI1R,
    ///0xbcc - PTP Source port identity 2 register
    pub macspi2r: MACSPI2R,
    ///0xbd0 - Log message interval register
    pub maclmir: MACLMIR,
    _reserved93: [u8; 0x2c],
    ///0xc00 - Operating mode Register
    pub mtlomr: MTLOMR,
    _reserved94: [u8; 0x1c],
    ///0xc20 - Interrupt status Register
    pub mtlisr: MTLISR,
    _reserved95: [u8; 0xdc],
    ///0xd00 - Tx queue operating mode Register
    pub mtltxqomr: MTLTXQOMR,
    ///0xd04 - Tx queue underflow register
    pub mtltxqur: MTLTXQUR,
    ///0xd08 - Tx queue debug register
    pub mtltxqdr: MTLTXQDR,
    _reserved98: [u8; 0x20],
    ///0xd2c - Queue interrupt control status Register
    pub mtlqicsr: MTLQICSR,
    ///0xd30 - Rx queue operating mode register
    pub mtlrxqomr: MTLRXQOMR,
    ///0xd34 - Rx queue missed packet and overflow counter register
    pub mtlrxqmpocr: MTLRXQMPOCR,
    ///0xd38 - Rx queue debug register
    pub mtlrxqdr: MTLRXQDR,
    _reserved102: [u8; 0x02c4],
    ///0x1000 - DMA mode register
    pub dmamr: DMAMR,
    ///0x1004 - System bus mode register
    pub dmasbmr: DMASBMR,
    ///0x1008 - Interrupt status register
    pub dmaisr: DMAISR,
    ///0x100c - Debug status register
    pub dmadsr: DMADSR,
    _reserved106: [u8; 0xf0],
    ///0x1100 - Channel control register
    pub dmaccr: DMACCR,
    ///0x1104 - Channel transmit control register
    pub dmactxcr: DMACTXCR,
    ///0x1108 - Channel receive control register
    pub dmacrxcr: DMACRXCR,
    _reserved109: [u8; 0x08],
    ///0x1114 - Channel Tx descriptor list address register
    pub dmactxdlar: DMACTXDLAR,
    _reserved110: [u8; 0x04],
    ///0x111c - Channel Rx descriptor list address register
    pub dmacrxdlar: DMACRXDLAR,
    ///0x1120 - Channel Tx descriptor tail pointer register
    pub dmactxdtpr: DMACTXDTPR,
    _reserved112: [u8; 0x04],
    ///0x1128 - Channel Rx descriptor tail pointer register
    pub dmacrxdtpr: DMACRXDTPR,
    ///0x112c - Channel Tx descriptor ring length register
    pub dmactxrlr: DMACTXRLR,
    ///0x1130 - Channel Rx descriptor ring length register
    pub dmacrxrlr: DMACRXRLR,
    ///0x1134 - Channel interrupt enable register
    pub dmacier: DMACIER,
    ///0x1138 - Channel Rx interrupt watchdog timer register
    pub dmacrxiwtr: DMACRXIWTR,
    _reserved117: [u8; 0x08],
    ///0x1144 - Channel current application transmit descriptor register
    pub dmaccatxdr: DMACCATXDR,
    _reserved118: [u8; 0x04],
    ///0x114c - Channel current application receive descriptor register
    pub dmaccarxdr: DMACCARXDR,
    _reserved119: [u8; 0x04],
    ///0x1154 - Channel current application transmit buffer register
    pub dmaccatxbr: DMACCATXBR,
    _reserved120: [u8; 0x04],
    ///0x115c - Channel current application receive buffer register
    pub dmaccarxbr: DMACCARXBR,
    ///0x1160 - Channel status register
    pub dmacsr: DMACSR,
    _reserved122: [u8; 0x08],
    ///0x116c - Channel missed frame count register
    pub dmacmfcr: DMACMFCR,
}
impl RegisterBlock {
    ///0xb70 - PPS control register
    #[inline(always)]
    pub const fn macppscr_alternate(&self) -> &MACPPSCR_ALTERNATE {
        unsafe { &*(self as *const Self).cast::<u8>().add(2928usize).cast() }
    }
    ///0xb70 - PPS control register
    #[inline(always)]
    pub const fn macppscr(&self) -> &MACPPSCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(2928usize).cast() }
    }
}
///MACCR (rw) register accessor: an alias for `Reg<MACCR_SPEC>`
pub type MACCR = crate::Reg<maccr::MACCR_SPEC>;
///Operating mode configuration register
pub mod maccr;
///MACECR (rw) register accessor: an alias for `Reg<MACECR_SPEC>`
pub type MACECR = crate::Reg<macecr::MACECR_SPEC>;
///Extended operating mode configuration register
pub mod macecr;
///MACPFR (rw) register accessor: an alias for `Reg<MACPFR_SPEC>`
pub type MACPFR = crate::Reg<macpfr::MACPFR_SPEC>;
///Packet filtering control register
pub mod macpfr;
///MACWTR (rw) register accessor: an alias for `Reg<MACWTR_SPEC>`
pub type MACWTR = crate::Reg<macwtr::MACWTR_SPEC>;
///Watchdog timeout register
pub mod macwtr;
///MACHT0R (rw) register accessor: an alias for `Reg<MACHT0R_SPEC>`
pub type MACHT0R = crate::Reg<macht0r::MACHT0R_SPEC>;
///Hash Table 0 register
pub mod macht0r;
///MACHT1R (rw) register accessor: an alias for `Reg<MACHT1R_SPEC>`
pub type MACHT1R = crate::Reg<macht1r::MACHT1R_SPEC>;
///Hash Table 1 register
pub mod macht1r;
///MACVTR (rw) register accessor: an alias for `Reg<MACVTR_SPEC>`
pub type MACVTR = crate::Reg<macvtr::MACVTR_SPEC>;
///VLAN tag register
pub mod macvtr;
///MACVHTR (rw) register accessor: an alias for `Reg<MACVHTR_SPEC>`
pub type MACVHTR = crate::Reg<macvhtr::MACVHTR_SPEC>;
///VLAN Hash table register
pub mod macvhtr;
///MACVIR (rw) register accessor: an alias for `Reg<MACVIR_SPEC>`
pub type MACVIR = crate::Reg<macvir::MACVIR_SPEC>;
///VLAN inclusion register
pub mod macvir;
///MACIVIR (rw) register accessor: an alias for `Reg<MACIVIR_SPEC>`
pub type MACIVIR = crate::Reg<macivir::MACIVIR_SPEC>;
///Inner VLAN inclusion register
pub mod macivir;
///MACQTXFCR (rw) register accessor: an alias for `Reg<MACQTXFCR_SPEC>`
pub type MACQTXFCR = crate::Reg<macqtxfcr::MACQTXFCR_SPEC>;
///Tx Queue flow control register
pub mod macqtxfcr;
///MACRXFCR (rw) register accessor: an alias for `Reg<MACRXFCR_SPEC>`
pub type MACRXFCR = crate::Reg<macrxfcr::MACRXFCR_SPEC>;
///Rx flow control register
pub mod macrxfcr;
///MACISR (rw) register accessor: an alias for `Reg<MACISR_SPEC>`
pub type MACISR = crate::Reg<macisr::MACISR_SPEC>;
///Interrupt status register
pub mod macisr;
///MACIER (rw) register accessor: an alias for `Reg<MACIER_SPEC>`
pub type MACIER = crate::Reg<macier::MACIER_SPEC>;
///Interrupt enable register
pub mod macier;
///MACRXTXSR (rw) register accessor: an alias for `Reg<MACRXTXSR_SPEC>`
pub type MACRXTXSR = crate::Reg<macrxtxsr::MACRXTXSR_SPEC>;
///Rx Tx status register
pub mod macrxtxsr;
///MACPCSR (rw) register accessor: an alias for `Reg<MACPCSR_SPEC>`
pub type MACPCSR = crate::Reg<macpcsr::MACPCSR_SPEC>;
///PMT control status register
pub mod macpcsr;
///MACRWKPFR (rw) register accessor: an alias for `Reg<MACRWKPFR_SPEC>`
pub type MACRWKPFR = crate::Reg<macrwkpfr::MACRWKPFR_SPEC>;
///Remote wakeup packet filter register
pub mod macrwkpfr;
///MACLCSR (rw) register accessor: an alias for `Reg<MACLCSR_SPEC>`
pub type MACLCSR = crate::Reg<maclcsr::MACLCSR_SPEC>;
///LPI control and status register
pub mod maclcsr;
///MACLTCR (rw) register accessor: an alias for `Reg<MACLTCR_SPEC>`
pub type MACLTCR = crate::Reg<macltcr::MACLTCR_SPEC>;
///LPI timers control register
pub mod macltcr;
///MACLETR (rw) register accessor: an alias for `Reg<MACLETR_SPEC>`
pub type MACLETR = crate::Reg<macletr::MACLETR_SPEC>;
///LPI entry timer register
pub mod macletr;
///MAC1USTCR (rw) register accessor: an alias for `Reg<MAC1USTCR_SPEC>`
pub type MAC1USTCR = crate::Reg<mac1ustcr::MAC1USTCR_SPEC>;
///One-microsecond-tick counter register
pub mod mac1ustcr;
///MACVR (r) register accessor: an alias for `Reg<MACVR_SPEC>`
pub type MACVR = crate::Reg<macvr::MACVR_SPEC>;
///Version register
pub mod macvr;
///MACDR (r) register accessor: an alias for `Reg<MACDR_SPEC>`
pub type MACDR = crate::Reg<macdr::MACDR_SPEC>;
///Debug register
pub mod macdr;
///MACHWF0R (r) register accessor: an alias for `Reg<MACHWF0R_SPEC>`
pub type MACHWF0R = crate::Reg<machwf0r::MACHWF0R_SPEC>;
///HW feature 0 register
pub mod machwf0r;
///MACHWF1R (r) register accessor: an alias for `Reg<MACHWF1R_SPEC>`
pub type MACHWF1R = crate::Reg<machwf1r::MACHWF1R_SPEC>;
///HW feature 1 register
pub mod machwf1r;
///MACHWF2R (r) register accessor: an alias for `Reg<MACHWF2R_SPEC>`
pub type MACHWF2R = crate::Reg<machwf2r::MACHWF2R_SPEC>;
///HW feature 2 register
pub mod machwf2r;
///MACHWF3R (r) register accessor: an alias for `Reg<MACHWF3R_SPEC>`
pub type MACHWF3R = crate::Reg<machwf3r::MACHWF3R_SPEC>;
///HW feature 3 register
pub mod machwf3r;
///MACMDIOAR (rw) register accessor: an alias for `Reg<MACMDIOAR_SPEC>`
pub type MACMDIOAR = crate::Reg<macmdioar::MACMDIOAR_SPEC>;
///MDIO address register
pub mod macmdioar;
///MACMDIODR (rw) register accessor: an alias for `Reg<MACMDIODR_SPEC>`
pub type MACMDIODR = crate::Reg<macmdiodr::MACMDIODR_SPEC>;
///MDIO data register
pub mod macmdiodr;
///MACARPAR (rw) register accessor: an alias for `Reg<MACARPAR_SPEC>`
pub type MACARPAR = crate::Reg<macarpar::MACARPAR_SPEC>;
///ARP address register
pub mod macarpar;
///MACCSRSWCR (rw) register accessor: an alias for `Reg<MACCSRSWCR_SPEC>`
pub type MACCSRSWCR = crate::Reg<maccsrswcr::MACCSRSWCR_SPEC>;
///CSR software control register
pub mod maccsrswcr;
///MACA0HR (rw) register accessor: an alias for `Reg<MACA0HR_SPEC>`
pub type MACA0HR = crate::Reg<maca0hr::MACA0HR_SPEC>;
///MAC Address 0 high register
pub mod maca0hr;
///MACA0LR (rw) register accessor: an alias for `Reg<MACA0LR_SPEC>`
pub type MACA0LR = crate::Reg<maca0lr::MACA0LR_SPEC>;
///MAC Address 0 low register
pub mod maca0lr;
///MACA1HR (rw) register accessor: an alias for `Reg<MACA1HR_SPEC>`
pub type MACA1HR = crate::Reg<maca1hr::MACA1HR_SPEC>;
///MAC Address 1 high register
pub mod maca1hr;
///MACA1LR (rw) register accessor: an alias for `Reg<MACA1LR_SPEC>`
pub type MACA1LR = crate::Reg<maca1lr::MACA1LR_SPEC>;
///MAC Address 1 low register
pub mod maca1lr;
///MACA2HR (rw) register accessor: an alias for `Reg<MACA2HR_SPEC>`
pub type MACA2HR = crate::Reg<maca2hr::MACA2HR_SPEC>;
///MAC Address 2 high register
pub mod maca2hr;
///MACA2LR (rw) register accessor: an alias for `Reg<MACA2LR_SPEC>`
pub type MACA2LR = crate::Reg<maca2lr::MACA2LR_SPEC>;
///MAC Address 2 low register
pub mod maca2lr;
///MACA3HR (rw) register accessor: an alias for `Reg<MACA3HR_SPEC>`
pub type MACA3HR = crate::Reg<maca3hr::MACA3HR_SPEC>;
///MAC Address 3 high register
pub mod maca3hr;
///MACA3LR (rw) register accessor: an alias for `Reg<MACA3LR_SPEC>`
pub type MACA3LR = crate::Reg<maca3lr::MACA3LR_SPEC>;
///MAC Address 3 low register
pub mod maca3lr;
///MMC_CONTROL (rw) register accessor: an alias for `Reg<MMC_CONTROL_SPEC>`
pub type MMC_CONTROL = crate::Reg<mmc_control::MMC_CONTROL_SPEC>;
///MMC control register
pub mod mmc_control;
///MMC_RX_INTERRUPT (rw) register accessor: an alias for `Reg<MMC_RX_INTERRUPT_SPEC>`
pub type MMC_RX_INTERRUPT = crate::Reg<mmc_rx_interrupt::MMC_RX_INTERRUPT_SPEC>;
///MMC Rx interrupt register
pub mod mmc_rx_interrupt;
///MMC_TX_INTERRUPT (rw) register accessor: an alias for `Reg<MMC_TX_INTERRUPT_SPEC>`
pub type MMC_TX_INTERRUPT = crate::Reg<mmc_tx_interrupt::MMC_TX_INTERRUPT_SPEC>;
///MMC Tx interrupt register
pub mod mmc_tx_interrupt;
///MMC_RX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_RX_INTERRUPT_MASK_SPEC>`
pub type MMC_RX_INTERRUPT_MASK = crate::Reg<mmc_rx_interrupt_mask::MMC_RX_INTERRUPT_MASK_SPEC>;
///MMC Rx interrupt mask register
pub mod mmc_rx_interrupt_mask;
///MMC_TX_INTERRUPT_MASK (rw) register accessor: an alias for `Reg<MMC_TX_INTERRUPT_MASK_SPEC>`
pub type MMC_TX_INTERRUPT_MASK = crate::Reg<mmc_tx_interrupt_mask::MMC_TX_INTERRUPT_MASK_SPEC>;
///MMC Tx interrupt mask register
pub mod mmc_tx_interrupt_mask;
///TX_SINGLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>`
pub type TX_SINGLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_single_collision_good_packets::TX_SINGLE_COLLISION_GOOD_PACKETS_SPEC>;
///Tx single collision good packets register
pub mod tx_single_collision_good_packets;
///TX_MULTIPLE_COLLISION_GOOD_PACKETS (r) register accessor: an alias for `Reg<TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>`
pub type TX_MULTIPLE_COLLISION_GOOD_PACKETS =
    crate::Reg<tx_multiple_collision_good_packets::TX_MULTIPLE_COLLISION_GOOD_PACKETS_SPEC>;
///Tx multiple collision good packets register
pub mod tx_multiple_collision_good_packets;
///TX_PACKET_COUNT_GOOD (r) register accessor: an alias for `Reg<TX_PACKET_COUNT_GOOD_SPEC>`
pub type TX_PACKET_COUNT_GOOD = crate::Reg<tx_packet_count_good::TX_PACKET_COUNT_GOOD_SPEC>;
///Tx packet count good register
pub mod tx_packet_count_good;
///RX_CRC_ERROR_PACKETS (r) register accessor: an alias for `Reg<RX_CRC_ERROR_PACKETS_SPEC>`
pub type RX_CRC_ERROR_PACKETS = crate::Reg<rx_crc_error_packets::RX_CRC_ERROR_PACKETS_SPEC>;
///Rx CRC error packets register
pub mod rx_crc_error_packets;
///RX_ALIGNMENT_ERROR_PACKETS (r) register accessor: an alias for `Reg<RX_ALIGNMENT_ERROR_PACKETS_SPEC>`
pub type RX_ALIGNMENT_ERROR_PACKETS =
    crate::Reg<rx_alignment_error_packets::RX_ALIGNMENT_ERROR_PACKETS_SPEC>;
///Rx alignment error packets register
pub mod rx_alignment_error_packets;
///RX_UNICAST_PACKETS_GOOD (r) register accessor: an alias for `Reg<RX_UNICAST_PACKETS_GOOD_SPEC>`
pub type RX_UNICAST_PACKETS_GOOD =
    crate::Reg<rx_unicast_packets_good::RX_UNICAST_PACKETS_GOOD_SPEC>;
///Rx unicast packets good register
pub mod rx_unicast_packets_good;
///TX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<TX_LPI_USEC_CNTR_SPEC>`
pub type TX_LPI_USEC_CNTR = crate::Reg<tx_lpi_usec_cntr::TX_LPI_USEC_CNTR_SPEC>;
///Tx LPI microsecond timer register
pub mod tx_lpi_usec_cntr;
///TX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<TX_LPI_TRAN_CNTR_SPEC>`
pub type TX_LPI_TRAN_CNTR = crate::Reg<tx_lpi_tran_cntr::TX_LPI_TRAN_CNTR_SPEC>;
///Tx LPI transition counter register
pub mod tx_lpi_tran_cntr;
///RX_LPI_USEC_CNTR (r) register accessor: an alias for `Reg<RX_LPI_USEC_CNTR_SPEC>`
pub type RX_LPI_USEC_CNTR = crate::Reg<rx_lpi_usec_cntr::RX_LPI_USEC_CNTR_SPEC>;
///Rx LPI microsecond counter register
pub mod rx_lpi_usec_cntr;
///RX_LPI_TRAN_CNTR (r) register accessor: an alias for `Reg<RX_LPI_TRAN_CNTR_SPEC>`
pub type RX_LPI_TRAN_CNTR = crate::Reg<rx_lpi_tran_cntr::RX_LPI_TRAN_CNTR_SPEC>;
///Rx LPI transition counter register
pub mod rx_lpi_tran_cntr;
///MACL3L4C0R (rw) register accessor: an alias for `Reg<MACL3L4C0R_SPEC>`
pub type MACL3L4C0R = crate::Reg<macl3l4c0r::MACL3L4C0R_SPEC>;
///L3 and L4 control 0 register
pub mod macl3l4c0r;
///MACL4A0R (rw) register accessor: an alias for `Reg<MACL4A0R_SPEC>`
pub type MACL4A0R = crate::Reg<macl4a0r::MACL4A0R_SPEC>;
///Layer4 Address filter 0 register
pub mod macl4a0r;
///MACL3A00R (rw) register accessor: an alias for `Reg<MACL3A00R_SPEC>`
pub type MACL3A00R = crate::Reg<macl3a00r::MACL3A00R_SPEC>;
///Layer3 Address 0 filter 0 register
pub mod macl3a00r;
///MACL3A10R (rw) register accessor: an alias for `Reg<MACL3A10R_SPEC>`
pub type MACL3A10R = crate::Reg<macl3a10r::MACL3A10R_SPEC>;
///Layer3 Address 1 filter 0 register
pub mod macl3a10r;
///MACL3A20R (rw) register accessor: an alias for `Reg<MACL3A20R_SPEC>`
pub type MACL3A20R = crate::Reg<macl3a20r::MACL3A20R_SPEC>;
///Layer3 Address 2 filter 0 register
pub mod macl3a20r;
///MACL3A30R (rw) register accessor: an alias for `Reg<MACL3A30R_SPEC>`
pub type MACL3A30R = crate::Reg<macl3a30r::MACL3A30R_SPEC>;
///Layer3 Address 3 filter 0 register
pub mod macl3a30r;
///MACL3L4C1R (rw) register accessor: an alias for `Reg<MACL3L4C1R_SPEC>`
pub type MACL3L4C1R = crate::Reg<macl3l4c1r::MACL3L4C1R_SPEC>;
///L3 and L4 control 1 register
pub mod macl3l4c1r;
///MACL4A1R (rw) register accessor: an alias for `Reg<MACL4A1R_SPEC>`
pub type MACL4A1R = crate::Reg<macl4a1r::MACL4A1R_SPEC>;
///Layer 4 address filter 1 register
pub mod macl4a1r;
///MACL3A01R (rw) register accessor: an alias for `Reg<MACL3A01R_SPEC>`
pub type MACL3A01R = crate::Reg<macl3a01r::MACL3A01R_SPEC>;
///Layer3 address 0 filter 1 Register
pub mod macl3a01r;
///MACL3A11R (rw) register accessor: an alias for `Reg<MACL3A11R_SPEC>`
pub type MACL3A11R = crate::Reg<macl3a11r::MACL3A11R_SPEC>;
///Layer3 address 1 filter 1 register
pub mod macl3a11r;
///MACL3A21R (rw) register accessor: an alias for `Reg<MACL3A21R_SPEC>`
pub type MACL3A21R = crate::Reg<macl3a21r::MACL3A21R_SPEC>;
///Layer3 address 2 filter 1 Register
pub mod macl3a21r;
///MACL3A31R (rw) register accessor: an alias for `Reg<MACL3A31R_SPEC>`
pub type MACL3A31R = crate::Reg<macl3a31r::MACL3A31R_SPEC>;
///Layer3 address 3 filter 1 register
pub mod macl3a31r;
///MACTSCR (rw) register accessor: an alias for `Reg<MACTSCR_SPEC>`
pub type MACTSCR = crate::Reg<mactscr::MACTSCR_SPEC>;
///Timestamp control Register
pub mod mactscr;
///MACSSIR (rw) register accessor: an alias for `Reg<MACSSIR_SPEC>`
pub type MACSSIR = crate::Reg<macssir::MACSSIR_SPEC>;
///Subsecond increment register
pub mod macssir;
///MACSTSR (r) register accessor: an alias for `Reg<MACSTSR_SPEC>`
pub type MACSTSR = crate::Reg<macstsr::MACSTSR_SPEC>;
///System time seconds register
pub mod macstsr;
///MACSTNR (r) register accessor: an alias for `Reg<MACSTNR_SPEC>`
pub type MACSTNR = crate::Reg<macstnr::MACSTNR_SPEC>;
///System time nanoseconds register
pub mod macstnr;
///MACSTSUR (rw) register accessor: an alias for `Reg<MACSTSUR_SPEC>`
pub type MACSTSUR = crate::Reg<macstsur::MACSTSUR_SPEC>;
///System time seconds update register
pub mod macstsur;
///MACSTNUR (rw) register accessor: an alias for `Reg<MACSTNUR_SPEC>`
pub type MACSTNUR = crate::Reg<macstnur::MACSTNUR_SPEC>;
///System time nanoseconds update register
pub mod macstnur;
///MACTSAR (rw) register accessor: an alias for `Reg<MACTSAR_SPEC>`
pub type MACTSAR = crate::Reg<mactsar::MACTSAR_SPEC>;
///Timestamp addend register
pub mod mactsar;
///MACTSSR (rw) register accessor: an alias for `Reg<MACTSSR_SPEC>`
pub type MACTSSR = crate::Reg<mactssr::MACTSSR_SPEC>;
///Timestamp status register
pub mod mactssr;
///MACTXTSSNR (rw) register accessor: an alias for `Reg<MACTXTSSNR_SPEC>`
pub type MACTXTSSNR = crate::Reg<mactxtssnr::MACTXTSSNR_SPEC>;
///Tx timestamp status nanoseconds register
pub mod mactxtssnr;
///MACTXTSSSR (r) register accessor: an alias for `Reg<MACTXTSSSR_SPEC>`
pub type MACTXTSSSR = crate::Reg<mactxtsssr::MACTXTSSSR_SPEC>;
///Tx timestamp status seconds register
pub mod mactxtsssr;
///MACACR (rw) register accessor: an alias for `Reg<MACACR_SPEC>`
pub type MACACR = crate::Reg<macacr::MACACR_SPEC>;
///Auxiliary control register
pub mod macacr;
///MACATSNR (r) register accessor: an alias for `Reg<MACATSNR_SPEC>`
pub type MACATSNR = crate::Reg<macatsnr::MACATSNR_SPEC>;
///Auxiliary timestamp nanoseconds register
pub mod macatsnr;
///MACATSSR (r) register accessor: an alias for `Reg<MACATSSR_SPEC>`
pub type MACATSSR = crate::Reg<macatssr::MACATSSR_SPEC>;
///Auxiliary timestamp seconds register
pub mod macatssr;
///MACTSIACR (rw) register accessor: an alias for `Reg<MACTSIACR_SPEC>`
pub type MACTSIACR = crate::Reg<mactsiacr::MACTSIACR_SPEC>;
///Timestamp Ingress asymmetric correction register
pub mod mactsiacr;
///MACTSEACR (rw) register accessor: an alias for `Reg<MACTSEACR_SPEC>`
pub type MACTSEACR = crate::Reg<mactseacr::MACTSEACR_SPEC>;
///Timestamp Egress asymmetric correction register
pub mod mactseacr;
///MACTSICNR (rw) register accessor: an alias for `Reg<MACTSICNR_SPEC>`
pub type MACTSICNR = crate::Reg<mactsicnr::MACTSICNR_SPEC>;
///Timestamp Ingress correction nanosecond register
pub mod mactsicnr;
///MACTSECNR (rw) register accessor: an alias for `Reg<MACTSECNR_SPEC>`
pub type MACTSECNR = crate::Reg<mactsecnr::MACTSECNR_SPEC>;
///Timestamp Egress correction nanosecond register
pub mod mactsecnr;
///MACPPSCR (rw) register accessor: an alias for `Reg<MACPPSCR_SPEC>`
pub type MACPPSCR = crate::Reg<macppscr::MACPPSCR_SPEC>;
///PPS control register
pub mod macppscr;
///MACPPSCR_ALTERNATE (rw) register accessor: an alias for `Reg<MACPPSCR_ALTERNATE_SPEC>`
pub type MACPPSCR_ALTERNATE = crate::Reg<macppscr_alternate::MACPPSCR_ALTERNATE_SPEC>;
///PPS control register
pub mod macppscr_alternate;
///MACPPSTTSR (rw) register accessor: an alias for `Reg<MACPPSTTSR_SPEC>`
pub type MACPPSTTSR = crate::Reg<macppsttsr::MACPPSTTSR_SPEC>;
///PPS target time seconds register
pub mod macppsttsr;
///MACPPSTTNR (rw) register accessor: an alias for `Reg<MACPPSTTNR_SPEC>`
pub type MACPPSTTNR = crate::Reg<macppsttnr::MACPPSTTNR_SPEC>;
///PPS target time nanoseconds register
pub mod macppsttnr;
///MACPPSIR (rw) register accessor: an alias for `Reg<MACPPSIR_SPEC>`
pub type MACPPSIR = crate::Reg<macppsir::MACPPSIR_SPEC>;
///PPS interval register
pub mod macppsir;
///MACPPSWR (rw) register accessor: an alias for `Reg<MACPPSWR_SPEC>`
pub type MACPPSWR = crate::Reg<macppswr::MACPPSWR_SPEC>;
///PPS width register
pub mod macppswr;
///MACPOCR (rw) register accessor: an alias for `Reg<MACPOCR_SPEC>`
pub type MACPOCR = crate::Reg<macpocr::MACPOCR_SPEC>;
///PTP Offload control register
pub mod macpocr;
///MACSPI0R (rw) register accessor: an alias for `Reg<MACSPI0R_SPEC>`
pub type MACSPI0R = crate::Reg<macspi0r::MACSPI0R_SPEC>;
///PTP Source Port Identity 0 Register
pub mod macspi0r;
///MACSPI1R (rw) register accessor: an alias for `Reg<MACSPI1R_SPEC>`
pub type MACSPI1R = crate::Reg<macspi1r::MACSPI1R_SPEC>;
///PTP Source port identity 1 register
pub mod macspi1r;
///MACSPI2R (rw) register accessor: an alias for `Reg<MACSPI2R_SPEC>`
pub type MACSPI2R = crate::Reg<macspi2r::MACSPI2R_SPEC>;
///PTP Source port identity 2 register
pub mod macspi2r;
///MACLMIR (rw) register accessor: an alias for `Reg<MACLMIR_SPEC>`
pub type MACLMIR = crate::Reg<maclmir::MACLMIR_SPEC>;
///Log message interval register
pub mod maclmir;
///MTLOMR (rw) register accessor: an alias for `Reg<MTLOMR_SPEC>`
pub type MTLOMR = crate::Reg<mtlomr::MTLOMR_SPEC>;
///Operating mode Register
pub mod mtlomr;
///MTLISR (r) register accessor: an alias for `Reg<MTLISR_SPEC>`
pub type MTLISR = crate::Reg<mtlisr::MTLISR_SPEC>;
///Interrupt status Register
pub mod mtlisr;
///MTLTXQOMR (rw) register accessor: an alias for `Reg<MTLTXQOMR_SPEC>`
pub type MTLTXQOMR = crate::Reg<mtltxqomr::MTLTXQOMR_SPEC>;
///Tx queue operating mode Register
pub mod mtltxqomr;
///MTLTXQUR (rw) register accessor: an alias for `Reg<MTLTXQUR_SPEC>`
pub type MTLTXQUR = crate::Reg<mtltxqur::MTLTXQUR_SPEC>;
///Tx queue underflow register
pub mod mtltxqur;
///MTLTXQDR (r) register accessor: an alias for `Reg<MTLTXQDR_SPEC>`
pub type MTLTXQDR = crate::Reg<mtltxqdr::MTLTXQDR_SPEC>;
///Tx queue debug register
pub mod mtltxqdr;
///MTLQICSR (rw) register accessor: an alias for `Reg<MTLQICSR_SPEC>`
pub type MTLQICSR = crate::Reg<mtlqicsr::MTLQICSR_SPEC>;
///Queue interrupt control status Register
pub mod mtlqicsr;
///MTLRXQOMR (rw) register accessor: an alias for `Reg<MTLRXQOMR_SPEC>`
pub type MTLRXQOMR = crate::Reg<mtlrxqomr::MTLRXQOMR_SPEC>;
///Rx queue operating mode register
pub mod mtlrxqomr;
///MTLRXQMPOCR (rw) register accessor: an alias for `Reg<MTLRXQMPOCR_SPEC>`
pub type MTLRXQMPOCR = crate::Reg<mtlrxqmpocr::MTLRXQMPOCR_SPEC>;
///Rx queue missed packet and overflow counter register
pub mod mtlrxqmpocr;
///MTLRXQDR (r) register accessor: an alias for `Reg<MTLRXQDR_SPEC>`
pub type MTLRXQDR = crate::Reg<mtlrxqdr::MTLRXQDR_SPEC>;
///Rx queue debug register
pub mod mtlrxqdr;
///DMAMR (rw) register accessor: an alias for `Reg<DMAMR_SPEC>`
pub type DMAMR = crate::Reg<dmamr::DMAMR_SPEC>;
///DMA mode register
pub mod dmamr;
///DMASBMR (rw) register accessor: an alias for `Reg<DMASBMR_SPEC>`
pub type DMASBMR = crate::Reg<dmasbmr::DMASBMR_SPEC>;
///System bus mode register
pub mod dmasbmr;
///DMAISR (r) register accessor: an alias for `Reg<DMAISR_SPEC>`
pub type DMAISR = crate::Reg<dmaisr::DMAISR_SPEC>;
///Interrupt status register
pub mod dmaisr;
///DMADSR (r) register accessor: an alias for `Reg<DMADSR_SPEC>`
pub type DMADSR = crate::Reg<dmadsr::DMADSR_SPEC>;
///Debug status register
pub mod dmadsr;
///DMACCR (rw) register accessor: an alias for `Reg<DMACCR_SPEC>`
pub type DMACCR = crate::Reg<dmaccr::DMACCR_SPEC>;
///Channel control register
pub mod dmaccr;
///DMACTXCR (rw) register accessor: an alias for `Reg<DMACTXCR_SPEC>`
pub type DMACTXCR = crate::Reg<dmactxcr::DMACTXCR_SPEC>;
///Channel transmit control register
pub mod dmactxcr;
///DMACRXCR (rw) register accessor: an alias for `Reg<DMACRXCR_SPEC>`
pub type DMACRXCR = crate::Reg<dmacrxcr::DMACRXCR_SPEC>;
///Channel receive control register
pub mod dmacrxcr;
///DMACTXDLAR (rw) register accessor: an alias for `Reg<DMACTXDLAR_SPEC>`
pub type DMACTXDLAR = crate::Reg<dmactxdlar::DMACTXDLAR_SPEC>;
///Channel Tx descriptor list address register
pub mod dmactxdlar;
///DMACRXDLAR (rw) register accessor: an alias for `Reg<DMACRXDLAR_SPEC>`
pub type DMACRXDLAR = crate::Reg<dmacrxdlar::DMACRXDLAR_SPEC>;
///Channel Rx descriptor list address register
pub mod dmacrxdlar;
///DMACTXDTPR (rw) register accessor: an alias for `Reg<DMACTXDTPR_SPEC>`
pub type DMACTXDTPR = crate::Reg<dmactxdtpr::DMACTXDTPR_SPEC>;
///Channel Tx descriptor tail pointer register
pub mod dmactxdtpr;
///DMACRXDTPR (rw) register accessor: an alias for `Reg<DMACRXDTPR_SPEC>`
pub type DMACRXDTPR = crate::Reg<dmacrxdtpr::DMACRXDTPR_SPEC>;
///Channel Rx descriptor tail pointer register
pub mod dmacrxdtpr;
///DMACTXRLR (rw) register accessor: an alias for `Reg<DMACTXRLR_SPEC>`
pub type DMACTXRLR = crate::Reg<dmactxrlr::DMACTXRLR_SPEC>;
///Channel Tx descriptor ring length register
pub mod dmactxrlr;
///DMACRXRLR (rw) register accessor: an alias for `Reg<DMACRXRLR_SPEC>`
pub type DMACRXRLR = crate::Reg<dmacrxrlr::DMACRXRLR_SPEC>;
///Channel Rx descriptor ring length register
pub mod dmacrxrlr;
///DMACIER (rw) register accessor: an alias for `Reg<DMACIER_SPEC>`
pub type DMACIER = crate::Reg<dmacier::DMACIER_SPEC>;
///Channel interrupt enable register
pub mod dmacier;
///DMACRXIWTR (rw) register accessor: an alias for `Reg<DMACRXIWTR_SPEC>`
pub type DMACRXIWTR = crate::Reg<dmacrxiwtr::DMACRXIWTR_SPEC>;
///Channel Rx interrupt watchdog timer register
pub mod dmacrxiwtr;
///DMACCATXDR (r) register accessor: an alias for `Reg<DMACCATXDR_SPEC>`
pub type DMACCATXDR = crate::Reg<dmaccatxdr::DMACCATXDR_SPEC>;
///Channel current application transmit descriptor register
pub mod dmaccatxdr;
///DMACCARXDR (r) register accessor: an alias for `Reg<DMACCARXDR_SPEC>`
pub type DMACCARXDR = crate::Reg<dmaccarxdr::DMACCARXDR_SPEC>;
///Channel current application receive descriptor register
pub mod dmaccarxdr;
///DMACCATXBR (r) register accessor: an alias for `Reg<DMACCATXBR_SPEC>`
pub type DMACCATXBR = crate::Reg<dmaccatxbr::DMACCATXBR_SPEC>;
///Channel current application transmit buffer register
pub mod dmaccatxbr;
///DMACCARXBR (r) register accessor: an alias for `Reg<DMACCARXBR_SPEC>`
pub type DMACCARXBR = crate::Reg<dmaccarxbr::DMACCARXBR_SPEC>;
///Channel current application receive buffer register
pub mod dmaccarxbr;
///DMACSR (rw) register accessor: an alias for `Reg<DMACSR_SPEC>`
pub type DMACSR = crate::Reg<dmacsr::DMACSR_SPEC>;
///Channel status register
pub mod dmacsr;
///DMACMFCR (rw) register accessor: an alias for `Reg<DMACMFCR_SPEC>`
pub type DMACMFCR = crate::Reg<dmacmfcr::DMACMFCR_SPEC>;
///Channel missed frame count register
pub mod dmacmfcr;
