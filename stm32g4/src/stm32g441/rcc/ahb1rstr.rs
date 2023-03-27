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
///Field `DMA1RST` reader - DMA1 reset
pub type DMA1RST_R = crate::BitReader<DMA1RST_A>;
///DMA1 reset
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
///Field `DMA1RST` writer - DMA1 reset
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA1RST_A, O>;
impl<'a, const O: u8> DMA1RST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::Reset)
    }
}
///Field `DMA2RST` reader - DMA2 reset
pub use DMA1RST_R as DMA2RST_R;
///Field `DMAMUX1RST` reader - DMAMUXRST
pub use DMA1RST_R as DMAMUX1RST_R;
///Field `CORDICRST` reader - CORDIC reset
pub use DMA1RST_R as CORDICRST_R;
///Field `FMACRST` reader - FMAC reset
pub use DMA1RST_R as FMACRST_R;
///Field `FLASHRST` reader - Flash memory interface reset
pub use DMA1RST_R as FLASHRST_R;
///Field `CRCRST` reader - CRC reset
pub use DMA1RST_R as CRCRST_R;
///Field `DMA2RST` writer - DMA2 reset
pub use DMA1RST_W as DMA2RST_W;
///Field `DMAMUX1RST` writer - DMAMUXRST
pub use DMA1RST_W as DMAMUX1RST_W;
///Field `CORDICRST` writer - CORDIC reset
pub use DMA1RST_W as CORDICRST_W;
///Field `FMACRST` writer - FMAC reset
pub use DMA1RST_W as FMACRST_W;
///Field `FLASHRST` writer - Flash memory interface reset
pub use DMA1RST_W as FLASHRST_W;
///Field `CRCRST` writer - CRC reset
pub use DMA1RST_W as CRCRST_W;
impl R {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUXRST
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CORDIC reset
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMAC reset
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    ///Bit 2 - DMAMUXRST
    #[inline(always)]
    #[must_use]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W<2> {
        DMAMUX1RST_W::new(self)
    }
    ///Bit 3 - CORDIC reset
    #[inline(always)]
    #[must_use]
    pub fn cordicrst(&mut self) -> CORDICRST_W<3> {
        CORDICRST_W::new(self)
    }
    ///Bit 4 - FMAC reset
    #[inline(always)]
    #[must_use]
    pub fn fmacrst(&mut self) -> FMACRST_W<4> {
        FMACRST_W::new(self)
    }
    ///Bit 8 - Flash memory interface reset
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<8> {
        FLASHRST_W::new(self)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB1 peripheral reset register
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
