///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - endpoint 0 register
    pub ep0r: EP0R,
    _reserved1: [u8; 0x02],
    ///0x04 - endpoint 1 register
    pub ep1r: EP1R,
    _reserved2: [u8; 0x02],
    ///0x08 - endpoint 2 register
    pub ep2r: EP2R,
    _reserved3: [u8; 0x02],
    ///0x0c - endpoint 3 register
    pub ep3r: EP3R,
    _reserved4: [u8; 0x02],
    ///0x10 - endpoint 4 register
    pub ep4r: EP4R,
    _reserved5: [u8; 0x02],
    ///0x14 - endpoint 5 register
    pub ep5r: EP5R,
    _reserved6: [u8; 0x02],
    ///0x18 - endpoint 6 register
    pub ep6r: EP6R,
    _reserved7: [u8; 0x02],
    ///0x1c - endpoint 7 register
    pub ep7r: EP7R,
    _reserved8: [u8; 0x22],
    ///0x40 - control register
    pub cntr: CNTR,
    _reserved9: [u8; 0x02],
    ///0x44 - interrupt status register
    pub istr: ISTR,
    _reserved10: [u8; 0x02],
    ///0x48 - frame number register
    pub fnr: FNR,
    _reserved11: [u8; 0x02],
    ///0x4c - device address
    pub daddr: DADDR,
    _reserved12: [u8; 0x02],
    ///0x50 - Buffer table address
    pub btable: BTABLE,
    ///0x52 - Transmission byte count 0
    pub count0_tx: COUNT0_TX,
    _reserved_14_lpmcsr: [u8; 0x02],
    ///0x56 - Reception byte count 0
    pub count0_rx: COUNT0_RX,
    ///0x58 - Battery charging detector(
    pub bcdr: BCDR,
    ///0x5a - Transmission byte count 0
    pub count1_tx: COUNT1_TX,
    ///0x5c - Reception buffer address 0
    pub addr1_rx: ADDR1_RX,
    ///0x5e - Reception byte count 0
    pub count1_rx: COUNT1_RX,
    _reserved20: [u8; 0x02],
    ///0x62 - Transmission byte count 0
    pub count2_tx: COUNT2_TX,
    ///0x64 - Reception buffer address 0
    pub addr2_rx: ADDR2_RX,
    ///0x66 - Reception byte count 0
    pub count2_rx: COUNT2_RX,
    _reserved23: [u8; 0x02],
    ///0x6a - Transmission byte count 0
    pub count3_tx: COUNT3_TX,
    ///0x6c - Reception buffer address 0
    pub addr3_rx: ADDR3_RX,
    ///0x6e - Reception byte count 0
    pub count3_rx: COUNT3_RX,
    _reserved26: [u8; 0x02],
    ///0x72 - Transmission byte count 0
    pub count4_tx: COUNT4_TX,
    ///0x74 - Reception buffer address 0
    pub addr4_rx: ADDR4_RX,
    ///0x76 - Reception byte count 0
    pub count4_rx: COUNT4_RX,
    _reserved29: [u8; 0x02],
    ///0x7a - Transmission byte count 0
    pub count5_tx: COUNT5_TX,
    ///0x7c - Reception buffer address 0
    pub addr5_rx: ADDR5_RX,
    ///0x7e - Reception byte count 0
    pub count5_rx: COUNT5_RX,
    _reserved32: [u8; 0x02],
    ///0x82 - Transmission byte count 0
    pub count6_tx: COUNT6_TX,
    ///0x84 - Reception buffer address 0
    pub addr6_rx: ADDR6_RX,
    ///0x86 - Reception byte count 0
    pub count6_rx: COUNT6_RX,
    _reserved35: [u8; 0x02],
    ///0x8a - Transmission byte count 0
    pub count7_tx: COUNT7_TX,
    ///0x8c - Reception buffer address 0
    pub addr7_rx: ADDR7_RX,
    ///0x8e - Reception byte count 0
    pub count7_rx: COUNT7_RX,
}
impl RegisterBlock {
    ///0x54 - control and status register
    #[inline(always)]
    pub const fn lpmcsr(&self) -> &LPMCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    ///0x54 - Reception buffer address 0
    #[inline(always)]
    pub const fn addr0_rx(&self) -> &ADDR0_RX {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
}
///EP0R (rw) register accessor: an alias for `Reg<EP0R_SPEC>`
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
///endpoint 0 register
pub mod ep0r;
///EP1R (rw) register accessor: an alias for `Reg<EP1R_SPEC>`
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
///endpoint 1 register
pub mod ep1r;
///EP2R (rw) register accessor: an alias for `Reg<EP2R_SPEC>`
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
///endpoint 2 register
pub mod ep2r;
///EP3R (rw) register accessor: an alias for `Reg<EP3R_SPEC>`
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
///endpoint 3 register
pub mod ep3r;
///EP4R (rw) register accessor: an alias for `Reg<EP4R_SPEC>`
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
///endpoint 4 register
pub mod ep4r;
///EP5R (rw) register accessor: an alias for `Reg<EP5R_SPEC>`
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
///endpoint 5 register
pub mod ep5r;
///EP6R (rw) register accessor: an alias for `Reg<EP6R_SPEC>`
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
///endpoint 6 register
pub mod ep6r;
///EP7R (rw) register accessor: an alias for `Reg<EP7R_SPEC>`
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
///endpoint 7 register
pub mod ep7r;
///CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
///control register
pub mod cntr;
///ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
///interrupt status register
pub mod istr;
///FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
///frame number register
pub mod fnr;
///DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
///device address
pub mod daddr;
///BTABLE (rw) register accessor: an alias for `Reg<BTABLE_SPEC>`
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
///Buffer table address
pub mod btable;
///COUNT0_TX (rw) register accessor: an alias for `Reg<COUNT0_TX_SPEC>`
pub type COUNT0_TX = crate::Reg<count0_tx::COUNT0_TX_SPEC>;
///Transmission byte count 0
pub mod count0_tx;
///COUNT1_TX (rw) register accessor: an alias for `Reg<COUNT1_TX_SPEC>`
pub type COUNT1_TX = crate::Reg<count1_tx::COUNT1_TX_SPEC>;
///Transmission byte count 0
pub mod count1_tx;
///COUNT2_TX (rw) register accessor: an alias for `Reg<COUNT2_TX_SPEC>`
pub type COUNT2_TX = crate::Reg<count2_tx::COUNT2_TX_SPEC>;
///Transmission byte count 0
pub mod count2_tx;
///COUNT3_TX (rw) register accessor: an alias for `Reg<COUNT3_TX_SPEC>`
pub type COUNT3_TX = crate::Reg<count3_tx::COUNT3_TX_SPEC>;
///Transmission byte count 0
pub mod count3_tx;
///COUNT4_TX (rw) register accessor: an alias for `Reg<COUNT4_TX_SPEC>`
pub type COUNT4_TX = crate::Reg<count4_tx::COUNT4_TX_SPEC>;
///Transmission byte count 0
pub mod count4_tx;
///COUNT5_TX (rw) register accessor: an alias for `Reg<COUNT5_TX_SPEC>`
pub type COUNT5_TX = crate::Reg<count5_tx::COUNT5_TX_SPEC>;
///Transmission byte count 0
pub mod count5_tx;
///COUNT6_TX (rw) register accessor: an alias for `Reg<COUNT6_TX_SPEC>`
pub type COUNT6_TX = crate::Reg<count6_tx::COUNT6_TX_SPEC>;
///Transmission byte count 0
pub mod count6_tx;
///COUNT7_TX (rw) register accessor: an alias for `Reg<COUNT7_TX_SPEC>`
pub type COUNT7_TX = crate::Reg<count7_tx::COUNT7_TX_SPEC>;
///Transmission byte count 0
pub mod count7_tx;
///ADDR0_RX (rw) register accessor: an alias for `Reg<ADDR0_RX_SPEC>`
pub type ADDR0_RX = crate::Reg<addr0_rx::ADDR0_RX_SPEC>;
///Reception buffer address 0
pub mod addr0_rx;
///ADDR1_RX (rw) register accessor: an alias for `Reg<ADDR1_RX_SPEC>`
pub type ADDR1_RX = crate::Reg<addr1_rx::ADDR1_RX_SPEC>;
///Reception buffer address 0
pub mod addr1_rx;
///ADDR2_RX (rw) register accessor: an alias for `Reg<ADDR2_RX_SPEC>`
pub type ADDR2_RX = crate::Reg<addr2_rx::ADDR2_RX_SPEC>;
///Reception buffer address 0
pub mod addr2_rx;
///ADDR3_RX (rw) register accessor: an alias for `Reg<ADDR3_RX_SPEC>`
pub type ADDR3_RX = crate::Reg<addr3_rx::ADDR3_RX_SPEC>;
///Reception buffer address 0
pub mod addr3_rx;
///ADDR4_RX (rw) register accessor: an alias for `Reg<ADDR4_RX_SPEC>`
pub type ADDR4_RX = crate::Reg<addr4_rx::ADDR4_RX_SPEC>;
///Reception buffer address 0
pub mod addr4_rx;
///ADDR5_RX (rw) register accessor: an alias for `Reg<ADDR5_RX_SPEC>`
pub type ADDR5_RX = crate::Reg<addr5_rx::ADDR5_RX_SPEC>;
///Reception buffer address 0
pub mod addr5_rx;
///ADDR6_RX (rw) register accessor: an alias for `Reg<ADDR6_RX_SPEC>`
pub type ADDR6_RX = crate::Reg<addr6_rx::ADDR6_RX_SPEC>;
///Reception buffer address 0
pub mod addr6_rx;
///ADDR7_RX (rw) register accessor: an alias for `Reg<ADDR7_RX_SPEC>`
pub type ADDR7_RX = crate::Reg<addr7_rx::ADDR7_RX_SPEC>;
///Reception buffer address 0
pub mod addr7_rx;
///COUNT0_RX (rw) register accessor: an alias for `Reg<COUNT0_RX_SPEC>`
pub type COUNT0_RX = crate::Reg<count0_rx::COUNT0_RX_SPEC>;
///Reception byte count 0
pub mod count0_rx;
///COUNT1_RX (rw) register accessor: an alias for `Reg<COUNT1_RX_SPEC>`
pub type COUNT1_RX = crate::Reg<count1_rx::COUNT1_RX_SPEC>;
///Reception byte count 0
pub mod count1_rx;
///COUNT2_RX (rw) register accessor: an alias for `Reg<COUNT2_RX_SPEC>`
pub type COUNT2_RX = crate::Reg<count2_rx::COUNT2_RX_SPEC>;
///Reception byte count 0
pub mod count2_rx;
///COUNT3_RX (rw) register accessor: an alias for `Reg<COUNT3_RX_SPEC>`
pub type COUNT3_RX = crate::Reg<count3_rx::COUNT3_RX_SPEC>;
///Reception byte count 0
pub mod count3_rx;
///COUNT4_RX (rw) register accessor: an alias for `Reg<COUNT4_RX_SPEC>`
pub type COUNT4_RX = crate::Reg<count4_rx::COUNT4_RX_SPEC>;
///Reception byte count 0
pub mod count4_rx;
///COUNT5_RX (rw) register accessor: an alias for `Reg<COUNT5_RX_SPEC>`
pub type COUNT5_RX = crate::Reg<count5_rx::COUNT5_RX_SPEC>;
///Reception byte count 0
pub mod count5_rx;
///COUNT6_RX (rw) register accessor: an alias for `Reg<COUNT6_RX_SPEC>`
pub type COUNT6_RX = crate::Reg<count6_rx::COUNT6_RX_SPEC>;
///Reception byte count 0
pub mod count6_rx;
///COUNT7_RX (rw) register accessor: an alias for `Reg<COUNT7_RX_SPEC>`
pub type COUNT7_RX = crate::Reg<count7_rx::COUNT7_RX_SPEC>;
///Reception byte count 0
pub mod count7_rx;
///LPMCSR (rw) register accessor: an alias for `Reg<LPMCSR_SPEC>`
pub type LPMCSR = crate::Reg<lpmcsr::LPMCSR_SPEC>;
///control and status register
pub mod lpmcsr;
///BCDR (rw) register accessor: an alias for `Reg<BCDR_SPEC>`
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
///Battery charging detector(
pub mod bcdr;
