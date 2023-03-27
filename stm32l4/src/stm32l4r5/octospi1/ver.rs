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
///Field `VER` reader - Version
pub type VER_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Version
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new((self.bits & 0xff) as u8)
    }
}
///version register
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
