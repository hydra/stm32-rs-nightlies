///Register `VER` reader
pub struct R(crate::R<VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VER_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - IP minor revision number.
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - IP major revision number.
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - IP minor revision number.
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - IP major revision number.
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///SDMMC IP version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ver](index.html) module
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ver::R](R) reader structure
impl crate::Readable for VER_SPEC {
    type Reader = R;
}
///`reset()` method sets VER to value 0x10
impl crate::Resettable for VER_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
