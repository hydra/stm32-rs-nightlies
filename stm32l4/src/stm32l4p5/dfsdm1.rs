///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x100 - DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers
    pub ch: [CH; 8],
    ///0x100..0x300 - Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR
    pub flt: [FLT; 4],
}
///DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers
pub use self::ch::CH;
///Cluster
///DFSDM Channel cluster: contains CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR and CH?DATINR registers
pub mod ch;
///Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR
pub use self::flt::FLT;
///Cluster
///Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR
pub mod flt;
