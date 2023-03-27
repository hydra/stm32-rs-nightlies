///Register `RESP1R` reader
pub struct R(crate::R<RESP1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESP1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESP1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESP1R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CARDSTATUSx` reader - Card status x See .
pub type CARDSTATUSX_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Card status x See .
    #[inline(always)]
    pub fn cardstatusx(&self) -> CARDSTATUSX_R {
        CARDSTATUSX_R::new(self.bits)
    }
}
///SDMMC response 1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [resp1r](index.html) module
pub struct RESP1R_SPEC;
impl crate::RegisterSpec for RESP1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [resp1r::R](R) reader structure
impl crate::Readable for RESP1R_SPEC {
    type Reader = R;
}
///`reset()` method sets RESP1R to value 0
impl crate::Resettable for RESP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
