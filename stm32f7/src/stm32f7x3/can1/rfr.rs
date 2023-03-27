///Register `RF%sR` reader
pub struct R(crate::R<RFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RF%sR` writer
pub struct W(crate::W<RFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFR_SPEC>;
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
impl From<crate::W<RFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FMP` reader - FMP0
pub type FMP_R = crate::FieldReader<u8, u8>;
///Field `FULL` reader - FULL0
pub type FULL_R = crate::BitReader<FULL0R_A>;
///FULL0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0R_A {
    ///0: FIFO x is not full
    NotFull = 0,
    ///1: FIFO x is full
    Full = 1,
}
impl From<FULL0R_A> for bool {
    #[inline(always)]
    fn from(variant: FULL0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FULL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FULL0R_A {
        match self.bits {
            false => FULL0R_A::NotFull,
            true => FULL0R_A::Full,
        }
    }
    ///Checks if the value of the field is `NotFull`
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FULL0R_A::NotFull
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FULL0R_A::Full
    }
}
///FULL0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0W_AW {
    ///1: Clear flag
    Clear = 1,
}
impl From<FULL0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FULL0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FULL` writer - FULL0
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, FULL0W_AW, O>;
impl<'a, const O: u8> FULL_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FULL0W_AW::Clear)
    }
}
///Field `FOVR` reader - FOVR0
pub type FOVR_R = crate::BitReader<FOVR0R_A>;
///FOVR0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0R_A {
    ///0: No FIFO x overrun
    NoOverrun = 0,
    ///1: FIFO x overrun
    Overrun = 1,
}
impl From<FOVR0R_A> for bool {
    #[inline(always)]
    fn from(variant: FOVR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FOVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FOVR0R_A {
        match self.bits {
            false => FOVR0R_A::NoOverrun,
            true => FOVR0R_A::Overrun,
        }
    }
    ///Checks if the value of the field is `NoOverrun`
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == FOVR0R_A::NoOverrun
    }
    ///Checks if the value of the field is `Overrun`
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == FOVR0R_A::Overrun
    }
}
///FOVR0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0W_AW {
    ///1: Clear flag
    Clear = 1,
}
impl From<FOVR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FOVR0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FOVR` writer - FOVR0
pub type FOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, FOVR0W_AW, O>;
impl<'a, const O: u8> FOVR_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FOVR0W_AW::Clear)
    }
}
///Field `RFOM` reader - RFOM0
pub type RFOM_R = crate::BitReader<RFOM0W_A>;
///RFOM0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOM0W_A {
    ///1: Set by software to release the output mailbox of the FIFO
    Release = 1,
}
impl From<RFOM0W_A> for bool {
    #[inline(always)]
    fn from(variant: RFOM0W_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RFOM0W_A> {
        match self.bits {
            true => Some(RFOM0W_A::Release),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Release`
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == RFOM0W_A::Release
    }
}
///Field `RFOM` writer - RFOM0
pub type RFOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, RFOM0W_A, O>;
impl<'a, const O: u8> RFOM_W<'a, O> {
    ///Set by software to release the output mailbox of the FIFO
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(RFOM0W_A::Release)
    }
}
impl R {
    ///Bits 0:1 - FMP0
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - FULL0
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - FULL0
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<3> {
        FULL_W::new(self)
    }
    ///Bit 4 - FOVR0
    #[inline(always)]
    #[must_use]
    pub fn fovr(&mut self) -> FOVR_W<4> {
        FOVR_W::new(self)
    }
    ///Bit 5 - RFOM0
    #[inline(always)]
    #[must_use]
    pub fn rfom(&mut self) -> RFOM_W<5> {
        RFOM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///receive FIFO %s register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rfr](index.html) module
pub struct RFR_SPEC;
impl crate::RegisterSpec for RFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rfr::R](R) reader structure
impl crate::Readable for RFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rfr::W](W) writer structure
impl crate::Writable for RFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RF%sR to value 0
impl crate::Resettable for RFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
