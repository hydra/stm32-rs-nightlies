///Register `MACTxTSSSR` reader
pub struct R(crate::R<MACTX_TSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTX_TSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTX_TSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTX_TSSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TXTSSHI` reader - Transmit Timestamp Status High
pub type TXTSSHI_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Transmit Timestamp Status High
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
///Tx timestamp status seconds register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mactx_tsssr](index.html) module
pub struct MACTX_TSSSR_SPEC;
impl crate::RegisterSpec for MACTX_TSSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mactx_tsssr::R](R) reader structure
impl crate::Readable for MACTX_TSSSR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACTxTSSSR to value 0
impl crate::Resettable for MACTX_TSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
