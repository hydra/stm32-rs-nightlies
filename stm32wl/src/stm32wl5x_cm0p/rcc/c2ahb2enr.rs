///Register `C2AHB2ENR` reader
pub struct R(crate::R<C2AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2AHB2ENR` writer
pub struct W(crate::W<C2AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2ENR_SPEC>;
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
impl From<crate::W<C2AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOAEN` reader - CPU2 IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
///CPU2 IO port A clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
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
///Field `GPIOAEN` writer - CPU2 IO port A clock enable
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
///Field `GPIOBEN` reader - CPU2 IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - CPU2 IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIOHEN` reader - CPU2 IO port H clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOBEN` writer - CPU2 IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - CPU2 IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIOHEN` writer - CPU2 IO port H clock enable
pub use GPIOAEN_W as GPIOHEN_W;
impl R {
    ///Bit 0 - CPU2 IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 7 - CPU2 IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU2 IO port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - CPU2 IO port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - CPU2 IO port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 7 - CPU2 IO port H clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2ahb2enr](index.html) module
pub struct C2AHB2ENR_SPEC;
impl crate::RegisterSpec for C2AHB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2ahb2enr::R](R) reader structure
impl crate::Readable for C2AHB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2ahb2enr::W](W) writer structure
impl crate::Writable for C2AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2AHB2ENR to value 0
impl crate::Resettable for C2AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
