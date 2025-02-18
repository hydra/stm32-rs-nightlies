///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `T` reader - 7-bit counter
pub type T_R = crate::FieldReader<u8, u8>;
///Field `T` writer - 7-bit counter
pub type T_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 7, O>;
///Field `WDGA` reader - Activation bit
pub type WDGA_R = crate::BitReader<WDGA_A>;
///Activation bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDGA_A {
    ///0: Watchdog disabled
    Disabled = 0,
    ///1: Watchdog enabled
    Enabled = 1,
}
impl From<WDGA_A> for bool {
    #[inline(always)]
    fn from(variant: WDGA_A) -> Self {
        variant as u8 != 0
    }
}
impl WDGA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDGA_A {
        match self.bits {
            false => WDGA_A::Disabled,
            true => WDGA_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDGA_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDGA_A::Enabled
    }
}
///Field `WDGA` writer - Activation bit
pub type WDGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, WDGA_A, O>;
impl<'a, const O: u8> WDGA_W<'a, O> {
    ///Watchdog disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDGA_A::Disabled)
    }
    ///Watchdog enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDGA_A::Enabled)
    }
}
impl R {
    ///Bits 0:6 - 7-bit counter
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Activation bit
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - 7-bit counter
    #[inline(always)]
    #[must_use]
    pub fn t(&mut self) -> T_W<0> {
        T_W::new(self)
    }
    ///Bit 7 - Activation bit
    #[inline(always)]
    #[must_use]
    pub fn wdga(&mut self) -> WDGA_W<7> {
        WDGA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0x7f
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
