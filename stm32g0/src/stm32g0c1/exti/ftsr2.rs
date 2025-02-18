///Register `FTSR2` reader
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FTSR2` writer
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FT2` reader - Falling trigger event configuration bit of configurable line 34
pub type FT2_R = crate::BitReader<FT2_A>;
///Falling trigger event configuration bit of configurable line 34
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT2_A {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FT2_A> for bool {
    #[inline(always)]
    fn from(variant: FT2_A) -> Self {
        variant as u8 != 0
    }
}
impl FT2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FT2_A {
        match self.bits {
            false => FT2_A::Disabled,
            true => FT2_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT2_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT2_A::Enabled
    }
}
///Field `FT2` writer - Falling trigger event configuration bit of configurable line 34
pub type FT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, FT2_A, O>;
impl<'a, const O: u8> FT2_W<'a, O> {
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT2_A::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT2_A::Enabled)
    }
}
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable line 34
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<2> {
        FT2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling trigger selection register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr2](index.html) module
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ftsr2::R](R) reader structure
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ftsr2::W](W) writer structure
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
