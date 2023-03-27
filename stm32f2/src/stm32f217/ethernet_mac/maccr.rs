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
///Field `RE` reader - RE
pub type RE_R = crate::BitReader<bool>;
///Field `RE` writer - RE
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `TE` reader - TE
pub type TE_R = crate::BitReader<bool>;
///Field `TE` writer - TE
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `DC` reader - DC
pub type DC_R = crate::BitReader<bool>;
///Field `DC` writer - DC
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `BL` reader - BL
pub type BL_R = crate::FieldReader<u8, u8>;
///Field `BL` writer - BL
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 2, O>;
///Field `APCS` reader - APCS
pub type APCS_R = crate::BitReader<bool>;
///Field `APCS` writer - APCS
pub type APCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `RD` reader - RD
pub type RD_R = crate::BitReader<bool>;
///Field `RD` writer - RD
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `IPCO` reader - IPCO
pub type IPCO_R = crate::BitReader<bool>;
///Field `IPCO` writer - IPCO
pub type IPCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `DM` reader - DM
pub type DM_R = crate::BitReader<bool>;
///Field `DM` writer - DM
pub type DM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `LM` reader - LM
pub type LM_R = crate::BitReader<bool>;
///Field `LM` writer - LM
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `ROD` reader - ROD
pub type ROD_R = crate::BitReader<bool>;
///Field `ROD` writer - ROD
pub type ROD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `FES` reader - FES
pub type FES_R = crate::BitReader<bool>;
///Field `FES` writer - FES
pub type FES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `CSD` reader - CSD
pub type CSD_R = crate::BitReader<bool>;
///Field `CSD` writer - CSD
pub type CSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `IFG` reader - IFG
pub type IFG_R = crate::FieldReader<u8, u8>;
///Field `IFG` writer - IFG
pub type IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACCR_SPEC, u8, u8, 3, O>;
///Field `JD` reader - JD
pub type JD_R = crate::BitReader<bool>;
///Field `JD` writer - JD
pub type JD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `WD` reader - WD
pub type WD_R = crate::BitReader<bool>;
///Field `WD` writer - WD
pub type WD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
///Field `CSTF` reader - CSTF
pub type CSTF_R = crate::BitReader<bool>;
///Field `CSTF` writer - CSTF
pub type CSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACCR_SPEC, bool, O>;
impl R {
    ///Bit 2 - RE
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TE
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DC
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - BL
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - APCS
    #[inline(always)]
    pub fn apcs(&self) -> APCS_R {
        APCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - RD
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - IPCO
    #[inline(always)]
    pub fn ipco(&self) -> IPCO_R {
        IPCO_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DM
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LM
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ROD
    #[inline(always)]
    pub fn rod(&self) -> ROD_R {
        ROD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - FES
    #[inline(always)]
    pub fn fes(&self) -> FES_R {
        FES_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CSD
    #[inline(always)]
    pub fn csd(&self) -> CSD_R {
        CSD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - IFG
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 22 - JD
    #[inline(always)]
    pub fn jd(&self) -> JD_R {
        JD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - WD
    #[inline(always)]
    pub fn wd(&self) -> WD_R {
        WD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CSTF
    #[inline(always)]
    pub fn cstf(&self) -> CSTF_R {
        CSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - RE
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - TE
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - DC
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<4> {
        DC_W::new(self)
    }
    ///Bits 5:6 - BL
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<5> {
        BL_W::new(self)
    }
    ///Bit 7 - APCS
    #[inline(always)]
    #[must_use]
    pub fn apcs(&mut self) -> APCS_W<7> {
        APCS_W::new(self)
    }
    ///Bit 9 - RD
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<9> {
        RD_W::new(self)
    }
    ///Bit 10 - IPCO
    #[inline(always)]
    #[must_use]
    pub fn ipco(&mut self) -> IPCO_W<10> {
        IPCO_W::new(self)
    }
    ///Bit 11 - DM
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<11> {
        DM_W::new(self)
    }
    ///Bit 12 - LM
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<12> {
        LM_W::new(self)
    }
    ///Bit 13 - ROD
    #[inline(always)]
    #[must_use]
    pub fn rod(&mut self) -> ROD_W<13> {
        ROD_W::new(self)
    }
    ///Bit 14 - FES
    #[inline(always)]
    #[must_use]
    pub fn fes(&mut self) -> FES_W<14> {
        FES_W::new(self)
    }
    ///Bit 16 - CSD
    #[inline(always)]
    #[must_use]
    pub fn csd(&mut self) -> CSD_W<16> {
        CSD_W::new(self)
    }
    ///Bits 17:19 - IFG
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<17> {
        IFG_W::new(self)
    }
    ///Bit 22 - JD
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JD_W<22> {
        JD_W::new(self)
    }
    ///Bit 23 - WD
    #[inline(always)]
    #[must_use]
    pub fn wd(&mut self) -> WD_W<23> {
        WD_W::new(self)
    }
    ///Bit 25 - CSTF
    #[inline(always)]
    #[must_use]
    pub fn cstf(&mut self) -> CSTF_W<25> {
        CSTF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC configuration register
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
