///Register `DFSDM_FLT2JDATAR` reader
pub struct R(crate::R<DFSDM_FLT2JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT2JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT2JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT2JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATACH` reader - JDATACH
pub type JDATACH_R = crate::FieldReader<u8, u8>;
///Field `JDATA` reader - JDATA
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:2 - JDATACH
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:31 - JDATA
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///DFSDM filter 2 data register for injected group
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt2jdatar](index.html) module
pub struct DFSDM_FLT2JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT2JDATAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt2jdatar::R](R) reader structure
impl crate::Readable for DFSDM_FLT2JDATAR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_FLT2JDATAR to value 0
impl crate::Resettable for DFSDM_FLT2JDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
