///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear Tamper event
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEW_AW {
    ///1: Reset the TEF Tamper event flag (and the Tamper detector)
    Reset = 1,
}
impl From<CTEW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTE` writer - Clear Tamper event
pub type CTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CTEW_AW, O>;
impl<'a, const O: u8> CTE_W<'a, O> {
    ///Reset the TEF Tamper event flag (and the Tamper detector)
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(CTEW_AW::Reset)
    }
}
///Clear Tamper Interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTIW_AW {
    ///1: Clear the Tamper interrupt and the TIF Tamper interrupt flag
    Clear = 1,
}
impl From<CTIW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTIW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTI` writer - Clear Tamper Interrupt
pub type CTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CTIW_AW, O>;
impl<'a, const O: u8> CTI_W<'a, O> {
    ///Clear the Tamper interrupt and the TIF Tamper interrupt flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTIW_AW::Clear)
    }
}
///Field `TPIE` reader - Tamper Pin interrupt enable
pub type TPIE_R = crate::BitReader<TPIE_A>;
///Tamper Pin interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIE_A {
    ///0: Tamper interrupt disabled
    Disabled = 0,
    ///1: Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
    Enabled = 1,
}
impl From<TPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TPIE_A {
        match self.bits {
            false => TPIE_A::Disabled,
            true => TPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIE_A::Enabled
    }
}
///Field `TPIE` writer - Tamper Pin interrupt enable
pub type TPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TPIE_A, O>;
impl<'a, const O: u8> TPIE_W<'a, O> {
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TPIE_A::Disabled)
    }
    ///Tamper interrupt enabled (the TPE bit must also be set in the BKP_CR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TPIE_A::Enabled)
    }
}
///Field `TEF` reader - Tamper Event Flag
pub type TEF_R = crate::BitReader<bool>;
///Field `TIF` reader - Tamper Interrupt Flag
pub type TIF_R = crate::BitReader<bool>;
impl R {
    ///Bit 2 - Tamper Pin interrupt enable
    #[inline(always)]
    pub fn tpie(&self) -> TPIE_R {
        TPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Tamper Event Flag
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tamper Interrupt Flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Tamper event
    #[inline(always)]
    #[must_use]
    pub fn cte(&mut self) -> CTE_W<0> {
        CTE_W::new(self)
    }
    ///Bit 1 - Clear Tamper Interrupt
    #[inline(always)]
    #[must_use]
    pub fn cti(&mut self) -> CTI_W<1> {
        CTI_W::new(self)
    }
    ///Bit 2 - Tamper Pin interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tpie(&mut self) -> TPIE_W<2> {
        TPIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BKP_CSR control/status register (BKP_CSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
