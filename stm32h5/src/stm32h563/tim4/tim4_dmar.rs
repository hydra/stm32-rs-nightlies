///Register `TIM4_DMAR` reader
pub struct R(crate::R<TIM4_DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_DMAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM4_DMAR` writer
pub struct W(crate::W<TIM4_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TIM4_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAB` reader - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
pub type DMAB_R = crate::FieldReader<u32, u32>;
///Field `DMAB` writer - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
pub type DMAB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM4_DMAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR).
    #[inline(always)]
    #[must_use]
    pub fn dmab(&mut self) -> DMAB_W<0> {
        DMAB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM4 DMA address for full transfer
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim4_dmar](index.html) module
pub struct TIM4_DMAR_SPEC;
impl crate::RegisterSpec for TIM4_DMAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim4_dmar::R](R) reader structure
impl crate::Readable for TIM4_DMAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim4_dmar::W](W) writer structure
impl crate::Writable for TIM4_DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM4_DMAR to value 0
impl crate::Resettable for TIM4_DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
