///Register `CTL` reader
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CTL` writer
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader<u16, u16>;
///Field `MPSIZ` writer - MPSIZ
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u16, u16, 11, O>;
///Field `USBAEP` reader - USBAEP
pub type USBAEP_R = crate::BitReader<bool>;
///Field `USBAEP` writer - USBAEP
pub type USBAEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `EONUM_DPID` reader - EONUM/DPID
pub type EONUM_DPID_R = crate::BitReader<bool>;
///Field `NAKSTS` reader - NAKSTS
pub type NAKSTS_R = crate::BitReader<bool>;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader<u8, u8>;
///Field `EPTYP` writer - EPTYP
pub type EPTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader<bool>;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `TXFNUM` reader - TXFNUM
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
///Field `TXFNUM` writer - TXFNUM
pub type TXFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 4, O>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `SD0PID_SEVNFRM` writer - SD0PID/SEVNFRM
pub type SD0PID_SEVNFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `SODDFRM_SD1PID` writer - SODDFRM/SD1PID
pub type SODDFRM_SD1PID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `EPDIS` reader - EPDIS
pub type EPDIS_R = crate::BitReader<bool>;
///Field `EPDIS` writer - EPDIS
pub type EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
///Field `EPENA` reader - EPENA
pub type EPENA_R = crate::BitReader<bool>;
///Field `EPENA` writer - EPENA
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - EONUM/DPID
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAKSTS
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    #[must_use]
    pub fn usbaep(&mut self) -> USBAEP_W<15> {
        USBAEP_W::new(self)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<18> {
        EPTYP_W::new(self)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<22> {
        TXFNUM_W::new(self)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    ///Bit 28 - SD0PID/SEVNFRM
    #[inline(always)]
    #[must_use]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<28> {
        SD0PID_SEVNFRM_W::new(self)
    }
    ///Bit 29 - SODDFRM/SD1PID
    #[inline(always)]
    #[must_use]
    pub fn soddfrm_sd1pid(&mut self) -> SODDFRM_SD1PID_W<29> {
        SODDFRM_SD1PID_W::new(self)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<30> {
        EPDIS_W::new(self)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG device endpoint-1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ctl](index.html) module
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ctl::R](R) reader structure
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ctl::W](W) writer structure
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CTL to value 0
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
