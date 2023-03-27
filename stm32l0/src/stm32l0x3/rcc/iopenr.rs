///Register `IOPENR` reader
pub struct R(crate::R<IOPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOPENR` writer
pub struct W(crate::W<IOPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPENR_SPEC>;
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
impl From<crate::W<IOPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IOPAEN` reader - IO port A clock enable bit
pub type IOPAEN_R = crate::BitReader<IOPAEN_A>;
///IO port A clock enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPAEN_A {
    ///0: Port clock disabled
    Disabled = 0,
    ///1: Port clock enabled
    Enabled = 1,
}
impl From<IOPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IOPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IOPAEN_A {
        match self.bits {
            false => IOPAEN_A::Disabled,
            true => IOPAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPAEN_A::Enabled
    }
}
///Field `IOPAEN` writer - IO port A clock enable bit
pub type IOPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPENR_SPEC, IOPAEN_A, O>;
impl<'a, const O: u8> IOPAEN_W<'a, O> {
    ///Port clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPAEN_A::Disabled)
    }
    ///Port clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPAEN_A::Enabled)
    }
}
///Field `IOPBEN` reader - IO port B clock enable bit
pub use IOPAEN_R as IOPBEN_R;
///Field `IOPCEN` reader - IO port A clock enable bit
pub use IOPAEN_R as IOPCEN_R;
///Field `IOPDEN` reader - I/O port D clock enable bit
pub use IOPAEN_R as IOPDEN_R;
///Field `IOPEEN` reader - I/O port E clock enable bit
pub use IOPAEN_R as IOPEEN_R;
///Field `IOPHEN` reader - I/O port H clock enable bit
pub use IOPAEN_R as IOPHEN_R;
///Field `IOPBEN` writer - IO port B clock enable bit
pub use IOPAEN_W as IOPBEN_W;
///Field `IOPCEN` writer - IO port A clock enable bit
pub use IOPAEN_W as IOPCEN_W;
///Field `IOPDEN` writer - I/O port D clock enable bit
pub use IOPAEN_W as IOPDEN_W;
///Field `IOPEEN` writer - I/O port E clock enable bit
pub use IOPAEN_W as IOPEEN_W;
///Field `IOPHEN` writer - I/O port H clock enable bit
pub use IOPAEN_W as IOPHEN_W;
impl R {
    ///Bit 0 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable bit
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port A clock enable bit
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I/O port D clock enable bit
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O port E clock enable bit
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - I/O port H clock enable bit
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iopaen(&mut self) -> IOPAEN_W<0> {
        IOPAEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iopben(&mut self) -> IOPBEN_W<1> {
        IOPBEN_W::new(self)
    }
    ///Bit 2 - IO port A clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iopcen(&mut self) -> IOPCEN_W<2> {
        IOPCEN_W::new(self)
    }
    ///Bit 3 - I/O port D clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iopden(&mut self) -> IOPDEN_W<3> {
        IOPDEN_W::new(self)
    }
    ///Bit 4 - I/O port E clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iopeen(&mut self) -> IOPEEN_W<4> {
        IOPEEN_W::new(self)
    }
    ///Bit 7 - I/O port H clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn iophen(&mut self) -> IOPHEN_W<7> {
        IOPHEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iopenr](index.html) module
pub struct IOPENR_SPEC;
impl crate::RegisterSpec for IOPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iopenr::R](R) reader structure
impl crate::Readable for IOPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iopenr::W](W) writer structure
impl crate::Writable for IOPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOPENR to value 0
impl crate::Resettable for IOPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
