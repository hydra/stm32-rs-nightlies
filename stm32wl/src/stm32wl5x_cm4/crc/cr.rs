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
///Field `RESET` reader - RESET bit
pub type RESET_R = crate::BitReader<RESETW_A>;
///RESET bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW_A {
    ///1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    Reset = 1,
}
impl From<RESETW_A> for bool {
    #[inline(always)]
    fn from(variant: RESETW_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RESETW_A> {
        match self.bits {
            true => Some(RESETW_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETW_A::Reset
    }
}
///Field `RESET` writer - RESET bit
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RESETW_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW_A::Reset)
    }
}
///Field `POLYSIZE` reader - Polynomial size
pub type POLYSIZE_R = crate::FieldReader<u8, POLYSIZE_A>;
///Polynomial size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    ///0: 32-bit polynomial
    Polysize32 = 0,
    ///1: 16-bit polynomial
    Polysize16 = 1,
    ///2: 8-bit polynomial
    Polysize8 = 2,
    ///3: 7-bit polynomial
    Polysize7 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
impl POLYSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::Polysize32,
            1 => POLYSIZE_A::Polysize16,
            2 => POLYSIZE_A::Polysize8,
            3 => POLYSIZE_A::Polysize7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Polysize32`
    #[inline(always)]
    pub fn is_polysize32(&self) -> bool {
        *self == POLYSIZE_A::Polysize32
    }
    ///Checks if the value of the field is `Polysize16`
    #[inline(always)]
    pub fn is_polysize16(&self) -> bool {
        *self == POLYSIZE_A::Polysize16
    }
    ///Checks if the value of the field is `Polysize8`
    #[inline(always)]
    pub fn is_polysize8(&self) -> bool {
        *self == POLYSIZE_A::Polysize8
    }
    ///Checks if the value of the field is `Polysize7`
    #[inline(always)]
    pub fn is_polysize7(&self) -> bool {
        *self == POLYSIZE_A::Polysize7
    }
}
///Field `POLYSIZE` writer - Polynomial size
pub type POLYSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, POLYSIZE_A, 2, O>;
impl<'a, const O: u8> POLYSIZE_W<'a, O> {
    ///32-bit polynomial
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_A::Polysize32)
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_A::Polysize16)
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_A::Polysize8)
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_A::Polysize7)
    }
}
///Field `REV_IN` reader - Reverse input data
pub type REV_IN_R = crate::FieldReader<u8, REV_IN_A>;
///Reverse input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN_A {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversal done by byte
    Byte = 1,
    ///2: Bit reversal done by half-word
    HalfWord = 2,
    ///3: Bit reversal done by word
    Word = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
impl REV_IN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::Normal,
            1 => REV_IN_A::Byte,
            2 => REV_IN_A::HalfWord,
            3 => REV_IN_A::Word,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_IN_A::Normal
    }
    ///Checks if the value of the field is `Byte`
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == REV_IN_A::Byte
    }
    ///Checks if the value of the field is `HalfWord`
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == REV_IN_A::HalfWord
    }
    ///Checks if the value of the field is `Word`
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == REV_IN_A::Word
    }
}
///Field `REV_IN` writer - Reverse input data
pub type REV_IN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, REV_IN_A, 2, O>;
impl<'a, const O: u8> REV_IN_W<'a, O> {
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_IN_A::Normal)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_IN_A::Byte)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_IN_A::HalfWord)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_IN_A::Word)
    }
}
///Field `REV_OUT` reader - Reverse output data
pub type REV_OUT_R = crate::BitReader<REV_OUT_A>;
///Reverse output data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT_A {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversed output
    Reversed = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REV_OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::Normal,
            true => REV_OUT_A::Reversed,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_OUT_A::Normal
    }
    ///Checks if the value of the field is `Reversed`
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == REV_OUT_A::Reversed
    }
}
///Field `REV_OUT` writer - Reverse output data
pub type REV_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, REV_OUT_A, O>;
impl<'a, const O: u8> REV_OUT_W<'a, O> {
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_OUT_A::Normal)
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_OUT_A::Reversed)
    }
}
impl R {
    ///Bit 0 - RESET bit
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Reverse output data
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RESET bit
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    ///Bits 3:4 - Polynomial size
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<3> {
        POLYSIZE_W::new(self)
    }
    ///Bits 5:6 - Reverse input data
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<5> {
        REV_IN_W::new(self)
    }
    ///Bit 7 - Reverse output data
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<7> {
        REV_OUT_W::new(self)
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
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
