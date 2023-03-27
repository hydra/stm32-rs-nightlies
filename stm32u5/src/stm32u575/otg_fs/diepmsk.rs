///Register `DIEPMSK` reader
pub struct R(crate::R<DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPMSK` writer
pub struct W(crate::W<DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPMSK_SPEC>;
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
impl From<crate::W<DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader<bool>;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `EPDM` reader - EPDM
pub type EPDM_R = crate::BitReader<bool>;
///Field `EPDM` writer - EPDM
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `TOM` reader - TOM
pub type TOM_R = crate::BitReader<bool>;
///Field `TOM` writer - TOM
pub type TOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `ITTXFEMSK` reader - ITTXFEMSK
pub type ITTXFEMSK_R = crate::BitReader<bool>;
///Field `ITTXFEMSK` writer - ITTXFEMSK
pub type ITTXFEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `INEPNMM` reader - INEPNMM
pub type INEPNMM_R = crate::BitReader<bool>;
///Field `INEPNMM` writer - INEPNMM
pub type INEPNMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `INEPNEM` reader - INEPNEM
pub type INEPNEM_R = crate::BitReader<bool>;
///Field `INEPNEM` writer - INEPNEM
pub type INEPNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
///Field `NAKM` reader - NAKM
pub type NAKM_R = crate::BitReader<bool>;
///Field `NAKM` writer - NAKM
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPMSK_SPEC, bool, O>;
impl R {
    ///Bit 0 - XFRCM
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDM
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - TOM
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ITTXFEMSK
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - INEPNMM
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - INEPNEM
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - NAKM
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - XFRCM
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    ///Bit 1 - EPDM
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    ///Bit 3 - TOM
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<3> {
        TOM_W::new(self)
    }
    ///Bit 4 - ITTXFEMSK
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<4> {
        ITTXFEMSK_W::new(self)
    }
    ///Bit 5 - INEPNMM
    #[inline(always)]
    #[must_use]
    pub fn inepnmm(&mut self) -> INEPNMM_W<5> {
        INEPNMM_W::new(self)
    }
    ///Bit 6 - INEPNEM
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<6> {
        INEPNEM_W::new(self)
    }
    ///Bit 13 - NAKM
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<13> {
        NAKM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepmsk](index.html) module
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [diepmsk::R](R) reader structure
impl crate::Readable for DIEPMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [diepmsk::W](W) writer structure
impl crate::Writable for DIEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPMSK to value 0
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
