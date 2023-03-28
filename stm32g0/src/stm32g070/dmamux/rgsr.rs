///Register `RGSR` reader
pub struct R(crate::R<RGSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OF` reader - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
pub type OF_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Trigger event overrun flag The flag is set when a trigger event occurs on DMA request generator channel x, while the DMA request generator counter value is lower than GNBREQ. The flag is cleared by writing 1 to the corresponding COFx bit in DMAMUX_RGCFR register.
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x0f) as u8)
    }
}
///DMAMux - DMA request generator status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgsr](index.html) module
pub struct RGSR_SPEC;
impl crate::RegisterSpec for RGSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rgsr::R](R) reader structure
impl crate::Readable for RGSR_SPEC {
    type Reader = R;
}
///`reset()` method sets RGSR to value 0
impl crate::Resettable for RGSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
