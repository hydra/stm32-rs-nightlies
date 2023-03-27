///Register `MACTXTSSSR` reader
pub struct R(crate::R<MACTXTSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTXTSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTXTSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTXTSSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXTSSHI` reader - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp.
pub type TXTSSHI_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Transmit Timestamp Status High This field contains the lower 32 bits of the Seconds field of Transmit packet's captured timestamp.
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
///Tx timestamp status seconds register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mactxtsssr](index.html) module
pub struct MACTXTSSSR_SPEC;
impl crate::RegisterSpec for MACTXTSSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mactxtsssr::R](R) reader structure
impl crate::Readable for MACTXTSSSR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACTXTSSSR to value 0
impl crate::Resettable for MACTXTSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
