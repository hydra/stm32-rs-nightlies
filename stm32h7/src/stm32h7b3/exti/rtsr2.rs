///Register `RTSR2` reader
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR2` writer
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TR49` reader - Rising trigger event configuration bit of Configurable Event input x+32
pub type TR49_R = crate::BitReader<TR49_A>;
///Rising trigger event configuration bit of Configurable Event input x+32
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR49_A {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<TR49_A> for bool {
    #[inline(always)]
    fn from(variant: TR49_A) -> Self {
        variant as u8 != 0
    }
}
impl TR49_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TR49_A {
        match self.bits {
            false => TR49_A::Disabled,
            true => TR49_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR49_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR49_A::Enabled
    }
}
///Field `TR49` writer - Rising trigger event configuration bit of Configurable Event input x+32
pub type TR49_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, TR49_A, O>;
impl<'a, const O: u8> TR49_W<'a, O> {
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR49_A::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR49_A::Enabled)
    }
}
///Field `TR51` reader - Rising trigger event configuration bit of Configurable Event input x+32
pub use TR49_R as TR51_R;
///Field `TR51` writer - Rising trigger event configuration bit of Configurable Event input x+32
pub use TR49_W as TR51_W;
impl R {
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn tr49(&mut self) -> TR49_W<17> {
        TR49_W::new(self)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn tr51(&mut self) -> TR51_W<19> {
        TR51_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI rising trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr2](index.html) module
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr2::R](R) reader structure
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr2::W](W) writer structure
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
