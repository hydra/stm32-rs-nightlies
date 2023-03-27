///Register `USBPHYC_TUNE1` reader
pub struct R(crate::R<USBPHYC_TUNE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHYC_TUNE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHYC_TUNE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHYC_TUNE1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USBPHYC_TUNE1` writer
pub struct W(crate::W<USBPHYC_TUNE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHYC_TUNE1_SPEC>;
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
impl From<crate::W<USBPHYC_TUNE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHYC_TUNE1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INCURREN` reader - INCURREN
pub type INCURREN_R = crate::BitReader<bool>;
///Field `INCURREN` writer - INCURREN
pub type INCURREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `INCURRINT` reader - INCURRINT
pub type INCURRINT_R = crate::BitReader<bool>;
///Field `INCURRINT` writer - INCURRINT
pub type INCURRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `LFSCAPEN` reader - LFSCAPEN
pub type LFSCAPEN_R = crate::BitReader<bool>;
///Field `LFSCAPEN` writer - LFSCAPEN
pub type LFSCAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVSLEW` reader - HSDRVSLEW
pub type HSDRVSLEW_R = crate::BitReader<bool>;
///Field `HSDRVSLEW` writer - HSDRVSLEW
pub type HSDRVSLEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVDCCUR` reader - HSDRVDCCUR
pub type HSDRVDCCUR_R = crate::BitReader<bool>;
///Field `HSDRVDCCUR` writer - HSDRVDCCUR
pub type HSDRVDCCUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVDCLEV` reader - HSDRVDCLEV
pub type HSDRVDCLEV_R = crate::BitReader<bool>;
///Field `HSDRVDCLEV` writer - HSDRVDCLEV
pub type HSDRVDCLEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVCURINCR` reader - HSDRVCURINCR
pub type HSDRVCURINCR_R = crate::BitReader<bool>;
///Field `HSDRVCURINCR` writer - HSDRVCURINCR
pub type HSDRVCURINCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `FSDRVRFADJ` reader - FSDRVRFADJ
pub type FSDRVRFADJ_R = crate::BitReader<bool>;
///Field `FSDRVRFADJ` writer - FSDRVRFADJ
pub type FSDRVRFADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVRFRED` reader - HSDRVRFRED
pub type HSDRVRFRED_R = crate::BitReader<bool>;
///Field `HSDRVRFRED` writer - HSDRVRFRED
pub type HSDRVRFRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSDRVCHKITRM` reader - HSDRVCHKITRM
pub type HSDRVCHKITRM_R = crate::FieldReader<u8, u8>;
///Field `HSDRVCHKITRM` writer - HSDRVCHKITRM
pub type HSDRVCHKITRM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 4, O>;
///Field `HSDRVCHKZTRM` reader - HSDRVCHKZTRM
pub type HSDRVCHKZTRM_R = crate::FieldReader<u8, u8>;
///Field `HSDRVCHKZTRM` writer - HSDRVCHKZTRM
pub type HSDRVCHKZTRM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
///Field `OTPCOMP` reader - OTPCOMP
pub type OTPCOMP_R = crate::FieldReader<u8, u8>;
///Field `OTPCOMP` writer - OTPCOMP
pub type OTPCOMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 5, O>;
///Field `SQLCHCTL` reader - SQLCHCTL
pub type SQLCHCTL_R = crate::FieldReader<u8, u8>;
///Field `SQLCHCTL` writer - SQLCHCTL
pub type SQLCHCTL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
///Field `HDRXGNEQEN` reader - HDRXGNEQEN
pub type HDRXGNEQEN_R = crate::BitReader<bool>;
///Field `HDRXGNEQEN` writer - HDRXGNEQEN
pub type HDRXGNEQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `HSRXOFF` reader - HSRXOFF
pub type HSRXOFF_R = crate::FieldReader<u8, u8>;
///Field `HSRXOFF` writer - HSRXOFF
pub type HSRXOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBPHYC_TUNE1_SPEC, u8, u8, 2, O>;
///Field `HSFALLPREEM` reader - HSFALLPREEM
pub type HSFALLPREEM_R = crate::BitReader<bool>;
///Field `HSFALLPREEM` writer - HSFALLPREEM
pub type HSFALLPREEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `SHTCCTCTLPROT` reader - SHTCCTCTLPROT
pub type SHTCCTCTLPROT_R = crate::BitReader<bool>;
///Field `SHTCCTCTLPROT` writer - SHTCCTCTLPROT
pub type SHTCCTCTLPROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
///Field `STAGSEL` reader - STAGSEL
pub type STAGSEL_R = crate::BitReader<bool>;
///Field `STAGSEL` writer - STAGSEL
pub type STAGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBPHYC_TUNE1_SPEC, bool, O>;
impl R {
    ///Bit 0 - INCURREN
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - INCURRINT
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LFSCAPEN
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSDRVSLEW
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSDRVDCCUR
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSDRVDCLEV
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSDRVCURINCR
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FSDRVRFADJ
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSDRVRFRED
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - HSDRVCHKITRM
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:14 - HSDRVCHKZTRM
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bits 15:19 - OTPCOMP
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:21 - SQLCHCTL
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - HDRXGNEQEN
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:24 - HSRXOFF
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bit 25 - HSFALLPREEM
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SHTCCTCTLPROT
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - STAGSEL
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - INCURREN
    #[inline(always)]
    #[must_use]
    pub fn incurren(&mut self) -> INCURREN_W<0> {
        INCURREN_W::new(self)
    }
    ///Bit 1 - INCURRINT
    #[inline(always)]
    #[must_use]
    pub fn incurrint(&mut self) -> INCURRINT_W<1> {
        INCURRINT_W::new(self)
    }
    ///Bit 2 - LFSCAPEN
    #[inline(always)]
    #[must_use]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<2> {
        LFSCAPEN_W::new(self)
    }
    ///Bit 3 - HSDRVSLEW
    #[inline(always)]
    #[must_use]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<3> {
        HSDRVSLEW_W::new(self)
    }
    ///Bit 4 - HSDRVDCCUR
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<4> {
        HSDRVDCCUR_W::new(self)
    }
    ///Bit 5 - HSDRVDCLEV
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<5> {
        HSDRVDCLEV_W::new(self)
    }
    ///Bit 6 - HSDRVCURINCR
    #[inline(always)]
    #[must_use]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<6> {
        HSDRVCURINCR_W::new(self)
    }
    ///Bit 7 - FSDRVRFADJ
    #[inline(always)]
    #[must_use]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<7> {
        FSDRVRFADJ_W::new(self)
    }
    ///Bit 8 - HSDRVRFRED
    #[inline(always)]
    #[must_use]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<8> {
        HSDRVRFRED_W::new(self)
    }
    ///Bits 9:12 - HSDRVCHKITRM
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<9> {
        HSDRVCHKITRM_W::new(self)
    }
    ///Bits 13:14 - HSDRVCHKZTRM
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<13> {
        HSDRVCHKZTRM_W::new(self)
    }
    ///Bits 15:19 - OTPCOMP
    #[inline(always)]
    #[must_use]
    pub fn otpcomp(&mut self) -> OTPCOMP_W<15> {
        OTPCOMP_W::new(self)
    }
    ///Bits 20:21 - SQLCHCTL
    #[inline(always)]
    #[must_use]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<20> {
        SQLCHCTL_W::new(self)
    }
    ///Bit 22 - HDRXGNEQEN
    #[inline(always)]
    #[must_use]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<22> {
        HDRXGNEQEN_W::new(self)
    }
    ///Bits 23:24 - HSRXOFF
    #[inline(always)]
    #[must_use]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<23> {
        HSRXOFF_W::new(self)
    }
    ///Bit 25 - HSFALLPREEM
    #[inline(always)]
    #[must_use]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<25> {
        HSFALLPREEM_W::new(self)
    }
    ///Bit 26 - SHTCCTCTLPROT
    #[inline(always)]
    #[must_use]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<26> {
        SHTCCTCTLPROT_W::new(self)
    }
    ///Bit 27 - STAGSEL
    #[inline(always)]
    #[must_use]
    pub fn stagsel(&mut self) -> STAGSEL_W<27> {
        STAGSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the tune interface of the HS PHY, port #x.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [usbphyc_tune1](index.html) module
pub struct USBPHYC_TUNE1_SPEC;
impl crate::RegisterSpec for USBPHYC_TUNE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [usbphyc_tune1::R](R) reader structure
impl crate::Readable for USBPHYC_TUNE1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [usbphyc_tune1::W](W) writer structure
impl crate::Writable for USBPHYC_TUNE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets USBPHYC_TUNE1 to value 0x0407_0004
impl crate::Resettable for USBPHYC_TUNE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0407_0004;
}
