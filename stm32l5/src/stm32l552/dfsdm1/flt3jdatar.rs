///Register `FLT3JDATAR` reader
pub struct R(crate::R<FLT3JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT3JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT3JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT3JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATACH` reader - Injected channel most recently converted
pub type JDATACH_R = crate::FieldReader<u8, u8>;
///Field `JDATA` reader - Injected group conversion data
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:2 - Injected channel most recently converted
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - Injected group conversion data
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flt3jdatar](index.html) module
pub struct FLT3JDATAR_SPEC;
impl crate::RegisterSpec for FLT3JDATAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flt3jdatar::R](R) reader structure
impl crate::Readable for FLT3JDATAR_SPEC {
    type Reader = R;
}
///`reset()` method sets FLT3JDATAR to value 0
impl crate::Resettable for FLT3JDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
