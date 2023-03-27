///Register `MACCR` reader
pub struct R(crate::R<MACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACCR` writer
pub struct W(crate::W<MACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACCR_SPEC>;
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
impl From<crate::W<MACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<bool>;
///Field `RE` writer - Receiver enable
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<bool>;
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `DC` reader - Deferral check
pub type DC_R = crate::BitReader<bool>;
///Field `DC` writer - Deferral check
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `BL` reader - Back-off limit
pub type BL_R = crate::FieldReader<u8, u8>;
///Field `BL` writer - Back-off limit
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 2, O>;
///Field `APCS` reader - Automatic pad/CRC stripping
pub type APCS_R = crate::BitReader<bool>;
///Field `APCS` writer - Automatic pad/CRC stripping
pub type APCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `RD` reader - Retry disable
pub type RD_R = crate::BitReader<bool>;
///Field `RD` writer - Retry disable
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `IPCO` reader - IPv4 checksum offload
pub type IPCO_R = crate::BitReader<bool>;
///Field `IPCO` writer - IPv4 checksum offload
pub type IPCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `DM` reader - Duplex mode
pub type DM_R = crate::BitReader<bool>;
///Field `DM` writer - Duplex mode
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `LM` reader - Loopback mode
pub type LM_R = crate::BitReader<bool>;
///Field `LM` writer - Loopback mode
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `ROD` reader - Receive own disable
pub type ROD_R = crate::BitReader<bool>;
///Field `ROD` writer - Receive own disable
pub type ROD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `FES` reader - Fast Ethernet speed
pub type FES_R = crate::BitReader<bool>;
///Field `FES` writer - Fast Ethernet speed
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `CSD` reader - Carrier sense disable
pub type CSD_R = crate::BitReader<bool>;
///Field `CSD` writer - Carrier sense disable
pub type CSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `IFG` reader - Interframe gap
pub type IFG_R = crate::FieldReader<u8, u8>;
///Field `IFG` writer - Interframe gap
pub type IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, O>;
///Field `JD` reader - Jabber disable
pub type JD_R = crate::BitReader<bool>;
///Field `JD` writer - Jabber disable
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `WD` reader - Watchdog disable
pub type WD_R = crate::BitReader<bool>;
///Field `WD` writer - Watchdog disable
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
impl R {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Receiver enable
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - Deferral check
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    ///Bits 5:6 - Back-off limit
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    ///Bit 7 - Automatic pad/CRC stripping
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<7> {
        APCS_W::new(self)
    }
    ///Bit 9 - Retry disable
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<9> {
        RD_W::new(self)
    }
    ///Bit 10 - IPv4 checksum offload
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<10> {
        IPCO_W::new(self)
    }
    ///Bit 11 - Duplex mode
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    ///Bit 12 - Loopback mode
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    ///Bit 13 - Receive own disable
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<13> {
        ROD_W::new(self)
    }
    ///Bit 14 - Fast Ethernet speed
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    ///Bit 16 - Carrier sense disable
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<16> {
        CSD_W::new(self)
    }
    ///Bits 17:19 - Interframe gap
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    ///Bit 22 - Jabber disable
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    ///Bit 23 - Watchdog disable
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<23> {
        WD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC configuration register (ETH_MACCR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maccr](index.html) module
pub struct MACCR_SPEC;
impl crate::RegisterSpec for MACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maccr::R](R) reader structure
impl crate::Readable for MACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maccr::W](W) writer structure
impl crate::Writable for MACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACCR to value 0x8000
impl crate::Resettable for MACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
