///Register `ETH_MACATSNR` reader
pub struct R(crate::R<ETH_MACATSNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACATSNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACATSNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACATSNR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AUXTSLO` reader - AUXTSLO
pub type AUXTSLO_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:30 - AUXTSLO
    #[inline(always)]
    pub fn auxtslo(&self) -> AUXTSLO_R {
        AUXTSLO_R::new(self.bits & 0x7fff_ffff)
    }
}
///The Auxiliary Timestamp Nanoseconds register, along with ETH_MACATSSR, gives the 64-bit timestamp stored as auxiliary snapshot. These two registers form the read port of a 64-bit wide FIFO with a depth of 4 words. You can store multiple snapshots in this FIFO. Bits\[29:25\]
///in ETH_MACTSSR indicate the fill-level of the FIFO. The top of the FIFO is removed only when the last byte of MAC Register 91 (Auxiliary Timestamp - Seconds Register) is read. In the little-endian mode, this means when Bits\[31:24\]
///are read and in big-endian mode, Bits\[7:0\]
///are read.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macatsnr](index.html) module
pub struct ETH_MACATSNR_SPEC;
impl crate::RegisterSpec for ETH_MACATSNR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macatsnr::R](R) reader structure
impl crate::Readable for ETH_MACATSNR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MACATSNR to value 0
impl crate::Resettable for ETH_MACATSNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
