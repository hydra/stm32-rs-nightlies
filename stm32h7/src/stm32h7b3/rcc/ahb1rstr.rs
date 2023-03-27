///Register `AHB1RSTR` reader
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1RSTR` writer
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1RST` reader - DMA1 and DMAMUX1 blocks reset Set and reset by software.
pub type DMA1RST_R = crate::BitReader<DMA1RST_A>;
///DMA1 and DMAMUX1 blocks reset Set and reset by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1RST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA1RST_A> {
        match self.bits {
            true => Some(DMA1RST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST_A::Reset
    }
}
///Field `DMA1RST` writer - DMA1 and DMAMUX1 blocks reset Set and reset by software.
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA1RST_A, O>;
impl<'a, const O: u8> DMA1RST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::Reset)
    }
}
///Field `DMA2RST` reader - DMA2 and DMAMUX2 blocks reset Set and reset by software.
pub use DMA1RST_R as DMA2RST_R;
///Field `ADC12RST` reader - ADC1 and 2 blocks reset Set and reset by software.
pub use DMA1RST_R as ADC12RST_R;
///Field `CRCRST` reader - CRC block reset Set and reset by software.
pub use DMA1RST_R as CRCRST_R;
///Field `USB1OTGRST` reader - USB1OTG block reset Set and reset by software.
pub use DMA1RST_R as USB1OTGRST_R;
///Field `DMA2RST` writer - DMA2 and DMAMUX2 blocks reset Set and reset by software.
pub use DMA1RST_W as DMA2RST_W;
///Field `ADC12RST` writer - ADC1 and 2 blocks reset Set and reset by software.
pub use DMA1RST_W as ADC12RST_W;
///Field `CRCRST` writer - CRC block reset Set and reset by software.
pub use DMA1RST_W as CRCRST_W;
///Field `USB1OTGRST` writer - USB1OTG block reset Set and reset by software.
pub use DMA1RST_W as USB1OTGRST_W;
impl R {
    ///Bit 0 - DMA1 and DMAMUX1 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 and DMAMUX2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - CRC block reset Set and reset by software.
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 25 - USB1OTG block reset Set and reset by software.
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX1 blocks reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    ///Bit 1 - DMA2 and DMAMUX2 blocks reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    ///Bit 5 - ADC1 and 2 blocks reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<5> {
        ADC12RST_W::new(self)
    }
    ///Bit 9 - CRC block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<9> {
        CRCRST_W::new(self)
    }
    ///Bit 25 - USB1OTG block reset Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W<25> {
        USB1OTGRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1rstr](index.html) module
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1rstr::R](R) reader structure
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
