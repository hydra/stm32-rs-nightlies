///Register `OTG_HCDMAB10` reader
pub struct R(crate::R<OTG_HCDMAB10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMAB10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMAB10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMAB10_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HCDMAB` reader - HCDMAB
pub type HCDMAB_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HCDMAB
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
///OTG host channel-n DMA address buffer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hcdmab10](index.html) module
pub struct OTG_HCDMAB10_SPEC;
impl crate::RegisterSpec for OTG_HCDMAB10_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_hcdmab10::R](R) reader structure
impl crate::Readable for OTG_HCDMAB10_SPEC {
    type Reader = R;
}
///`reset()` method sets OTG_HCDMAB10 to value 0
impl crate::Resettable for OTG_HCDMAB10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
