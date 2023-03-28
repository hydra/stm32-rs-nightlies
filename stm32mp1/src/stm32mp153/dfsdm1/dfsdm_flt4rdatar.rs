///Register `DFSDM_FLT4RDATAR` reader
pub struct R(crate::R<DFSDM_FLT4RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT4RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT4RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT4RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDATACH` reader - RDATACH
pub type RDATACH_R = crate::FieldReader<u8, u8>;
///Field `RPEND` reader - RPEND
pub type RPEND_R = crate::BitReader<bool>;
///Field `RDATA` reader - RDATA
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:2 - RDATACH
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - RPEND
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:31 - RDATA
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///DFSDM filter 4 data register for the regular channel
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt4rdatar](index.html) module
pub struct DFSDM_FLT4RDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT4RDATAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt4rdatar::R](R) reader structure
impl crate::Readable for DFSDM_FLT4RDATAR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_FLT4RDATAR to value 0
impl crate::Resettable for DFSDM_FLT4RDATAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
