///Register `AHB4ENR` reader
pub struct R(crate::R<AHB4ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB4ENR` writer
pub struct W(crate::W<AHB4ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4ENR_SPEC>;
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
impl From<crate::W<AHB4ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOAEN` reader - GPIOA peripheral clock enable Set and reset by software.
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
///GPIOA peripheral clock enable Set and reset by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
///Field `GPIOAEN` writer - GPIOA peripheral clock enable Set and reset by software.
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
///Field `GPIOBEN` reader - GPIOB peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - GPIOC peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - GPIOD peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - GPIOE peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - GPIOF peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - GPIOG peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - GPIOH peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - GPIOI peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOIEN_R;
///Field `GPIOJEN` reader - GPIOJ peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOJEN_R;
///Field `GPIOKEN` reader - GPIOK peripheral clock enable Set and reset by software.
pub use GPIOAEN_R as GPIOKEN_R;
///Field `BDMA2EN` reader - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
pub use GPIOAEN_R as BDMA2EN_R;
///Field `BKPRAMEN` reader - Backup RAM clock enable Set and reset by software.
pub use GPIOAEN_R as BKPRAMEN_R;
///Field `SRDSRAMEN` reader - SmartRun domain SRAM clock enable Set and reset by software.
pub use GPIOAEN_R as SRDSRAMEN_R;
///Field `GPIOBEN` writer - GPIOB peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - GPIOC peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - GPIOD peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - GPIOE peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - GPIOF peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - GPIOG peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - GPIOH peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - GPIOI peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOIEN_W;
///Field `GPIOJEN` writer - GPIOJ peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOJEN_W;
///Field `GPIOKEN` writer - GPIOK peripheral clock enable Set and reset by software.
pub use GPIOAEN_W as GPIOKEN_W;
///Field `BDMA2EN` writer - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
pub use GPIOAEN_W as BDMA2EN_W;
///Field `BKPRAMEN` writer - Backup RAM clock enable Set and reset by software.
pub use GPIOAEN_W as BKPRAMEN_W;
///Field `SRDSRAMEN` writer - SmartRun domain SRAM clock enable Set and reset by software.
pub use GPIOAEN_W as SRDSRAMEN_W;
impl R {
    ///Bit 0 - GPIOA peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOI peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJ peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOK peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
    #[inline(always)]
    pub fn bdma2en(&self) -> BDMA2EN_R {
        BDMA2EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - Backup RAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable Set and reset by software.
    #[inline(always)]
    pub fn srdsramen(&self) -> SRDSRAMEN_R {
        SRDSRAMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOA peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - GPIOB peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - GPIOC peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 3 - GPIOD peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    ///Bit 4 - GPIOE peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    ///Bit 5 - GPIOF peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    ///Bit 6 - GPIOG peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    ///Bit 7 - GPIOH peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    ///Bit 8 - GPIOI peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    ///Bit 9 - GPIOJ peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<9> {
        GPIOJEN_W::new(self)
    }
    ///Bit 10 - GPIOK peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gpioken(&mut self) -> GPIOKEN_W<10> {
        GPIOKEN_W::new(self)
    }
    ///Bit 21 - SmartRun domain DMA and DMAMUX clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn bdma2en(&mut self) -> BDMA2EN_W<21> {
        BDMA2EN_W::new(self)
    }
    ///Bit 28 - Backup RAM clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<28> {
        BKPRAMEN_W::new(self)
    }
    ///Bit 29 - SmartRun domain SRAM clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn srdsramen(&mut self) -> SRDSRAMEN_W<29> {
        SRDSRAMEN_W::new(self)
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
///For information about available fields see [ahb4enr](index.html) module
pub struct AHB4ENR_SPEC;
impl crate::RegisterSpec for AHB4ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb4enr::R](R) reader structure
impl crate::Readable for AHB4ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb4enr::W](W) writer structure
impl crate::Writable for AHB4ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB4ENR to value 0
impl crate::Resettable for AHB4ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
