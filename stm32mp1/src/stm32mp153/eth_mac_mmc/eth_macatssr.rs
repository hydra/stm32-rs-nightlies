///Register `ETH_MACATSSR` reader
pub struct R(crate::R<ETH_MACATSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACATSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACATSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACATSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AUXTSHI` reader - AUXTSHI
pub type AUXTSHI_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - AUXTSHI
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
///The Auxiliary Timestamp - Seconds register contains the lower 32 bits of the Seconds field of the auxiliary timestamp register.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macatssr](index.html) module
pub struct ETH_MACATSSR_SPEC;
impl crate::RegisterSpec for ETH_MACATSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macatssr::R](R) reader structure
impl crate::Readable for ETH_MACATSSR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MACATSSR to value 0
impl crate::Resettable for ETH_MACATSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
