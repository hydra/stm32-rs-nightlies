///Register `TSSSR` reader
pub struct R(crate::R<TSSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SS` reader - Sub second value
pub type SS_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Sub second value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
///timestamp sub second register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsssr](index.html) module
pub struct TSSSR_SPEC;
impl crate::RegisterSpec for TSSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tsssr::R](R) reader structure
impl crate::Readable for TSSSR_SPEC {
    type Reader = R;
}
///`reset()` method sets TSSSR to value 0
impl crate::Resettable for TSSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
