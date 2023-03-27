///Register `PCR` reader
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCR` writer
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PWAITEN` reader - PWAITEN
pub type PWAITEN_R = crate::BitReader<bool>;
///Field `PWAITEN` writer - PWAITEN
pub type PWAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `PBKEN` reader - PBKEN
pub type PBKEN_R = crate::BitReader<bool>;
///Field `PBKEN` writer - PBKEN
pub type PBKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `PTYP` reader - PTYP
pub type PTYP_R = crate::BitReader<bool>;
///Field `PTYP` writer - PTYP
pub type PTYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `PWID` reader - PWID
pub type PWID_R = crate::FieldReader<u8, u8>;
///Field `PWID` writer - PWID
pub type PWID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, u8, 2, O>;
///Field `ECCEN` reader - ECCEN
pub type ECCEN_R = crate::BitReader<bool>;
///Field `ECCEN` writer - ECCEN
pub type ECCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `TCLR` reader - TCLR
pub type TCLR_R = crate::FieldReader<u8, u8>;
///Field `TCLR` writer - TCLR
pub type TCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, u8, 4, O>;
///Field `TAR` reader - TAR
pub type TAR_R = crate::FieldReader<u8, u8>;
///Field `TAR` writer - TAR
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, u8, 4, O>;
///Field `ECCPS` reader - ECCPS
pub type ECCPS_R = crate::FieldReader<u8, u8>;
///Field `ECCPS` writer - ECCPS
pub type ECCPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 1 - PWAITEN
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    ///Bit 1 - PWAITEN
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<1> {
        PWAITEN_W::new(self)
    }
    ///Bit 2 - PBKEN
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<2> {
        PBKEN_W::new(self)
    }
    ///Bit 3 - PTYP
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<3> {
        PTYP_W::new(self)
    }
    ///Bits 4:5 - PWID
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<4> {
        PWID_W::new(self)
    }
    ///Bit 6 - ECCEN
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<6> {
        ECCEN_W::new(self)
    }
    ///Bits 9:12 - TCLR
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<9> {
        TCLR_W::new(self)
    }
    ///Bits 13:16 - TAR
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<13> {
        TAR_W::new(self)
    }
    ///Bits 17:19 - ECCPS
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<17> {
        ECCPS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PC Card/NAND Flash control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcr](index.html) module
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcr::R](R) reader structure
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcr::W](W) writer structure
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCR to value 0x18
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
