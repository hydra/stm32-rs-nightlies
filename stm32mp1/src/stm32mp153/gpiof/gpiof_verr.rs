///Register `GPIOF_VERR` reader
pub struct R(crate::R<GPIOF_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOF_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOF_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOF_VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - MINREV
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - MAJREV
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - MINREV
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MAJREV
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///GPIO version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiof_verr](index.html) module
pub struct GPIOF_VERR_SPEC;
impl crate::RegisterSpec for GPIOF_VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiof_verr::R](R) reader structure
impl crate::Readable for GPIOF_VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOF_VERR to value 0x40
impl crate::Resettable for GPIOF_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
