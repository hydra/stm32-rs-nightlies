///Register `MMC_CONTROL` reader
pub struct R(crate::R<MMC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMC_CONTROL` writer
pub struct W(crate::W<MMC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_CONTROL_SPEC>;
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
impl From<crate::W<MMC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNTRST` reader - Counters Reset
pub type CNTRST_R = crate::BitReader<bool>;
///Field `CNTRST` writer - Counters Reset
pub type CNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `CNTSTOPRO` reader - Counter Stop Rollover
pub type CNTSTOPRO_R = crate::BitReader<bool>;
///Field `CNTSTOPRO` writer - Counter Stop Rollover
pub type CNTSTOPRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `RSTONRD` reader - Reset on Read
pub type RSTONRD_R = crate::BitReader<bool>;
///Field `RSTONRD` writer - Reset on Read
pub type RSTONRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `CNTFREEZ` reader - MMC Counter Freeze
pub type CNTFREEZ_R = crate::BitReader<bool>;
///Field `CNTFREEZ` writer - MMC Counter Freeze
pub type CNTFREEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `CNTPRST` reader - Counters Preset
pub type CNTPRST_R = crate::BitReader<bool>;
///Field `CNTPRST` writer - Counters Preset
pub type CNTPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `CNTPRSTLVL` reader - Full-Half Preset
pub type CNTPRSTLVL_R = crate::BitReader<bool>;
///Field `CNTPRSTLVL` writer - Full-Half Preset
pub type CNTPRSTLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
///Field `UCDBC` reader - Update MMC Counters for Dropped Broadcast Packets
pub type UCDBC_R = crate::BitReader<bool>;
///Field `UCDBC` writer - Update MMC Counters for Dropped Broadcast Packets
pub type UCDBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_CONTROL_SPEC, bool, O>;
impl R {
    ///Bit 0 - Counters Reset
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Counter Stop Rollover
    #[inline(always)]
    pub fn cntstopro(&self) -> CNTSTOPRO_R {
        CNTSTOPRO_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reset on Read
    #[inline(always)]
    pub fn rstonrd(&self) -> RSTONRD_R {
        RSTONRD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MMC Counter Freeze
    #[inline(always)]
    pub fn cntfreez(&self) -> CNTFREEZ_R {
        CNTFREEZ_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Counters Preset
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Full-Half Preset
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CNTPRSTLVL_R {
        CNTPRSTLVL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Update MMC Counters for Dropped Broadcast Packets
    #[inline(always)]
    pub fn ucdbc(&self) -> UCDBC_R {
        UCDBC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Counters Reset
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<0> {
        CNTRST_W::new(self)
    }
    ///Bit 1 - Counter Stop Rollover
    #[inline(always)]
    #[must_use]
    pub fn cntstopro(&mut self) -> CNTSTOPRO_W<1> {
        CNTSTOPRO_W::new(self)
    }
    ///Bit 2 - Reset on Read
    #[inline(always)]
    #[must_use]
    pub fn rstonrd(&mut self) -> RSTONRD_W<2> {
        RSTONRD_W::new(self)
    }
    ///Bit 3 - MMC Counter Freeze
    #[inline(always)]
    #[must_use]
    pub fn cntfreez(&mut self) -> CNTFREEZ_W<3> {
        CNTFREEZ_W::new(self)
    }
    ///Bit 4 - Counters Preset
    #[inline(always)]
    #[must_use]
    pub fn cntprst(&mut self) -> CNTPRST_W<4> {
        CNTPRST_W::new(self)
    }
    ///Bit 5 - Full-Half Preset
    #[inline(always)]
    #[must_use]
    pub fn cntprstlvl(&mut self) -> CNTPRSTLVL_W<5> {
        CNTPRSTLVL_W::new(self)
    }
    ///Bit 8 - Update MMC Counters for Dropped Broadcast Packets
    #[inline(always)]
    #[must_use]
    pub fn ucdbc(&mut self) -> UCDBC_W<8> {
        UCDBC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MMC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmc_control](index.html) module
pub struct MMC_CONTROL_SPEC;
impl crate::RegisterSpec for MMC_CONTROL_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmc_control::R](R) reader structure
impl crate::Readable for MMC_CONTROL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmc_control::W](W) writer structure
impl crate::Writable for MMC_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMC_CONTROL to value 0
impl crate::Resettable for MMC_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
