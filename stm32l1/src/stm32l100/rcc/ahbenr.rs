///Register `AHBENR` reader
pub struct R(crate::R<AHBENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBENR` writer
pub struct W(crate::W<AHBENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBENR_SPEC>;
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
impl From<crate::W<AHBENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOPAEN` reader - IO port A clock enable
pub type GPIOPAEN_R = crate::BitReader<GPIOPAEN_A>;
///IO port A clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOPAEN_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<GPIOPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOPAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOPAEN_A {
        match self.bits {
            false => GPIOPAEN_A::Disabled,
            true => GPIOPAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOPAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOPAEN_A::Enabled
    }
}
///Field `GPIOPAEN` writer - IO port A clock enable
pub type GPIOPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, GPIOPAEN_A, O>;
impl<'a, const O: u8> GPIOPAEN_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOPAEN_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOPAEN_A::Enabled)
    }
}
///Field `GPIOPBEN` reader - IO port B clock enable
pub use GPIOPAEN_R as GPIOPBEN_R;
///Field `GPIOPCEN` reader - IO port C clock enable
pub use GPIOPAEN_R as GPIOPCEN_R;
///Field `GPIOPDEN` reader - IO port D clock enable
pub use GPIOPAEN_R as GPIOPDEN_R;
///Field `GPIOPEEN` reader - IO port E clock enable
pub use GPIOPAEN_R as GPIOPEEN_R;
///Field `GPIOPHEN` reader - IO port H clock enable
pub use GPIOPAEN_R as GPIOPHEN_R;
///Field `GPIOPFEN` reader - IO port F clock enable
pub use GPIOPAEN_R as GPIOPFEN_R;
///Field `GPIOPGEN` reader - IO port G clock enable
pub use GPIOPAEN_R as GPIOPGEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use GPIOPAEN_R as CRCEN_R;
///Field `FLITFEN` reader - FLITF clock enable
pub use GPIOPAEN_R as FLITFEN_R;
///Field `DMA1EN` reader - DMA1 clock enable
pub use GPIOPAEN_R as DMA1EN_R;
///Field `DMA2EN` reader - DMA2 clock enable
pub use GPIOPAEN_R as DMA2EN_R;
///Field `FSMCEN` reader - FSMCEN
pub use GPIOPAEN_R as FSMCEN_R;
///Field `GPIOPBEN` writer - IO port B clock enable
pub use GPIOPAEN_W as GPIOPBEN_W;
///Field `GPIOPCEN` writer - IO port C clock enable
pub use GPIOPAEN_W as GPIOPCEN_W;
///Field `GPIOPDEN` writer - IO port D clock enable
pub use GPIOPAEN_W as GPIOPDEN_W;
///Field `GPIOPEEN` writer - IO port E clock enable
pub use GPIOPAEN_W as GPIOPEEN_W;
///Field `GPIOPHEN` writer - IO port H clock enable
pub use GPIOPAEN_W as GPIOPHEN_W;
///Field `GPIOPFEN` writer - IO port F clock enable
pub use GPIOPAEN_W as GPIOPFEN_W;
///Field `GPIOPGEN` writer - IO port G clock enable
pub use GPIOPAEN_W as GPIOPGEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use GPIOPAEN_W as CRCEN_W;
///Field `FLITFEN` writer - FLITF clock enable
pub use GPIOPAEN_W as FLITFEN_W;
///Field `DMA1EN` writer - DMA1 clock enable
pub use GPIOPAEN_W as DMA1EN_W;
///Field `DMA2EN` writer - DMA2 clock enable
pub use GPIOPAEN_W as DMA2EN_W;
///Field `FSMCEN` writer - FSMCEN
pub use GPIOPAEN_W as FSMCEN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpiopaen(&self) -> GPIOPAEN_R {
        GPIOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpiopben(&self) -> GPIOPBEN_R {
        GPIOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiopcen(&self) -> GPIOPCEN_R {
        GPIOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpiopden(&self) -> GPIOPDEN_R {
        GPIOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpiopeen(&self) -> GPIOPEEN_R {
        GPIOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    pub fn gpiophen(&self) -> GPIOPHEN_R {
        GPIOPHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    pub fn gpiopfen(&self) -> GPIOPFEN_R {
        GPIOPFEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    pub fn gpiopgen(&self) -> GPIOPGEN_R {
        GPIOPGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopaen(&mut self) -> GPIOPAEN_W<0> {
        GPIOPAEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopben(&mut self) -> GPIOPBEN_W<1> {
        GPIOPBEN_W::new(self)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopcen(&mut self) -> GPIOPCEN_W<2> {
        GPIOPCEN_W::new(self)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopden(&mut self) -> GPIOPDEN_W<3> {
        GPIOPDEN_W::new(self)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopeen(&mut self) -> GPIOPEEN_W<4> {
        GPIOPEEN_W::new(self)
    }
    ///Bit 5 - IO port H clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiophen(&mut self) -> GPIOPHEN_W<5> {
        GPIOPHEN_W::new(self)
    }
    ///Bit 6 - IO port F clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopfen(&mut self) -> GPIOPFEN_W<6> {
        GPIOPFEN_W::new(self)
    }
    ///Bit 7 - IO port G clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiopgen(&mut self) -> GPIOPGEN_W<7> {
        GPIOPGEN_W::new(self)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    ///Bit 15 - FLITF clock enable
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<15> {
        FLITFEN_W::new(self)
    }
    ///Bit 24 - DMA1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<24> {
        DMA1EN_W::new(self)
    }
    ///Bit 25 - DMA2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<25> {
        DMA2EN_W::new(self)
    }
    ///Bit 30 - FSMCEN
    #[inline(always)]
    #[must_use]
    pub fn fsmcen(&mut self) -> FSMCEN_W<30> {
        FSMCEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbenr](index.html) module
pub struct AHBENR_SPEC;
impl crate::RegisterSpec for AHBENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbenr::R](R) reader structure
impl crate::Readable for AHBENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbenr::W](W) writer structure
impl crate::Writable for AHBENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHBENR to value 0x8000
impl crate::Resettable for AHBENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
