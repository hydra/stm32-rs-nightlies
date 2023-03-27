///Register `DDRPHYC_DSGCR` reader
pub struct R(crate::R<DDRPHYC_DSGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DSGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DSGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DSGCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_DSGCR` writer
pub struct W(crate::W<DDRPHYC_DSGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DSGCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DSGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DSGCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PUREN` reader - PUREN
pub type PUREN_R = crate::BitReader<bool>;
///Field `PUREN` writer - PUREN
pub type PUREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `BDISEN` reader - BDISEN
pub type BDISEN_R = crate::BitReader<bool>;
///Field `BDISEN` writer - BDISEN
pub type BDISEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `ZUEN` reader - ZUEN
pub type ZUEN_R = crate::BitReader<bool>;
///Field `ZUEN` writer - ZUEN
pub type ZUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `LPIOPD` reader - LPIOPD
pub type LPIOPD_R = crate::BitReader<bool>;
///Field `LPIOPD` writer - LPIOPD
pub type LPIOPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `LPDLLPD` reader - LPDLLPD
pub type LPDLLPD_R = crate::BitReader<bool>;
///Field `LPDLLPD` writer - LPDLLPD
pub type LPDLLPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `DQSGX` reader - DQSGX
pub type DQSGX_R = crate::FieldReader<u8, u8>;
///Field `DQSGX` writer - DQSGX
pub type DQSGX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DSGCR_SPEC, u8, u8, 3, O>;
///Field `DQSGE` reader - DQSGE
pub type DQSGE_R = crate::FieldReader<u8, u8>;
///Field `DQSGE` writer - DQSGE
pub type DQSGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DSGCR_SPEC, u8, u8, 3, O>;
///Field `NOBUB` reader - NOBUB
pub type NOBUB_R = crate::BitReader<bool>;
///Field `NOBUB` writer - NOBUB
pub type NOBUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `FXDLAT` reader - FXDLAT
pub type FXDLAT_R = crate::BitReader<bool>;
///Field `FXDLAT` writer - FXDLAT
pub type FXDLAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `CKEPDD` reader - CKEPDD
pub type CKEPDD_R = crate::BitReader<bool>;
///Field `CKEPDD` writer - CKEPDD
pub type CKEPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `ODTPDD` reader - ODTPDD
pub type ODTPDD_R = crate::BitReader<bool>;
///Field `ODTPDD` writer - ODTPDD
pub type ODTPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `NL2PD` reader - NL2PD
pub type NL2PD_R = crate::BitReader<bool>;
///Field `NL2PD` writer - NL2PD
pub type NL2PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `NL2OE` reader - NL2OE
pub type NL2OE_R = crate::BitReader<bool>;
///Field `NL2OE` writer - NL2OE
pub type NL2OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `TPDPD` reader - TPDPD
pub type TPDPD_R = crate::BitReader<bool>;
///Field `TPDPD` writer - TPDPD
pub type TPDPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `TPDOE` reader - TPDOE
pub type TPDOE_R = crate::BitReader<bool>;
///Field `TPDOE` writer - TPDOE
pub type TPDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `CKOE` reader - CKOE
pub type CKOE_R = crate::BitReader<bool>;
///Field `CKOE` writer - CKOE
pub type CKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `ODTOE` reader - ODTOE
pub type ODTOE_R = crate::BitReader<bool>;
///Field `ODTOE` writer - ODTOE
pub type ODTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `RSTOE` reader - RSTOE
pub type RSTOE_R = crate::BitReader<bool>;
///Field `RSTOE` writer - RSTOE
pub type RSTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
///Field `CKEOE` reader - CKEOE
pub type CKEOE_R = crate::BitReader<bool>;
///Field `CKEOE` writer - CKEOE
pub type CKEOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DSGCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - PUREN
    #[inline(always)]
    pub fn puren(&self) -> PUREN_R {
        PUREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BDISEN
    #[inline(always)]
    pub fn bdisen(&self) -> BDISEN_R {
        BDISEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ZUEN
    #[inline(always)]
    pub fn zuen(&self) -> ZUEN_R {
        ZUEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPIOPD
    #[inline(always)]
    pub fn lpiopd(&self) -> LPIOPD_R {
        LPIOPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LPDLLPD
    #[inline(always)]
    pub fn lpdllpd(&self) -> LPDLLPD_R {
        LPDLLPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - DQSGX
    #[inline(always)]
    pub fn dqsgx(&self) -> DQSGX_R {
        DQSGX_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - DQSGE
    #[inline(always)]
    pub fn dqsge(&self) -> DQSGE_R {
        DQSGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - NOBUB
    #[inline(always)]
    pub fn nobub(&self) -> NOBUB_R {
        NOBUB_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - FXDLAT
    #[inline(always)]
    pub fn fxdlat(&self) -> FXDLAT_R {
        FXDLAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - CKEPDD
    #[inline(always)]
    pub fn ckepdd(&self) -> CKEPDD_R {
        CKEPDD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - ODTPDD
    #[inline(always)]
    pub fn odtpdd(&self) -> ODTPDD_R {
        ODTPDD_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - NL2PD
    #[inline(always)]
    pub fn nl2pd(&self) -> NL2PD_R {
        NL2PD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - NL2OE
    #[inline(always)]
    pub fn nl2oe(&self) -> NL2OE_R {
        NL2OE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TPDPD
    #[inline(always)]
    pub fn tpdpd(&self) -> TPDPD_R {
        TPDPD_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TPDOE
    #[inline(always)]
    pub fn tpdoe(&self) -> TPDOE_R {
        TPDOE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CKOE
    #[inline(always)]
    pub fn ckoe(&self) -> CKOE_R {
        CKOE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ODTOE
    #[inline(always)]
    pub fn odtoe(&self) -> ODTOE_R {
        ODTOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - RSTOE
    #[inline(always)]
    pub fn rstoe(&self) -> RSTOE_R {
        RSTOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CKEOE
    #[inline(always)]
    pub fn ckeoe(&self) -> CKEOE_R {
        CKEOE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PUREN
    #[inline(always)]
    #[must_use]
    pub fn puren(&mut self) -> PUREN_W<0> {
        PUREN_W::new(self)
    }
    ///Bit 1 - BDISEN
    #[inline(always)]
    #[must_use]
    pub fn bdisen(&mut self) -> BDISEN_W<1> {
        BDISEN_W::new(self)
    }
    ///Bit 2 - ZUEN
    #[inline(always)]
    #[must_use]
    pub fn zuen(&mut self) -> ZUEN_W<2> {
        ZUEN_W::new(self)
    }
    ///Bit 3 - LPIOPD
    #[inline(always)]
    #[must_use]
    pub fn lpiopd(&mut self) -> LPIOPD_W<3> {
        LPIOPD_W::new(self)
    }
    ///Bit 4 - LPDLLPD
    #[inline(always)]
    #[must_use]
    pub fn lpdllpd(&mut self) -> LPDLLPD_W<4> {
        LPDLLPD_W::new(self)
    }
    ///Bits 5:7 - DQSGX
    #[inline(always)]
    #[must_use]
    pub fn dqsgx(&mut self) -> DQSGX_W<5> {
        DQSGX_W::new(self)
    }
    ///Bits 8:10 - DQSGE
    #[inline(always)]
    #[must_use]
    pub fn dqsge(&mut self) -> DQSGE_W<8> {
        DQSGE_W::new(self)
    }
    ///Bit 11 - NOBUB
    #[inline(always)]
    #[must_use]
    pub fn nobub(&mut self) -> NOBUB_W<11> {
        NOBUB_W::new(self)
    }
    ///Bit 12 - FXDLAT
    #[inline(always)]
    #[must_use]
    pub fn fxdlat(&mut self) -> FXDLAT_W<12> {
        FXDLAT_W::new(self)
    }
    ///Bit 16 - CKEPDD
    #[inline(always)]
    #[must_use]
    pub fn ckepdd(&mut self) -> CKEPDD_W<16> {
        CKEPDD_W::new(self)
    }
    ///Bit 20 - ODTPDD
    #[inline(always)]
    #[must_use]
    pub fn odtpdd(&mut self) -> ODTPDD_W<20> {
        ODTPDD_W::new(self)
    }
    ///Bit 24 - NL2PD
    #[inline(always)]
    #[must_use]
    pub fn nl2pd(&mut self) -> NL2PD_W<24> {
        NL2PD_W::new(self)
    }
    ///Bit 25 - NL2OE
    #[inline(always)]
    #[must_use]
    pub fn nl2oe(&mut self) -> NL2OE_W<25> {
        NL2OE_W::new(self)
    }
    ///Bit 26 - TPDPD
    #[inline(always)]
    #[must_use]
    pub fn tpdpd(&mut self) -> TPDPD_W<26> {
        TPDPD_W::new(self)
    }
    ///Bit 27 - TPDOE
    #[inline(always)]
    #[must_use]
    pub fn tpdoe(&mut self) -> TPDOE_W<27> {
        TPDOE_W::new(self)
    }
    ///Bit 28 - CKOE
    #[inline(always)]
    #[must_use]
    pub fn ckoe(&mut self) -> CKOE_W<28> {
        CKOE_W::new(self)
    }
    ///Bit 29 - ODTOE
    #[inline(always)]
    #[must_use]
    pub fn odtoe(&mut self) -> ODTOE_W<29> {
        ODTOE_W::new(self)
    }
    ///Bit 30 - RSTOE
    #[inline(always)]
    #[must_use]
    pub fn rstoe(&mut self) -> RSTOE_W<30> {
        RSTOE_W::new(self)
    }
    ///Bit 31 - CKEOE
    #[inline(always)]
    #[must_use]
    pub fn ckeoe(&mut self) -> CKEOE_W<31> {
        CKEOE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC DSGC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dsgcr](index.html) module
pub struct DDRPHYC_DSGCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DSGCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dsgcr::R](R) reader structure
impl crate::Readable for DDRPHYC_DSGCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_dsgcr::W](W) writer structure
impl crate::Writable for DDRPHYC_DSGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_DSGCR to value 0xfa00_001f
impl crate::Resettable for DDRPHYC_DSGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa00_001f;
}
