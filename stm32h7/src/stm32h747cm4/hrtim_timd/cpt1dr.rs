///Register `CPT1DR` reader
pub struct R(crate::R<CPT1DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT1DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT1DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT1DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPT1x` reader - Timerx Capture 1 value
pub type CPT1X_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Timerx Capture 1 value
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
}
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1dr](index.html) module
pub struct CPT1DR_SPEC;
impl crate::RegisterSpec for CPT1DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cpt1dr::R](R) reader structure
impl crate::Readable for CPT1DR_SPEC {
    type Reader = R;
}
///`reset()` method sets CPT1DR to value 0
impl crate::Resettable for CPT1DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
