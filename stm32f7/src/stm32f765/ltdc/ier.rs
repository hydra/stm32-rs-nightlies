///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LIE` reader - Line Interrupt Enable
pub type LIE_R = crate::BitReader<LIE_A>;
///Line Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIE_A {
    ///0: Line interrupt disabled
    Disabled = 0,
    ///1: Line interrupt enabled
    Enabled = 1,
}
impl From<LIE_A> for bool {
    #[inline(always)]
    fn from(variant: LIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LIE_A {
        match self.bits {
            false => LIE_A::Disabled,
            true => LIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LIE_A::Enabled
    }
}
///Field `LIE` writer - Line Interrupt Enable
pub type LIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LIE_A, O>;
impl<'a, const O: u8> LIE_W<'a, O> {
    ///Line interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIE_A::Disabled)
    }
    ///Line interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LIE_A::Enabled)
    }
}
///Field `FUIE` reader - FIFO Underrun Interrupt Enable
pub type FUIE_R = crate::BitReader<FUIE_A>;
///FIFO Underrun Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUIE_A {
    ///0: FIFO underrun interrupt disabled
    Disabled = 0,
    ///1: FIFO underrun interrupt enabled
    Enabled = 1,
}
impl From<FUIE_A> for bool {
    #[inline(always)]
    fn from(variant: FUIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FUIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FUIE_A {
        match self.bits {
            false => FUIE_A::Disabled,
            true => FUIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FUIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FUIE_A::Enabled
    }
}
///Field `FUIE` writer - FIFO Underrun Interrupt Enable
pub type FUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FUIE_A, O>;
impl<'a, const O: u8> FUIE_W<'a, O> {
    ///FIFO underrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FUIE_A::Disabled)
    }
    ///FIFO underrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FUIE_A::Enabled)
    }
}
///Field `TERRIE` reader - Transfer Error Interrupt Enable
pub type TERRIE_R = crate::BitReader<TERRIE_A>;
///Transfer Error Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIE_A {
    ///0: Transfer error interrupt disabled
    Disabled = 0,
    ///1: Transfer error interrupt enabled
    Enabled = 1,
}
impl From<TERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TERRIE_A {
        match self.bits {
            false => TERRIE_A::Disabled,
            true => TERRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE_A::Enabled
    }
}
///Field `TERRIE` writer - Transfer Error Interrupt Enable
pub type TERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TERRIE_A, O>;
impl<'a, const O: u8> TERRIE_W<'a, O> {
    ///Transfer error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TERRIE_A::Disabled)
    }
    ///Transfer error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TERRIE_A::Enabled)
    }
}
///Field `RRIE` reader - Register Reload interrupt enable
pub type RRIE_R = crate::BitReader<RRIE_A>;
///Register Reload interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRIE_A {
    ///0: Register reload interrupt disabled
    Disabled = 0,
    ///1: Register reload interrupt enabled
    Enabled = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::Disabled,
            true => RRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRIE_A::Enabled
    }
}
///Field `RRIE` writer - Register Reload interrupt enable
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RRIE_A, O>;
impl<'a, const O: u8> RRIE_W<'a, O> {
    ///Register reload interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RRIE_A::Disabled)
    }
    ///Register reload interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RRIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Line Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<0> {
        LIE_W::new(self)
    }
    ///Bit 1 - FIFO Underrun Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn fuie(&mut self) -> FUIE_W<1> {
        FUIE_W::new(self)
    }
    ///Bit 2 - Transfer Error Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<2> {
        TERRIE_W::new(self)
    }
    ///Bit 3 - Register Reload interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<3> {
        RRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
