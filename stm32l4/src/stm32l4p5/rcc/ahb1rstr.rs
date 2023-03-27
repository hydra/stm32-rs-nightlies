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
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DMA1
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
    pub fn variant(&self) -> DMA1RST_A {
        match self.bits {
            false => DMA1RST_A::NoEffect,
            true => DMA1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA1RST_A::NoEffect
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
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA1RST_A::NoEffect)
    }
    ///Reset DMA1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::Reset)
    }
}
///Field `DMA2RST` reader - DMA2 reset
pub type DMA2RST_R = crate::BitReader<DMA2RST_A>;
///DMA2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DMA2
    Reset = 1,
}
impl From<DMA2RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA2RST_A {
        match self.bits {
            false => DMA2RST_A::NoEffect,
            true => DMA2RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2RST_A::Reset
    }
}
///Field `DMA2RST` writer - DMA2 reset
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA2RST_A, O>;
impl<'a, const O: u8> DMA2RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA2RST_A::NoEffect)
    }
    ///Reset DMA2
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA2RST_A::Reset)
    }
}
///Field `DMAMUX1RST` reader - DMAMUXRST
pub type DMAMUX1RST_R = crate::BitReader<DMAMUX1RST_A>;
///DMAMUXRST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DMAMUX1
    Reset = 1,
}
impl From<DMAMUX1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1RST_A {
        match self.bits {
            false => DMAMUX1RST_A::NoEffect,
            true => DMAMUX1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMAMUX1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMAMUX1RST_A::Reset
    }
}
///Field `DMAMUX1RST` writer - DMAMUXRST
pub type DMAMUX1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMAMUX1RST_A, O>;
impl<'a, const O: u8> DMAMUX1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::NoEffect)
    }
    ///Reset DMAMUX1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMAMUX1RST_A::Reset)
    }
}
///Field `FLASHRST` reader - Flash memory interface reset
pub type FLASHRST_R = crate::BitReader<FLASHRST_A>;
///Flash memory interface reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset Flash memory interface
    Reset = 1,
}
impl From<FLASHRST_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHRST_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLASHRST_A {
        match self.bits {
            false => FLASHRST_A::NoEffect,
            true => FLASHRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == FLASHRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FLASHRST_A::Reset
    }
}
///Field `FLASHRST` writer - Flash memory interface reset
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, FLASHRST_A, O>;
impl<'a, const O: u8> FLASHRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FLASHRST_A::NoEffect)
    }
    ///Reset Flash memory interface
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FLASHRST_A::Reset)
    }
}
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader<CRCRST_A>;
///CRC reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset CRC
    Reset = 1,
}
impl From<CRCRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCRST_A {
        match self.bits {
            false => CRCRST_A::NoEffect,
            true => CRCRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CRCRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRCRST_A::Reset
    }
}
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, CRCRST_A, O>;
impl<'a, const O: u8> CRCRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CRCRST_A::NoEffect)
    }
    ///Reset CRC
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CRCRST_A::Reset)
    }
}
///Field `TSCRST` reader - Touch Sensing Controller reset
pub type TSCRST_R = crate::BitReader<TSCRST_A>;
///Touch Sensing Controller reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset TSC
    Reset = 1,
}
impl From<TSCRST_A> for bool {
    #[inline(always)]
    fn from(variant: TSCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSCRST_A {
        match self.bits {
            false => TSCRST_A::NoEffect,
            true => TSCRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TSCRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TSCRST_A::Reset
    }
}
///Field `TSCRST` writer - Touch Sensing Controller reset
pub type TSCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, TSCRST_A, O>;
impl<'a, const O: u8> TSCRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TSCRST_A::NoEffect)
    }
    ///Reset TSC
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TSCRST_A::Reset)
    }
}
///Field `DMA2DRST` reader - DMA2D reset
pub type DMA2DRST_R = crate::BitReader<DMA2DRST_A>;
///DMA2D reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DMA2D
    Reset = 1,
}
impl From<DMA2DRST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2DRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2DRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA2DRST_A {
        match self.bits {
            false => DMA2DRST_A::NoEffect,
            true => DMA2DRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA2DRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA2DRST_A::Reset
    }
}
///Field `DMA2DRST` writer - DMA2D reset
pub type DMA2DRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA2DRST_A, O>;
impl<'a, const O: u8> DMA2DRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA2DRST_A::NoEffect)
    }
    ///Reset DMA2D
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA2DRST_A::Reset)
    }
}
///Field `GFXMMURST` reader - GFXMMU reset
pub type GFXMMURST_R = crate::BitReader<GFXMMURST_A>;
///GFXMMU reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMURST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset GFXMMU
    Reset = 1,
}
impl From<GFXMMURST_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMURST_A) -> Self {
        variant as u8 != 0
    }
}
impl GFXMMURST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GFXMMURST_A {
        match self.bits {
            false => GFXMMURST_A::NoEffect,
            true => GFXMMURST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GFXMMURST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GFXMMURST_A::Reset
    }
}
///Field `GFXMMURST` writer - GFXMMU reset
pub type GFXMMURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, GFXMMURST_A, O>;
impl<'a, const O: u8> GFXMMURST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(GFXMMURST_A::NoEffect)
    }
    ///Reset GFXMMU
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GFXMMURST_A::Reset)
    }
}
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
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA2D reset
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - GFXMMU reset
    #[inline(always)]
    pub fn gfxmmurst(&self) -> GFXMMURST_R {
        GFXMMURST_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<16> {
        TSCRST_W::new(self)
    }
    ///Bit 17 - DMA2D reset
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<17> {
        DMA2DRST_W::new(self)
    }
    ///Bit 18 - GFXMMU reset
    #[inline(always)]
    #[must_use]
    pub fn gfxmmurst(&mut self) -> GFXMMURST_W<18> {
        GFXMMURST_W::new(self)
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
