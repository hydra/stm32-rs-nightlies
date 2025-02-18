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
///RESET bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW_AW {
    ///1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    Reset = 1,
}
impl From<RESETW_AW> for bool {
    #[inline(always)]
    fn from(variant: RESETW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` writer - RESET bit
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RESETW_AW, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    ///Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW_AW::Reset)
    }
}
///Polynomial size
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE_AW {
    ///0: 32-bit polynomial
    Polysize32 = 0,
    ///1: 16-bit polynomial
    Polysize16 = 1,
    ///2: 8-bit polynomial
    Polysize8 = 2,
    ///3: 7-bit polynomial
    Polysize7 = 3,
}
impl From<POLYSIZE_AW> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_AW) -> Self {
        variant as _
    }
}
///Field `POLYSIZE` writer - Polynomial size
pub type POLYSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, POLYSIZE_AW, 2, O>;
impl<'a, const O: u8> POLYSIZE_W<'a, O> {
    ///32-bit polynomial
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::Polysize32)
    }
    ///16-bit polynomial
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::Polysize16)
    }
    ///8-bit polynomial
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::Polysize8)
    }
    ///7-bit polynomial
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_AW::Polysize7)
    }
}
///Reverse input data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN_AW {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversal done by byte
    Byte = 1,
    ///2: Bit reversal done by half-word
    HalfWord = 2,
    ///3: Bit reversal done by word
    Word = 3,
}
impl From<REV_IN_AW> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_AW) -> Self {
        variant as _
    }
}
///Field `REV_IN` writer - Reverse input data
pub type REV_IN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, REV_IN_AW, 2, O>;
impl<'a, const O: u8> REV_IN_W<'a, O> {
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_IN_AW::Normal)
    }
    ///Bit reversal done by byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_IN_AW::Byte)
    }
    ///Bit reversal done by half-word
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_IN_AW::HalfWord)
    }
    ///Bit reversal done by word
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_IN_AW::Word)
    }
}
///Reverse output data
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT_AW {
    ///0: Bit order not affected
    Normal = 0,
    ///1: Bit reversed output
    Reversed = 1,
}
impl From<REV_OUT_AW> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `REV_OUT` writer - Reverse output data
pub type REV_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, REV_OUT_AW, O>;
impl<'a, const O: u8> REV_OUT_W<'a, O> {
    ///Bit order not affected
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_OUT_AW::Normal)
    }
    ///Bit reversed output
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_OUT_AW::Reversed)
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
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
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
