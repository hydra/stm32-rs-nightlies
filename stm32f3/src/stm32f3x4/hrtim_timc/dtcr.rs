///Register `DTCR` reader
pub struct R(crate::R<DTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTCR` writer
pub struct W(crate::W<DTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCR_SPEC>;
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
impl From<crate::W<DTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTRx` reader - Deadtime Rising value
pub type DTRX_R = crate::FieldReader<u16, u16>;
///Field `DTRx` writer - Deadtime Rising value
pub type DTRX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTCR_SPEC, u16, u16, 9, O>;
///Field `SDTRx` reader - Sign Deadtime Rising value
pub type SDTRX_R = crate::BitReader<SDTRX_A>;
///Sign Deadtime Rising value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTRX_A {
    ///0: Positive deadtime on rising edge
    Positive = 0,
    ///1: Negative deadtime on rising edge
    Negative = 1,
}
impl From<SDTRX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTRX_A) -> Self {
        variant as u8 != 0
    }
}
impl SDTRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDTRX_A {
        match self.bits {
            false => SDTRX_A::Positive,
            true => SDTRX_A::Negative,
        }
    }
    ///Checks if the value of the field is `Positive`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTRX_A::Positive
    }
    ///Checks if the value of the field is `Negative`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTRX_A::Negative
    }
}
///Field `SDTRx` writer - Sign Deadtime Rising value
pub type SDTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, SDTRX_A, O>;
impl<'a, const O: u8> SDTRX_W<'a, O> {
    ///Positive deadtime on rising edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTRX_A::Positive)
    }
    ///Negative deadtime on rising edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTRX_A::Negative)
    }
}
///Field `DTPRSC` reader - Deadtime Prescaler
pub type DTPRSC_R = crate::FieldReader<u8, u8>;
///Field `DTPRSC` writer - Deadtime Prescaler
pub type DTPRSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTCR_SPEC, u8, u8, 3, O>;
///Field `DTRSLKx` reader - Deadtime Rising Sign Lock
pub type DTRSLKX_R = crate::BitReader<DTRSLKX_A>;
///Deadtime Rising Sign Lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRSLKX_A {
    ///0: Deadtime rising sign is writable
    Unlocked = 0,
    ///1: Deadtime rising sign is read-only
    Locked = 1,
}
impl From<DTRSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRSLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTRSLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTRSLKX_A {
        match self.bits {
            false => DTRSLKX_A::Unlocked,
            true => DTRSLKX_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRSLKX_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLKX_A::Locked
    }
}
///Field `DTRSLKx` writer - Deadtime Rising Sign Lock
pub type DTRSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, DTRSLKX_A, O>;
impl<'a, const O: u8> DTRSLKX_W<'a, O> {
    ///Deadtime rising sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::Unlocked)
    }
    ///Deadtime rising sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRSLKX_A::Locked)
    }
}
///Field `DTRLKx` reader - Deadtime Rising Lock
pub type DTRLKX_R = crate::BitReader<DTRLKX_A>;
///Deadtime Rising Lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRLKX_A {
    ///0: Deadtime rising value and sign is writable
    Unlocked = 0,
    ///1: Deadtime rising value and sign is read-only
    Locked = 1,
}
impl From<DTRLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTRLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTRLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTRLKX_A {
        match self.bits {
            false => DTRLKX_A::Unlocked,
            true => DTRLKX_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRLKX_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLKX_A::Locked
    }
}
///Field `DTRLKx` writer - Deadtime Rising Lock
pub type DTRLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, DTRLKX_A, O>;
impl<'a, const O: u8> DTRLKX_W<'a, O> {
    ///Deadtime rising value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTRLKX_A::Unlocked)
    }
    ///Deadtime rising value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTRLKX_A::Locked)
    }
}
///Field `DTFx` reader - Deadtime Falling value
pub type DTFX_R = crate::FieldReader<u16, u16>;
///Field `DTFx` writer - Deadtime Falling value
pub type DTFX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DTCR_SPEC, u16, u16, 9, O>;
///Field `SDTFx` reader - Sign Deadtime Falling value
pub type SDTFX_R = crate::BitReader<SDTFX_A>;
///Sign Deadtime Falling value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTFX_A {
    ///0: Positive deadtime on falling edge
    Positive = 0,
    ///1: Negative deadtime on falling edge
    Negative = 1,
}
impl From<SDTFX_A> for bool {
    #[inline(always)]
    fn from(variant: SDTFX_A) -> Self {
        variant as u8 != 0
    }
}
impl SDTFX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDTFX_A {
        match self.bits {
            false => SDTFX_A::Positive,
            true => SDTFX_A::Negative,
        }
    }
    ///Checks if the value of the field is `Positive`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTFX_A::Positive
    }
    ///Checks if the value of the field is `Negative`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTFX_A::Negative
    }
}
///Field `SDTFx` writer - Sign Deadtime Falling value
pub type SDTFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, SDTFX_A, O>;
impl<'a, const O: u8> SDTFX_W<'a, O> {
    ///Positive deadtime on falling edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(SDTFX_A::Positive)
    }
    ///Negative deadtime on falling edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(SDTFX_A::Negative)
    }
}
///Field `DTFSLKx` reader - Deadtime Falling Sign Lock
pub type DTFSLKX_R = crate::BitReader<DTFSLKX_A>;
///Deadtime Falling Sign Lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFSLKX_A {
    ///0: Deadtime falling sign is writable
    Unlocked = 0,
    ///1: Deadtime falling sign is read-only
    Locked = 1,
}
impl From<DTFSLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFSLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTFSLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTFSLKX_A {
        match self.bits {
            false => DTFSLKX_A::Unlocked,
            true => DTFSLKX_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFSLKX_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLKX_A::Locked
    }
}
///Field `DTFSLKx` writer - Deadtime Falling Sign Lock
pub type DTFSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, DTFSLKX_A, O>;
impl<'a, const O: u8> DTFSLKX_W<'a, O> {
    ///Deadtime falling sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::Unlocked)
    }
    ///Deadtime falling sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFSLKX_A::Locked)
    }
}
///Field `DTFLKx` reader - Deadtime Falling Lock
pub type DTFLKX_R = crate::BitReader<DTFLKX_A>;
///Deadtime Falling Lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFLKX_A {
    ///0: Deadtime falling value and sign is writable
    Unlocked = 0,
    ///1: Deadtime falling value and sign is read-only
    Locked = 1,
}
impl From<DTFLKX_A> for bool {
    #[inline(always)]
    fn from(variant: DTFLKX_A) -> Self {
        variant as u8 != 0
    }
}
impl DTFLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTFLKX_A {
        match self.bits {
            false => DTFLKX_A::Unlocked,
            true => DTFLKX_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFLKX_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLKX_A::Locked
    }
}
///Field `DTFLKx` writer - Deadtime Falling Lock
pub type DTFLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCR_SPEC, DTFLKX_A, O>;
impl<'a, const O: u8> DTFLKX_W<'a, O> {
    ///Deadtime falling value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DTFLKX_A::Unlocked)
    }
    ///Deadtime falling value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DTFLKX_A::Locked)
    }
}
impl R {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<0> {
        DTRX_W::new(self)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<9> {
        SDTRX_W::new(self)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<10> {
        DTPRSC_W::new(self)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<14> {
        DTRSLKX_W::new(self)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<15> {
        DTRLKX_W::new(self)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<16> {
        DTFX_W::new(self)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<25> {
        SDTFX_W::new(self)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<30> {
        DTFSLKX_W::new(self)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<31> {
        DTFLKX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtcr](index.html) module
pub struct DTCR_SPEC;
impl crate::RegisterSpec for DTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dtcr::R](R) reader structure
impl crate::Readable for DTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dtcr::W](W) writer structure
impl crate::Writable for DTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTCR to value 0
impl crate::Resettable for DTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
