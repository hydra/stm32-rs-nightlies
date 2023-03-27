///Register `WDATR` reader
pub struct R(crate::R<WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDATR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WDATA` reader - Input channel y watchdog data
pub type WDATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input channel y watchdog data
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdatr](index.html) module
pub struct WDATR_SPEC;
impl crate::RegisterSpec for WDATR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wdatr::R](R) reader structure
impl crate::Readable for WDATR_SPEC {
    type Reader = R;
}
///`reset()` method sets WDATR to value 0
impl crate::Resettable for WDATR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
