///Register `DFSDM2_AWSR` reader
pub struct R(crate::R<DFSDM2_AWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM2_AWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM2_AWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM2_AWSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AWLTF` reader - Analog watchdog low threshold flag
pub type AWLTF_R = crate::FieldReader<u8, u8>;
///Field `AWHTF` reader - Analog watchdog high threshold flag
pub type AWHTF_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///analog watchdog status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm2_awsr](index.html) module
pub struct DFSDM2_AWSR_SPEC;
impl crate::RegisterSpec for DFSDM2_AWSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm2_awsr::R](R) reader structure
impl crate::Readable for DFSDM2_AWSR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM2_AWSR to value 0
impl crate::Resettable for DFSDM2_AWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
