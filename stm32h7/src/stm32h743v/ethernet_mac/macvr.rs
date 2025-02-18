///Register `MACVR` reader
pub struct R(crate::R<MACVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SNPSVER` reader - IP version
pub type SNPSVER_R = crate::FieldReader<u8, u8>;
///Field `USERVER` reader - ST-defined version
pub type USERVER_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - IP version
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - ST-defined version
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///Version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macvr](index.html) module
pub struct MACVR_SPEC;
impl crate::RegisterSpec for MACVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macvr::R](R) reader structure
impl crate::Readable for MACVR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACVR to value 0x3041
impl crate::Resettable for MACVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3041;
}
