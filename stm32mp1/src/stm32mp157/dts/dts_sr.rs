///Register `DTS_SR` reader
pub struct R(crate::R<DTS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TS1_ITEF` reader - TS1_ITEF
pub type TS1_ITEF_R = crate::BitReader<bool>;
///Field `TS1_ITLF` reader - TS1_ITLF
pub type TS1_ITLF_R = crate::BitReader<bool>;
///Field `TS1_ITHF` reader - TS1_ITHF
pub type TS1_ITHF_R = crate::BitReader<bool>;
///Field `TS1_AITEF` reader - TS1_AITEF
pub type TS1_AITEF_R = crate::BitReader<bool>;
///Field `TS1_AITLF` reader - TS1_AITLF
pub type TS1_AITLF_R = crate::BitReader<bool>;
///Field `TS1_AITHF` reader - TS1_AITHF
pub type TS1_AITHF_R = crate::BitReader<bool>;
///Field `TS1_RDY` reader - TS1_RDY
pub type TS1_RDY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TS1_ITEF
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TS1_ITLF
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TS1_ITHF
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TS1_AITEF
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TS1_AITLF
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TS1_AITHF
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 15 - TS1_RDY
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///Temperature sensor status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dts_sr](index.html) module
pub struct DTS_SR_SPEC;
impl crate::RegisterSpec for DTS_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dts_sr::R](R) reader structure
impl crate::Readable for DTS_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets DTS_SR to value 0
impl crate::Resettable for DTS_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
