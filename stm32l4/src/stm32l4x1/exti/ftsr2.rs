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
///Field `FT35` reader - Falling trigger event configuration bit of line 35
pub type FT35_R = crate::BitReader<FT35_A>;
///Falling trigger event configuration bit of line 35
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT35_A {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FT35_A> for bool {
    #[inline(always)]
    fn from(variant: FT35_A) -> Self {
        variant as u8 != 0
    }
}
impl FT35_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FT35_A {
        match self.bits {
            false => FT35_A::Disabled,
            true => FT35_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT35_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT35_A::Enabled
    }
}
///Field `FT35` writer - Falling trigger event configuration bit of line 35
pub type FT35_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, FT35_A, O>;
impl<'a, const O: u8> FT35_W<'a, O> {
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT35_A::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT35_A::Enabled)
    }
}
///Field `FT36` reader - Falling trigger event configuration bit of line 36
pub use FT35_R as FT36_R;
///Field `FT37` reader - Falling trigger event configuration bit of line 37
pub use FT35_R as FT37_R;
///Field `FT38` reader - Falling trigger event configuration bit of line 38
pub use FT35_R as FT38_R;
///Field `FT36` writer - Falling trigger event configuration bit of line 36
pub use FT35_W as FT36_W;
///Field `FT37` writer - Falling trigger event configuration bit of line 37
pub use FT35_W as FT37_W;
///Field `FT38` writer - Falling trigger event configuration bit of line 38
pub use FT35_W as FT38_W;
impl R {
    ///Bit 3 - Falling trigger event configuration bit of line 35
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration bit of line 36
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration bit of line 37
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration bit of line 38
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Falling trigger event configuration bit of line 35
    #[inline(always)]
    #[must_use]
    pub fn ft35(&mut self) -> FT35_W<3> {
        FT35_W::new(self)
    }
    ///Bit 4 - Falling trigger event configuration bit of line 36
    #[inline(always)]
    #[must_use]
    pub fn ft36(&mut self) -> FT36_W<4> {
        FT36_W::new(self)
    }
    ///Bit 5 - Falling trigger event configuration bit of line 37
    #[inline(always)]
    #[must_use]
    pub fn ft37(&mut self) -> FT37_W<5> {
        FT37_W::new(self)
    }
    ///Bit 6 - Falling trigger event configuration bit of line 38
    #[inline(always)]
    #[must_use]
    pub fn ft38(&mut self) -> FT38_W<6> {
        FT38_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Falling Trigger selection register
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
