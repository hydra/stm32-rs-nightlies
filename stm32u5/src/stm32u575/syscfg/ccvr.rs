///Register `CCVR` reader
pub struct R(crate::R<CCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCVR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NCV1` reader - NCV1
pub type NCV1_R = crate::FieldReader<u8, u8>;
///Field `PCV1` reader - PCV1
pub type PCV1_R = crate::FieldReader<u8, u8>;
///Field `NCV2` reader - NCV2
pub type NCV2_R = crate::FieldReader<u8, u8>;
///Field `PCV2` reader - PCV2
pub type PCV2_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - NCV1
    #[inline(always)]
    pub fn ncv1(&self) -> NCV1_R {
        NCV1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PCV1
    #[inline(always)]
    pub fn pcv1(&self) -> PCV1_R {
        PCV1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NCV2
    #[inline(always)]
    pub fn ncv2(&self) -> NCV2_R {
        NCV2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PCV2
    #[inline(always)]
    pub fn pcv2(&self) -> PCV2_R {
        PCV2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
///compensation cell value register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccvr](index.html) module
pub struct CCVR_SPEC;
impl crate::RegisterSpec for CCVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccvr::R](R) reader structure
impl crate::Readable for CCVR_SPEC {
    type Reader = R;
}
///`reset()` method sets CCVR to value 0
impl crate::Resettable for CCVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
