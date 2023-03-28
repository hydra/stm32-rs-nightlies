///Register `AHB3RSTR` reader
pub struct R(crate::R<AHB3RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3RSTR` writer
pub struct W(crate::W<AHB3RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3RSTR_SPEC>;
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
impl From<crate::W<AHB3RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PKARST` reader - PKARST
pub type PKARST_R = crate::BitReader<PKARST_A>;
///PKARST
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST_A {
    ///0: No effect
    NoReset = 0,
    ///1: Reset peripheral
    Reset = 1,
}
impl From<PKARST_A> for bool {
    #[inline(always)]
    fn from(variant: PKARST_A) -> Self {
        variant as u8 != 0
    }
}
impl PKARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PKARST_A {
        match self.bits {
            false => PKARST_A::NoReset,
            true => PKARST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == PKARST_A::NoReset
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST_A::Reset
    }
}
///Field `PKARST` writer - PKARST
pub type PKARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3RSTR_SPEC, PKARST_A, O>;
impl<'a, const O: u8> PKARST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(PKARST_A::NoReset)
    }
    ///Reset peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PKARST_A::Reset)
    }
}
///Field `AESRST` reader - AESRST
pub use PKARST_R as AESRST_R;
///Field `RNGRST` reader - RNGRST
pub use PKARST_R as RNGRST_R;
///Field `HSEMRST` reader - HSEMRST
pub use PKARST_R as HSEMRST_R;
///Field `IPCCRST` reader - IPCCRST
pub use PKARST_R as IPCCRST_R;
///Field `FLASHRST` reader - Flash interface reset
pub use PKARST_R as FLASHRST_R;
///Field `AESRST` writer - AESRST
pub use PKARST_W as AESRST_W;
///Field `RNGRST` writer - RNGRST
pub use PKARST_W as RNGRST_W;
///Field `HSEMRST` writer - HSEMRST
pub use PKARST_W as HSEMRST_W;
///Field `IPCCRST` writer - IPCCRST
pub use PKARST_W as IPCCRST_W;
///Field `FLASHRST` writer - Flash interface reset
pub use PKARST_W as FLASHRST_W;
impl R {
    ///Bit 16 - PKARST
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    pub fn ipccrst(&self) -> IPCCRST_R {
        IPCCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - PKARST
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<16> {
        PKARST_W::new(self)
    }
    ///Bit 17 - AESRST
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<17> {
        AESRST_W::new(self)
    }
    ///Bit 18 - RNGRST
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    ///Bit 19 - HSEMRST
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<19> {
        HSEMRST_W::new(self)
    }
    ///Bit 20 - IPCCRST
    #[inline(always)]
    #[must_use]
    pub fn ipccrst(&mut self) -> IPCCRST_W<20> {
        IPCCRST_W::new(self)
    }
    ///Bit 25 - Flash interface reset
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<25> {
        FLASHRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB3 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3rstr](index.html) module
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3rstr::R](R) reader structure
impl crate::Readable for AHB3RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3rstr::W](W) writer structure
impl crate::Writable for AHB3RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
