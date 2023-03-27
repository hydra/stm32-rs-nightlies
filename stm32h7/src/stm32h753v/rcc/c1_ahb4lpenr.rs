///Register `C1_AHB4LPENR` reader
pub struct R(crate::R<C1_AHB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1_AHB4LPENR` writer
pub struct W(crate::W<C1_AHB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB4LPENR_SPEC>;
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
impl From<crate::W<C1_AHB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOALPEN` reader - GPIO peripheral clock enable during CSleep mode
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN_A>;
///GPIO peripheral clock enable during CSleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOALPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::Disabled,
            true => GPIOALPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOALPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOALPEN_A::Enabled
    }
}
///Field `GPIOALPEN` writer - GPIO peripheral clock enable during CSleep mode
pub type GPIOALPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, C1_AHB4LPENR_SPEC, GPIOALPEN_A, O>;
impl<'a, const O: u8> GPIOALPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::Enabled)
    }
}
///Field `GPIOBLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOBLPEN_R;
///Field `GPIOCLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOCLPEN_R;
///Field `GPIODLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIODLPEN_R;
///Field `GPIOELPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOELPEN_R;
///Field `GPIOFLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOFLPEN_R;
///Field `GPIOGLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOGLPEN_R;
///Field `GPIOHLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOHLPEN_R;
///Field `GPIOILPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOILPEN_R;
///Field `GPIOJLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOJLPEN_R;
///Field `GPIOKLPEN` reader - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as GPIOKLPEN_R;
///Field `CRCLPEN` reader - CRC peripheral clock enable during CSleep mode
pub use GPIOALPEN_R as CRCLPEN_R;
///Field `BDMALPEN` reader - BDMA Clock Enable During CSleep Mode
pub use GPIOALPEN_R as BDMALPEN_R;
///Field `ADC3LPEN` reader - ADC3 Peripheral Clocks Enable During CSleep Mode
pub use GPIOALPEN_R as ADC3LPEN_R;
///Field `BKPRAMLPEN` reader - Backup RAM Clock Enable During CSleep Mode
pub use GPIOALPEN_R as BKPRAMLPEN_R;
///Field `SRAM4LPEN` reader - SRAM4 Clock Enable During CSleep Mode
pub use GPIOALPEN_R as SRAM4LPEN_R;
///Field `GPIOBLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOBLPEN_W;
///Field `GPIOCLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOCLPEN_W;
///Field `GPIODLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIODLPEN_W;
///Field `GPIOELPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOELPEN_W;
///Field `GPIOFLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOFLPEN_W;
///Field `GPIOGLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOGLPEN_W;
///Field `GPIOHLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOHLPEN_W;
///Field `GPIOILPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOILPEN_W;
///Field `GPIOJLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOJLPEN_W;
///Field `GPIOKLPEN` writer - GPIO peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as GPIOKLPEN_W;
///Field `CRCLPEN` writer - CRC peripheral clock enable during CSleep mode
pub use GPIOALPEN_W as CRCLPEN_W;
///Field `BDMALPEN` writer - BDMA Clock Enable During CSleep Mode
pub use GPIOALPEN_W as BDMALPEN_W;
///Field `ADC3LPEN` writer - ADC3 Peripheral Clocks Enable During CSleep Mode
pub use GPIOALPEN_W as ADC3LPEN_W;
///Field `BKPRAMLPEN` writer - Backup RAM Clock Enable During CSleep Mode
pub use GPIOALPEN_W as BKPRAMLPEN_W;
///Field `SRAM4LPEN` writer - SRAM4 Clock Enable During CSleep Mode
pub use GPIOALPEN_W as SRAM4LPEN_W;
impl R {
    ///Bit 0 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 19 - CRC peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - BDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bdmalpen(&self) -> BDMALPEN_R {
        BDMALPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Backup RAM Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM4 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sram4lpen(&self) -> SRAM4LPEN_R {
        SRAM4LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    ///Bit 1 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    ///Bit 2 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    ///Bit 3 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    ///Bit 4 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<4> {
        GPIOELPEN_W::new(self)
    }
    ///Bit 5 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<5> {
        GPIOFLPEN_W::new(self)
    }
    ///Bit 6 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<6> {
        GPIOGLPEN_W::new(self)
    }
    ///Bit 7 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    ///Bit 8 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<8> {
        GPIOILPEN_W::new(self)
    }
    ///Bit 9 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<9> {
        GPIOJLPEN_W::new(self)
    }
    ///Bit 10 - GPIO peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<10> {
        GPIOKLPEN_W::new(self)
    }
    ///Bit 19 - CRC peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<19> {
        CRCLPEN_W::new(self)
    }
    ///Bit 21 - BDMA Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn bdmalpen(&mut self) -> BDMALPEN_W<21> {
        BDMALPEN_W::new(self)
    }
    ///Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W<24> {
        ADC3LPEN_W::new(self)
    }
    ///Bit 28 - Backup RAM Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<28> {
        BKPRAMLPEN_W::new(self)
    }
    ///Bit 29 - SRAM4 Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn sram4lpen(&mut self) -> SRAM4LPEN_W<29> {
        SRAM4LPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB4 Sleep Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1_ahb4lpenr](index.html) module
pub struct C1_AHB4LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB4LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1_ahb4lpenr::R](R) reader structure
impl crate::Readable for C1_AHB4LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1_ahb4lpenr::W](W) writer structure
impl crate::Writable for C1_AHB4LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1_AHB4LPENR to value 0
impl crate::Resettable for C1_AHB4LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
