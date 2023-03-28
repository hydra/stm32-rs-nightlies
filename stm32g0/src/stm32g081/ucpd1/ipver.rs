///Register `IPVER` reader
pub struct R(crate::R<IPVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPVER_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IPVER` reader - IPVER
pub type IPVER_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - IPVER
    #[inline(always)]
    pub fn ipver(&self) -> IPVER_R {
        IPVER_R::new(self.bits)
    }
}
///UCPD IP ID register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ipver](index.html) module
pub struct IPVER_SPEC;
impl crate::RegisterSpec for IPVER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ipver::R](R) reader structure
impl crate::Readable for IPVER_SPEC {
    type Reader = R;
}
///`reset()` method sets IPVER to value 0x10
impl crate::Resettable for IPVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
