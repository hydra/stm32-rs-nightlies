///Register `DOEPMSK` reader
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOEPMSK` writer
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
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
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRCM` reader - XFRCM
pub type XFRCM_R = crate::BitReader<bool>;
///Field `XFRCM` writer - XFRCM
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `EPDM` reader - EPDM
pub type EPDM_R = crate::BitReader<bool>;
///Field `EPDM` writer - EPDM
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `STUPM` reader - STUPM
pub type STUPM_R = crate::BitReader<bool>;
///Field `STUPM` writer - STUPM
pub type STUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `OTEPDM` reader - OTEPDM
pub type OTEPDM_R = crate::BitReader<bool>;
///Field `OTEPDM` writer - OTEPDM
pub type OTEPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `STSPHSRXM` reader - STSPHSRXM
pub type STSPHSRXM_R = crate::BitReader<bool>;
///Field `STSPHSRXM` writer - STSPHSRXM
pub type STSPHSRXM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `OUTPKTERRM` reader - OUTPKTERRM
pub type OUTPKTERRM_R = crate::BitReader<bool>;
///Field `OUTPKTERRM` writer - OUTPKTERRM
pub type OUTPKTERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `BERRM` reader - BERRM
pub type BERRM_R = crate::BitReader<bool>;
///Field `BERRM` writer - BERRM
pub type BERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
///Field `NAKMSK` reader - NAKMSK
pub type NAKMSK_R = crate::BitReader<bool>;
///Field `NAKMSK` writer - NAKMSK
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
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
    ///Bit 3 - STUPM
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDM
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STSPHSRXM
    #[inline(always)]
    pub fn stsphsrxm(&self) -> STSPHSRXM_R {
        STSPHSRXM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - OUTPKTERRM
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - BERRM
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAKMSK
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 3 - STUPM
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<3> {
        STUPM_W::new(self)
    }
    ///Bit 4 - OTEPDM
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<4> {
        OTEPDM_W::new(self)
    }
    ///Bit 5 - STSPHSRXM
    #[inline(always)]
    #[must_use]
    pub fn stsphsrxm(&mut self) -> STSPHSRXM_W<5> {
        STSPHSRXM_W::new(self)
    }
    ///Bit 8 - OUTPKTERRM
    #[inline(always)]
    #[must_use]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<8> {
        OUTPKTERRM_W::new(self)
    }
    ///Bit 12 - BERRM
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BERRM_W<12> {
        BERRM_W::new(self)
    }
    ///Bit 13 - NAKMSK
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepmsk](index.html) module
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [doepmsk::R](R) reader structure
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doepmsk::W](W) writer structure
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOEPMSK to value 0
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
