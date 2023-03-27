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
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
///DMA1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
///Field `SRAMEN` reader - SRAM interface clock enable
pub use DMA1EN_R as SRAMEN_R;
///Field `FLITFEN` reader - FLITF clock enable
pub use DMA1EN_R as FLITFEN_R;
///Field `CRCEN` reader - CRC clock enable
pub use DMA1EN_R as CRCEN_R;
///Field `IOPAEN` reader - I/O port A clock enable
pub use DMA1EN_R as IOPAEN_R;
///Field `IOPBEN` reader - I/O port B clock enable
pub use DMA1EN_R as IOPBEN_R;
///Field `IOPCEN` reader - I/O port C clock enable
pub use DMA1EN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable
pub use DMA1EN_R as IOPDEN_R;
///Field `IOPFEN` reader - I/O port F clock enable
pub use DMA1EN_R as IOPFEN_R;
///Field `TSCEN` reader - Touch sensing controller clock enable
pub use DMA1EN_R as TSCEN_R;
///Field `ADC12EN` reader - ADC1 and ADC2 clock enable
pub use DMA1EN_R as ADC12EN_R;
///Field `ADC34EN` reader - ADC3 and ADC4 clock enable
pub use DMA1EN_R as ADC34EN_R;
///Field `SRAMEN` writer - SRAM interface clock enable
pub use DMA1EN_W as SRAMEN_W;
///Field `FLITFEN` writer - FLITF clock enable
pub use DMA1EN_W as FLITFEN_W;
///Field `CRCEN` writer - CRC clock enable
pub use DMA1EN_W as CRCEN_W;
///Field `IOPAEN` writer - I/O port A clock enable
pub use DMA1EN_W as IOPAEN_W;
///Field `IOPBEN` writer - I/O port B clock enable
pub use DMA1EN_W as IOPBEN_W;
///Field `IOPCEN` writer - I/O port C clock enable
pub use DMA1EN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable
pub use DMA1EN_W as IOPDEN_W;
///Field `IOPFEN` writer - I/O port F clock enable
pub use DMA1EN_W as IOPFEN_W;
///Field `TSCEN` writer - Touch sensing controller clock enable
pub use DMA1EN_W as TSCEN_W;
///Field `ADC12EN` writer - ADC1 and ADC2 clock enable
pub use DMA1EN_W as ADC12EN_W;
///Field `ADC34EN` writer - ADC3 and ADC4 clock enable
pub use DMA1EN_W as ADC34EN_W;
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - I/O port A clock enable
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - I/O port B clock enable
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - I/O port C clock enable
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I/O port D clock enable
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - I/O port F clock enable
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Touch sensing controller clock enable
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ADC1 and ADC2 clock enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC3 and ADC4 clock enable
    #[inline(always)]
    pub fn adc34en(&self) -> ADC34EN_R {
        ADC34EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    ///Bit 2 - SRAM interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    ///Bit 4 - FLITF clock enable
    #[inline(always)]
    #[must_use]
    pub fn flitfen(&mut self) -> FLITFEN_W<4> {
        FLITFEN_W::new(self)
    }
    ///Bit 6 - CRC clock enable
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<6> {
        CRCEN_W::new(self)
    }
    ///Bit 17 - I/O port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<17> {
        IOPAEN_W::new(self)
    }
    ///Bit 18 - I/O port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<18> {
        IOPBEN_W::new(self)
    }
    ///Bit 19 - I/O port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<19> {
        IOPCEN_W::new(self)
    }
    ///Bit 20 - I/O port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<20> {
        IOPDEN_W::new(self)
    }
    ///Bit 22 - I/O port F clock enable
    #[inline(always)]
    #[must_use]
    pub fn iopfen(&mut self) -> IOPFEN_W<22> {
        IOPFEN_W::new(self)
    }
    ///Bit 24 - Touch sensing controller clock enable
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<24> {
        TSCEN_W::new(self)
    }
    ///Bit 28 - ADC1 and ADC2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<28> {
        ADC12EN_W::new(self)
    }
    ///Bit 29 - ADC3 and ADC4 clock enable
    #[inline(always)]
    #[must_use]
    pub fn adc34en(&mut self) -> ADC34EN_W<29> {
        ADC34EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB Peripheral Clock enable register (RCC_AHBENR)
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
///`reset()` method sets AHBENR to value 0x14
impl crate::Resettable for AHBENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
