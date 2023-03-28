///Register `SDRTR` reader
pub struct R(crate::R<SDRTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDRTR` writer
pub struct W(crate::W<SDRTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRTR_SPEC>;
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
impl From<crate::W<SDRTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRTR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear Refresh error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRE_AW {
    ///1: Refresh Error Flag is cleared
    Clear = 1,
}
impl From<CRE_AW> for bool {
    #[inline(always)]
    fn from(variant: CRE_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CRE` writer - Clear Refresh error flag
pub type CRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRTR_SPEC, CRE_AW, O>;
impl<'a, const O: u8> CRE_W<'a, O> {
    ///Refresh Error Flag is cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRE_AW::Clear)
    }
}
///Field `COUNT` reader - Refresh Timer Count
pub type COUNT_R = crate::FieldReader<u16, u16>;
///Field `COUNT` writer - Refresh Timer Count
pub type COUNT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDRTR_SPEC, u16, u16, 13, O>;
///Field `REIE` reader - RES Interrupt Enable
pub type REIE_R = crate::BitReader<REIE_A>;
///RES Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is generated if RE = 1
    Enabled = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::Disabled,
            true => REIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REIE_A::Enabled
    }
}
///Field `REIE` writer - RES Interrupt Enable
pub type REIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDRTR_SPEC, REIE_A, O>;
impl<'a, const O: u8> REIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REIE_A::Disabled)
    }
    ///Interrupt is generated if RE = 1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REIE_A::Enabled)
    }
}
impl R {
    ///Bits 1:13 - Refresh Timer Count
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Refresh error flag
    #[inline(always)]
    #[must_use]
    pub fn cre(&mut self) -> CRE_W<0> {
        CRE_W::new(self)
    }
    ///Bits 1:13 - Refresh Timer Count
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<1> {
        COUNT_W::new(self)
    }
    ///Bit 14 - RES Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> REIE_W<14> {
        REIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDRAM Refresh Timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdrtr](index.html) module
pub struct SDRTR_SPEC;
impl crate::RegisterSpec for SDRTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdrtr::R](R) reader structure
impl crate::Readable for SDRTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdrtr::W](W) writer structure
impl crate::Writable for SDRTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDRTR to value 0
impl crate::Resettable for SDRTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
