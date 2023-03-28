///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PVDL` reader - PVD lock enable bit.
pub type PVDL_R = crate::BitReader<bool>;
///Field `PVDL` writer - PVD lock enable bit.
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
///Field `FLASHL` reader - Flash double ECC error lock bit
pub type FLASHL_R = crate::BitReader<bool>;
///Field `FLASHL` writer - Flash double ECC error lock bit
pub type FLASHL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
///Field `CM7L` reader - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit
pub type CM7L_R = crate::BitReader<bool>;
///Field `CM7L` writer - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit
pub type CM7L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
///Field `DTCML` reader - D1TCM or D0TCM double ECC error signal lock
pub type DTCML_R = crate::BitReader<bool>;
///Field `DTCML` writer - D1TCM or D0TCM double ECC error signal lock
pub type DTCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
///Field `ITCML` reader - ITCM double ECC error signal lock
pub type ITCML_R = crate::BitReader<bool>;
///Field `ITCML` writer - ITCM double ECC error signal lock
pub type ITCML_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    ///Bit 2 - PVD lock enable bit.
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash double ECC error lock bit
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - D1TCM or D0TCM double ECC error signal lock
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ITCM double ECC error signal lock
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - PVD lock enable bit.
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    ///Bit 3 - Flash double ECC error lock bit
    #[inline(always)]
    #[must_use]
    pub fn flashl(&mut self) -> FLASHL_W<3> {
        FLASHL_W::new(self)
    }
    ///Bit 6 - CortexÃ‚Â®-M7 LOCKUP (HardFault) output enable bit
    #[inline(always)]
    #[must_use]
    pub fn cm7l(&mut self) -> CM7L_W<6> {
        CM7L_W::new(self)
    }
    ///Bit 13 - D1TCM or D0TCM double ECC error signal lock
    #[inline(always)]
    #[must_use]
    pub fn dtcml(&mut self) -> DTCML_W<13> {
        DTCML_W::new(self)
    }
    ///Bit 14 - ITCM double ECC error signal lock
    #[inline(always)]
    #[must_use]
    pub fn itcml(&mut self) -> ITCML_W<14> {
        ITCML_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SYSCFG timer break lockup register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
