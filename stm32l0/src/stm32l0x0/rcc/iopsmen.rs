///Register `IOPSMEN` reader
pub struct R(crate::R<IOPSMEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMEN_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOPSMEN` writer
pub struct W(crate::W<IOPSMEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMEN_SPEC>;
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
impl From<crate::W<IOPSMEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMEN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IOPASMEN` reader - Port A clock enable during Sleep mode bit
pub type IOPASMEN_R = crate::BitReader<IOPASMEN_A>;
///Port A clock enable during Sleep mode bit
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOPASMEN_A {
    ///0: Port x clock is disabled in Sleep mode
    Disabled = 0,
    ///1: Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
    Enabled = 1,
}
impl From<IOPASMEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOPASMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IOPASMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IOPASMEN_A {
        match self.bits {
            false => IOPASMEN_A::Disabled,
            true => IOPASMEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOPASMEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOPASMEN_A::Enabled
    }
}
///Field `IOPASMEN` writer - Port A clock enable during Sleep mode bit
pub type IOPASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMEN_SPEC, IOPASMEN_A, O>;
impl<'a, const O: u8> IOPASMEN_W<'a, O> {
    ///Port x clock is disabled in Sleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::Disabled)
    }
    ///Port x clock is enabled in Sleep mode (if enabled by IOPHEN)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOPASMEN_A::Enabled)
    }
}
///Field `IOPBSMEN` reader - Port B clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPBSMEN_R;
///Field `IOPCSMEN` reader - Port C clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPCSMEN_R;
///Field `IOPDSMEN` reader - Port D clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPDSMEN_R;
///Field `IOPESMEN` reader - Port E clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPESMEN_R;
///Field `IOPHSMEN` reader - Port H clock enable during Sleep mode bit
pub use IOPASMEN_R as IOPHSMEN_R;
///Field `IOPBSMEN` writer - Port B clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPBSMEN_W;
///Field `IOPCSMEN` writer - Port C clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPCSMEN_W;
///Field `IOPDSMEN` writer - Port D clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPDSMEN_W;
///Field `IOPESMEN` writer - Port E clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPESMEN_W;
///Field `IOPHSMEN` writer - Port H clock enable during Sleep mode bit
pub use IOPASMEN_W as IOPHSMEN_W;
impl R {
    ///Bit 0 - Port A clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopasmen(&self) -> IOPASMEN_R {
        IOPASMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port B clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopbsmen(&self) -> IOPBSMEN_R {
        IOPBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port C clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopcsmen(&self) -> IOPCSMEN_R {
        IOPCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port D clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopdsmen(&self) -> IOPDSMEN_R {
        IOPDSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iopesmen(&self) -> IOPESMEN_R {
        IOPESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Port H clock enable during Sleep mode bit
    #[inline(always)]
    pub fn iophsmen(&self) -> IOPHSMEN_R {
        IOPHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port A clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iopasmen(&mut self) -> IOPASMEN_W<0> {
        IOPASMEN_W::new(self)
    }
    ///Bit 1 - Port B clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iopbsmen(&mut self) -> IOPBSMEN_W<1> {
        IOPBSMEN_W::new(self)
    }
    ///Bit 2 - Port C clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iopcsmen(&mut self) -> IOPCSMEN_W<2> {
        IOPCSMEN_W::new(self)
    }
    ///Bit 3 - Port D clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iopdsmen(&mut self) -> IOPDSMEN_W<3> {
        IOPDSMEN_W::new(self)
    }
    ///Bit 4 - Port E clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iopesmen(&mut self) -> IOPESMEN_W<4> {
        IOPESMEN_W::new(self)
    }
    ///Bit 7 - Port H clock enable during Sleep mode bit
    #[inline(always)]
    #[must_use]
    pub fn iophsmen(&mut self) -> IOPHSMEN_W<7> {
        IOPHSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO clock enable in sleep mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iopsmen](index.html) module
pub struct IOPSMEN_SPEC;
impl crate::RegisterSpec for IOPSMEN_SPEC {
    type Ux = u32;
}
///`read()` method returns [iopsmen::R](R) reader structure
impl crate::Readable for IOPSMEN_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iopsmen::W](W) writer structure
impl crate::Writable for IOPSMEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOPSMEN to value 0x8f
impl crate::Resettable for IOPSMEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x8f;
}
