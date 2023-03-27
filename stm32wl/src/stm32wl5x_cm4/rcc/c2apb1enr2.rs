///Register `C2APB1ENR2` reader
pub struct R(crate::R<C2APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB1ENR2` writer
pub struct W(crate::W<C2APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB1ENR2_SPEC>;
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
impl From<crate::W<C2APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPUART1EN` reader - CPU2 Low power UART 1 clocks enable
pub type LPUART1EN_R = crate::BitReader<LPUART1EN_A>;
///CPU2 Low power UART 1 clocks enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1EN_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPUART1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::Disabled,
            true => LPUART1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN_A::Enabled
    }
}
///Field `LPUART1EN` writer - CPU2 Low power UART 1 clocks enable
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB1ENR2_SPEC, LPUART1EN_A, O>;
impl<'a, const O: u8> LPUART1EN_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Enabled)
    }
}
///Field `LPTIM2EN` reader - CPU2 Low power timer 2 clocks enable
pub use LPUART1EN_R as LPTIM2EN_R;
///Field `LPTIM3EN` reader - CPU2 Low power timer 3 clocks enable
pub use LPUART1EN_R as LPTIM3EN_R;
///Field `LPTIM2EN` writer - CPU2 Low power timer 2 clocks enable
pub use LPUART1EN_W as LPTIM2EN_W;
///Field `LPTIM3EN` writer - CPU2 Low power timer 3 clocks enable
pub use LPUART1EN_W as LPTIM3EN_W;
impl R {
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU2 Low power UART 1 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<0> {
        LPUART1EN_W::new(self)
    }
    ///Bit 5 - CPU2 Low power timer 2 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<5> {
        LPTIM2EN_W::new(self)
    }
    ///Bit 6 - CPU2 Low power timer 3 clocks enable
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<6> {
        LPTIM3EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 APB1 peripheral clock enable register 2 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb1enr2](index.html) module
pub struct C2APB1ENR2_SPEC;
impl crate::RegisterSpec for C2APB1ENR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb1enr2::R](R) reader structure
impl crate::Readable for C2APB1ENR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb1enr2::W](W) writer structure
impl crate::Writable for C2APB1ENR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2APB1ENR2 to value 0
impl crate::Resettable for C2APB1ENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
