///Register `OTG_DOEPINT7` reader
pub struct R(crate::R<OTG_DOEPINT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPINT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPINT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPINT7_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_DOEPINT7` writer
pub struct W(crate::W<OTG_DOEPINT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPINT7_SPEC>;
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
impl From<crate::W<OTG_DOEPINT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPINT7_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader<bool>;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader<bool>;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `AHBERR` reader - AHBERR
pub type AHBERR_R = crate::BitReader<bool>;
///Field `AHBERR` writer - AHBERR
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `STUP` reader - STUP
pub type STUP_R = crate::BitReader<bool>;
///Field `STUP` writer - STUP
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `OTEPDIS` reader - OTEPDIS
pub type OTEPDIS_R = crate::BitReader<bool>;
///Field `OTEPDIS` writer - OTEPDIS
pub type OTEPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `STSPHSRX` reader - STSPHSRX
pub type STSPHSRX_R = crate::BitReader<bool>;
///Field `STSPHSRX` writer - STSPHSRX
pub type STSPHSRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `B2BSTUP` reader - B2BSTUP
pub type B2BSTUP_R = crate::BitReader<bool>;
///Field `B2BSTUP` writer - B2BSTUP
pub type B2BSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `OUTPKTERR` reader - OUTPKTERR
pub type OUTPKTERR_R = crate::BitReader<bool>;
///Field `OUTPKTERR` writer - OUTPKTERR
pub type OUTPKTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `BNA` reader - BNA
pub type BNA_R = crate::BitReader<bool>;
///Field `BNA` writer - BNA
pub type BNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `BERR` reader - BERR
pub type BERR_R = crate::BitReader<bool>;
///Field `BERR` writer - BERR
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader<bool>;
///Field `NAK` writer - NAK
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `NYET` reader - NYET
pub type NYET_R = crate::BitReader<bool>;
///Field `NYET` writer - NYET
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
///Field `STPKTRX` reader - STPKTRX
pub type STPKTRX_R = crate::BitReader<bool>;
///Field `STPKTRX` writer - STPKTRX
pub type STPKTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPINT7_SPEC, bool, O>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBERR
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STSPHSRX
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - OUTPKTERR
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - BERR
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - NYET
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - STPKTRX
    #[inline(always)]
    pub fn stpktrx(&self) -> STPKTRX_R {
        STPKTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    ///Bit 2 - AHBERR
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<3> {
        STUP_W::new(self)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<4> {
        OTEPDIS_W::new(self)
    }
    ///Bit 5 - STSPHSRX
    #[inline(always)]
    #[must_use]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<5> {
        STSPHSRX_W::new(self)
    }
    ///Bit 6 - B2BSTUP
    #[inline(always)]
    #[must_use]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<6> {
        B2BSTUP_W::new(self)
    }
    ///Bit 8 - OUTPKTERR
    #[inline(always)]
    #[must_use]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<8> {
        OUTPKTERR_W::new(self)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<9> {
        BNA_W::new(self)
    }
    ///Bit 12 - BERR
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<12> {
        BERR_W::new(self)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<13> {
        NAK_W::new(self)
    }
    ///Bit 14 - NYET
    #[inline(always)]
    #[must_use]
    pub fn nyet(&mut self) -> NYET_W<14> {
        NYET_W::new(self)
    }
    ///Bit 15 - STPKTRX
    #[inline(always)]
    #[must_use]
    pub fn stpktrx(&mut self) -> STPKTRX_W<15> {
        STPKTRX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_doepint7](index.html) module
pub struct OTG_DOEPINT7_SPEC;
impl crate::RegisterSpec for OTG_DOEPINT7_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_doepint7::R](R) reader structure
impl crate::Readable for OTG_DOEPINT7_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_doepint7::W](W) writer structure
impl crate::Writable for OTG_DOEPINT7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_DOEPINT7 to value 0x80
impl crate::Resettable for OTG_DOEPINT7_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
