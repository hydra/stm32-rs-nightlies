///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FREQ` reader - Peripheral clock frequency
pub type FREQ_R = crate::FieldReader<u8, u8>;
///Field `FREQ` writer - Peripheral clock frequency
pub type FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 6, O>;
///Field `ITERREN` reader - Error interrupt enable
pub type ITERREN_R = crate::BitReader<ITERREN_A>;
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERREN_A {
    ///0: Error interrupt disabled
    Disabled = 0,
    ///1: Error interrupt enabled
    Enabled = 1,
}
impl From<ITERREN_A> for bool {
    #[inline(always)]
    fn from(variant: ITERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITERREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITERREN_A {
        match self.bits {
            false => ITERREN_A::Disabled,
            true => ITERREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITERREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITERREN_A::Enabled
    }
}
///Field `ITERREN` writer - Error interrupt enable
pub type ITERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ITERREN_A, O>;
impl<'a, const O: u8> ITERREN_W<'a, O> {
    ///Error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITERREN_A::Disabled)
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITERREN_A::Enabled)
    }
}
///Field `ITEVTEN` reader - Event interrupt enable
pub type ITEVTEN_R = crate::BitReader<ITEVTEN_A>;
///Event interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEVTEN_A {
    ///0: Event interrupt disabled
    Disabled = 0,
    ///1: Event interrupt enabled
    Enabled = 1,
}
impl From<ITEVTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ITEVTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITEVTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITEVTEN_A {
        match self.bits {
            false => ITEVTEN_A::Disabled,
            true => ITEVTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITEVTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITEVTEN_A::Enabled
    }
}
///Field `ITEVTEN` writer - Event interrupt enable
pub type ITEVTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ITEVTEN_A, O>;
impl<'a, const O: u8> ITEVTEN_W<'a, O> {
    ///Event interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITEVTEN_A::Disabled)
    }
    ///Event interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITEVTEN_A::Enabled)
    }
}
///Field `ITBUFEN` reader - Buffer interrupt enable
pub type ITBUFEN_R = crate::BitReader<ITBUFEN_A>;
///Buffer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITBUFEN_A {
    ///0: TxE=1 or RxNE=1 does not generate any interrupt
    Disabled = 0,
    ///1: TxE=1 or RxNE=1 generates Event interrupt
    Enabled = 1,
}
impl From<ITBUFEN_A> for bool {
    #[inline(always)]
    fn from(variant: ITBUFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ITBUFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ITBUFEN_A {
        match self.bits {
            false => ITBUFEN_A::Disabled,
            true => ITBUFEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITBUFEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITBUFEN_A::Enabled
    }
}
///Field `ITBUFEN` writer - Buffer interrupt enable
pub type ITBUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ITBUFEN_A, O>;
impl<'a, const O: u8> ITBUFEN_W<'a, O> {
    ///TxE=1 or RxNE=1 does not generate any interrupt
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ITBUFEN_A::Disabled)
    }
    ///TxE=1 or RxNE=1 generates Event interrupt
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ITBUFEN_A::Enabled)
    }
}
///Field `DMAEN` reader - DMA requests enable
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
///DMA requests enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    ///0: DMA requests disabled
    Disabled = 0,
    ///1: DMA request enabled when TxE=1 or RxNE=1
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
///Field `DMAEN` writer - DMA requests enable
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    ///DMA requests disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    ///DMA request enabled when TxE=1 or RxNE=1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
///Field `LAST` reader - DMA last transfer
pub type LAST_R = crate::BitReader<LAST_A>;
///DMA last transfer
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAST_A {
    ///0: Next DMA EOT is not the last transfer
    NotLast = 0,
    ///1: Next DMA EOT is the last transfer
    Last = 1,
}
impl From<LAST_A> for bool {
    #[inline(always)]
    fn from(variant: LAST_A) -> Self {
        variant as u8 != 0
    }
}
impl LAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LAST_A {
        match self.bits {
            false => LAST_A::NotLast,
            true => LAST_A::Last,
        }
    }
    ///Checks if the value of the field is `NotLast`
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == LAST_A::NotLast
    }
    ///Checks if the value of the field is `Last`
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == LAST_A::Last
    }
}
///Field `LAST` writer - DMA last transfer
pub type LAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LAST_A, O>;
impl<'a, const O: u8> LAST_W<'a, O> {
    ///Next DMA EOT is not the last transfer
    #[inline(always)]
    pub fn not_last(self) -> &'a mut W {
        self.variant(LAST_A::NotLast)
    }
    ///Next DMA EOT is the last transfer
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(LAST_A::Last)
    }
}
impl R {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    pub fn iterren(&self) -> ITERREN_R {
        ITERREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    pub fn itevten(&self) -> ITEVTEN_R {
        ITEVTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    pub fn itbufen(&self) -> ITBUFEN_R {
        ITBUFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    pub fn last(&self) -> LAST_R {
        LAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - Peripheral clock frequency
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FREQ_W<0> {
        FREQ_W::new(self)
    }
    ///Bit 8 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn iterren(&mut self) -> ITERREN_W<8> {
        ITERREN_W::new(self)
    }
    ///Bit 9 - Event interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itevten(&mut self) -> ITEVTEN_W<9> {
        ITEVTEN_W::new(self)
    }
    ///Bit 10 - Buffer interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn itbufen(&mut self) -> ITBUFEN_W<10> {
        ITBUFEN_W::new(self)
    }
    ///Bit 11 - DMA requests enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<11> {
        DMAEN_W::new(self)
    }
    ///Bit 12 - DMA last transfer
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LAST_W<12> {
        LAST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CR2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
