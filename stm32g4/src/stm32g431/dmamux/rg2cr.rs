///Register `RG2CR` reader
pub struct R(crate::R<RG2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RG2CR` writer
pub struct W(crate::W<RG2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG2CR_SPEC>;
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
impl From<crate::W<RG2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG2CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SIG_ID` reader - DMA request trigger input selected
pub type SIG_ID_R = crate::FieldReader<u8, u8>;
///Field `SIG_ID` writer - DMA request trigger input selected
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG2CR_SPEC, u8, u8, 5, O>;
///Field `OIE` reader - Interrupt enable at trigger event overrun
pub type OIE_R = crate::BitReader<bool>;
///Field `OIE` writer - Interrupt enable at trigger event overrun
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG2CR_SPEC, bool, O>;
///Field `GE` reader - DMA request generator channel enable/disable
pub type GE_R = crate::BitReader<bool>;
///Field `GE` writer - DMA request generator channel enable/disable
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG2CR_SPEC, bool, O>;
///Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_R = crate::FieldReader<u8, u8>;
///Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG2CR_SPEC, u8, u8, 2, O>;
///Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
///Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG2CR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMAMux - DMA request generator channel x control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg2cr](index.html) module
pub struct RG2CR_SPEC;
impl crate::RegisterSpec for RG2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rg2cr::R](R) reader structure
impl crate::Readable for RG2CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rg2cr::W](W) writer structure
impl crate::Writable for RG2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RG2CR to value 0
impl crate::Resettable for RG2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
