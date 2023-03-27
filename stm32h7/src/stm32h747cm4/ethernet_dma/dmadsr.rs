///Register `DMADSR` reader
pub struct R(crate::R<DMADSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMADSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMADSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMADSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMADSR` writer
pub struct W(crate::W<DMADSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMADSR_SPEC>;
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
impl From<crate::W<DMADSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMADSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AXWHSTS` reader - AHB Master Write Channel
pub type AXWHSTS_R = crate::BitReader<bool>;
///Field `AXWHSTS` writer - AHB Master Write Channel
pub type AXWHSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMADSR_SPEC, bool, O>;
///Field `RPS0` reader - DMA Channel Receive Process State
pub type RPS0_R = crate::FieldReader<u8, u8>;
///Field `RPS0` writer - DMA Channel Receive Process State
pub type RPS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMADSR_SPEC, u8, u8, 4, O>;
///Field `TPS0` reader - DMA Channel Transmit Process State
pub type TPS0_R = crate::FieldReader<u8, u8>;
///Field `TPS0` writer - DMA Channel Transmit Process State
pub type TPS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMADSR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:11 - DMA Channel Receive Process State
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - AHB Master Write Channel
    #[inline(always)]
    #[must_use]
    pub fn axwhsts(&mut self) -> AXWHSTS_W<0> {
        AXWHSTS_W::new(self)
    }
    ///Bits 8:11 - DMA Channel Receive Process State
    #[inline(always)]
    #[must_use]
    pub fn rps0(&mut self) -> RPS0_W<8> {
        RPS0_W::new(self)
    }
    ///Bits 12:15 - DMA Channel Transmit Process State
    #[inline(always)]
    #[must_use]
    pub fn tps0(&mut self) -> TPS0_W<12> {
        TPS0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmadsr](index.html) module
pub struct DMADSR_SPEC;
impl crate::RegisterSpec for DMADSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmadsr::R](R) reader structure
impl crate::Readable for DMADSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmadsr::W](W) writer structure
impl crate::Writable for DMADSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMADSR to value 0
impl crate::Resettable for DMADSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
