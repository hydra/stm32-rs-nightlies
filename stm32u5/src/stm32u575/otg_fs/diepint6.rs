///Register `DIEPINT6` reader
pub struct R(crate::R<DIEPINT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPINT6` writer
pub struct W(crate::W<DIEPINT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT6_SPEC>;
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
impl From<crate::W<DIEPINT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader<bool>;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader<bool>;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `AHBERR` reader - AHBERR
pub type AHBERR_R = crate::BitReader<bool>;
///Field `AHBERR` writer - AHBERR
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `TOC` reader - TOC
pub type TOC_R = crate::BitReader<bool>;
///Field `TOC` writer - TOC
pub type TOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `ITTXFE` reader - ITTXFE
pub type ITTXFE_R = crate::BitReader<bool>;
///Field `ITTXFE` writer - ITTXFE
pub type ITTXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `INEPNM` reader - INEPNM
pub type INEPNM_R = crate::BitReader<bool>;
///Field `INEPNM` writer - INEPNM
pub type INEPNM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `INEPNE` reader - INEPNE
pub type INEPNE_R = crate::BitReader<bool>;
///Field `TXFE` reader - TXFE
pub type TXFE_R = crate::BitReader<bool>;
///Field `TXFIFOUDRN` reader - TXFIFOUDRN
pub type TXFIFOUDRN_R = crate::BitReader<bool>;
///Field `TXFIFOUDRN` writer - TXFIFOUDRN
pub type TXFIFOUDRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `BNA` reader - BNA
pub type BNA_R = crate::BitReader<bool>;
///Field `BNA` writer - BNA
pub type BNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `PKTDRPSTS` reader - PKTDRPSTS
pub type PKTDRPSTS_R = crate::BitReader<bool>;
///Field `PKTDRPSTS` writer - PKTDRPSTS
pub type PKTDRPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
///Field `NAK` reader - NAK
pub type NAK_R = crate::BitReader<bool>;
///Field `NAK` writer - NAK
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT6_SPEC, bool, O>;
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
    ///Bit 3 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - INEPNM
    #[inline(always)]
    pub fn inepnm(&self) -> INEPNM_R {
        INEPNM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - INEPNE
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFE
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TXFIFOUDRN
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - PKTDRPSTS
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 3 - TOC
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<3> {
        TOC_W::new(self)
    }
    ///Bit 4 - ITTXFE
    #[inline(always)]
    #[must_use]
    pub fn ittxfe(&mut self) -> ITTXFE_W<4> {
        ITTXFE_W::new(self)
    }
    ///Bit 5 - INEPNM
    #[inline(always)]
    #[must_use]
    pub fn inepnm(&mut self) -> INEPNM_W<5> {
        INEPNM_W::new(self)
    }
    ///Bit 8 - TXFIFOUDRN
    #[inline(always)]
    #[must_use]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<8> {
        TXFIFOUDRN_W::new(self)
    }
    ///Bit 9 - BNA
    #[inline(always)]
    #[must_use]
    pub fn bna(&mut self) -> BNA_W<9> {
        BNA_W::new(self)
    }
    ///Bit 11 - PKTDRPSTS
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<11> {
        PKTDRPSTS_W::new(self)
    }
    ///Bit 13 - NAK
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<13> {
        NAK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepint6](index.html) module
pub struct DIEPINT6_SPEC;
impl crate::RegisterSpec for DIEPINT6_SPEC {
    type Ux = u32;
}
///`read()` method returns [diepint6::R](R) reader structure
impl crate::Readable for DIEPINT6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [diepint6::W](W) writer structure
impl crate::Writable for DIEPINT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPINT6 to value 0x80
impl crate::Resettable for DIEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
