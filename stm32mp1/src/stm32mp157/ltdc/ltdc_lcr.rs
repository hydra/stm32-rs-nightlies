///Register `LTDC_LCR` reader
pub struct R(crate::R<LTDC_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_LCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `LNBR` reader - LNBR
pub type LNBR_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - LNBR
    #[inline(always)]
    pub fn lnbr(&self) -> LNBR_R {
        LNBR_R::new((self.bits & 0xff) as u8)
    }
}
///LDTC layer count register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltdc_lcr](index.html) module
pub struct LTDC_LCR_SPEC;
impl crate::RegisterSpec for LTDC_LCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltdc_lcr::R](R) reader structure
impl crate::Readable for LTDC_LCR_SPEC {
    type Reader = R;
}
///`reset()` method sets LTDC_LCR to value 0x02
impl crate::Resettable for LTDC_LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
