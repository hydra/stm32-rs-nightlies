///Register `VLCCR` reader
pub struct R(crate::R<VLCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HLINE` reader - HLINE
pub type HLINE_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:14 - HLINE
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
///DSI Host video line current configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vlccr](index.html) module
pub struct VLCCR_SPEC;
impl crate::RegisterSpec for VLCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vlccr::R](R) reader structure
impl crate::Readable for VLCCR_SPEC {
    type Reader = R;
}
///`reset()` method sets VLCCR to value 0
impl crate::Resettable for VLCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
