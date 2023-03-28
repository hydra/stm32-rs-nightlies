///Register `AHBRSTR` reader
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHBRSTR` writer
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader<OTGFSRST_A>;
///USB OTG FS reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST_A {
    ///1: Reset the selected module
    Reset = 1,
}
impl From<OTGFSRST_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OTGFSRST_A> {
        match self.bits {
            true => Some(OTGFSRST_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST_A::Reset
    }
}
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, OTGFSRST_A, O>;
impl<'a, const O: u8> OTGFSRST_W<'a, O> {
    ///Reset the selected module
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::Reset)
    }
}
///Field `ETHMACRST` reader - Ethernet MAC reset
pub use OTGFSRST_R as ETHMACRST_R;
///Field `ETHMACRST` writer - Ethernet MAC reset
pub use OTGFSRST_W as ETHMACRST_W;
impl R {
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Ethernet MAC reset
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<12> {
        OTGFSRST_W::new(self)
    }
    ///Bit 14 - Ethernet MAC reset
    #[inline(always)]
    #[must_use]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W<14> {
        ETHMACRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB peripheral clock reset register (RCC_AHBRSTR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahbrstr](index.html) module
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahbrstr::R](R) reader structure
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahbrstr::W](W) writer structure
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHBRSTR to value 0
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
