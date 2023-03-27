///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x100 - DFSDM channel configuration cluster
    pub ch: [CH; 8],
    ///0x100..0x300 - DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
    pub flt: [FLT; 4],
}
///DFSDM channel configuration cluster
pub use self::ch::CH;
///Cluster
///DFSDM channel configuration cluster
pub mod ch;
///DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
pub use self::flt::FLT;
///Cluster
///DFSDM cluster: CR1, CR2, ISR, ICR, JCHGR, FCR, JDATAR, RDATAR, AWHTR, AWLTR, AWSR, AWCFR, EXMAX, EXMIN, CNVTIMR registers
pub mod flt;
