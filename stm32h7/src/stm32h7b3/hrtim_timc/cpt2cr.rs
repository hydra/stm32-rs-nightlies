///Register `CPT2CR` reader
pub struct R(crate::R<CPT2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPT2x` reader - Timerx Capture 2 value
pub type CPT2X_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Timerx Capture 2 value
    #[inline(always)]
    pub fn cpt2x(&self) -> CPT2X_R {
        CPT2X_R::new((self.bits & 0xffff) as u16)
    }
}
///Timerx Capture 2 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt2cr](index.html) module
pub struct CPT2CR_SPEC;
impl crate::RegisterSpec for CPT2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cpt2cr::R](R) reader structure
impl crate::Readable for CPT2CR_SPEC {
    type Reader = R;
}
///`reset()` method sets CPT2CR to value 0
impl crate::Resettable for CPT2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
