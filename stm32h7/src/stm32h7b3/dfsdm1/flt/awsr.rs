///Register `AWSR` reader
pub struct R(crate::R<AWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AWLTF` reader - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\]
///bit in the DFSDM_FLTxAWCFR register.
pub type AWLTF_R = crate::FieldReader<u8, u8>;
///Field `AWHTF` reader - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\]
///bit in the DFSDM_FLTxAWCFR register.
pub type AWHTF_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Analog watchdog low threshold flag AWLTF\[y\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\[y\]
    ///bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Analog watchdog high threshold flag AWHTF\[y\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\[y\]
    ///bit in the DFSDM_FLTxAWCFR register.
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awsr](index.html) module
pub struct AWSR_SPEC;
impl crate::RegisterSpec for AWSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [awsr::R](R) reader structure
impl crate::Readable for AWSR_SPEC {
    type Reader = R;
}
///`reset()` method sets AWSR to value 0
impl crate::Resettable for AWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
