///Register `DFSDM_FLT0JDATAR` reader
pub struct R(crate::R<DFSDM_FLT0JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT0JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT0JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT0JDATAR_SPEC>) -> Self {
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
///For information about available fields see [dfsdm_flt0jdatar](index.html) module
pub struct DFSDM_FLT0JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0JDATAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt0jdatar::R](R) reader structure
impl crate::Readable for DFSDM_FLT0JDATAR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_FLT0JDATAR to value 0
impl crate::Resettable for DFSDM_FLT0JDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
