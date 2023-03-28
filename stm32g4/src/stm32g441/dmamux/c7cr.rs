///Register `C7CR` reader
pub struct R(crate::R<C7CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C7CR` writer
pub struct W(crate::W<C7CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7CR_SPEC>;
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
impl From<crate::W<C7CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAREQ_ID` reader - Input DMA request line selected
pub type DMAREQ_ID_R = crate::FieldReader<u8, u8>;
///Field `DMAREQ_ID` writer - Input DMA request line selected
pub type DMAREQ_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7CR_SPEC, u8, u8, 7, O>;
///Field `SOIE` reader - Interrupt enable at synchronization event overrun
pub type SOIE_R = crate::BitReader<bool>;
///Field `SOIE` writer - Interrupt enable at synchronization event overrun
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7CR_SPEC, bool, O>;
///Field `EGE` reader - Event generation enable/disable
pub type EGE_R = crate::BitReader<bool>;
///Field `EGE` writer - Event generation enable/disable
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7CR_SPEC, bool, O>;
///Field `SE` reader - Synchronous operating mode enable/disable
pub type SE_R = crate::BitReader<bool>;
///Field `SE` writer - Synchronous operating mode enable/disable
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C7CR_SPEC, bool, O>;
///Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_R = crate::FieldReader<u8, u8>;
///Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7CR_SPEC, u8, u8, 2, O>;
///Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_R = crate::FieldReader<u8, u8>;
///Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7CR_SPEC, u8, u8, 5, O>;
///Field `SYNC_ID` reader - Synchronization input selected
pub type SYNC_ID_R = crate::FieldReader<u8, u8>;
///Field `SYNC_ID` writer - Synchronization input selected
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C7CR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:6 - Input DMA request line selected
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:6 - Input DMA request line selected
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMAMux - DMA request line multiplexer channel x control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c7cr](index.html) module
pub struct C7CR_SPEC;
impl crate::RegisterSpec for C7CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c7cr::R](R) reader structure
impl crate::Readable for C7CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c7cr::W](W) writer structure
impl crate::Writable for C7CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C7CR to value 0
impl crate::Resettable for C7CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
