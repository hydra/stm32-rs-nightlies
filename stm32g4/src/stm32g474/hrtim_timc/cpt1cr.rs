///Register `CPT1CR` reader
pub struct R(crate::R<CPT1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPT1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPT1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPT1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CPT1x` reader - Timerx Capture 1 value
pub type CPT1X_R = crate::FieldReader<u16, u16>;
///Field `DIR` reader - Timerx Capture 1 Direction status
pub type DIR_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:15 - Timerx Capture 1 value
    #[inline(always)]
    pub fn cpt1x(&self) -> CPT1X_R {
        CPT1X_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Timerx Capture 1 Direction status
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///Timerx Capture 1 Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpt1cr](index.html) module
pub struct CPT1CR_SPEC;
impl crate::RegisterSpec for CPT1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cpt1cr::R](R) reader structure
impl crate::Readable for CPT1CR_SPEC {
    type Reader = R;
}
///`reset()` method sets CPT1CR to value 0
impl crate::Resettable for CPT1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
