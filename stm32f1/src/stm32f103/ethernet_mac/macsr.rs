///Register `MACSR` reader
pub struct R(crate::R<MACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACSR` writer
pub struct W(crate::W<MACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSR_SPEC>;
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
impl From<crate::W<MACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PMTS` reader - PMT status
pub type PMTS_R = crate::BitReader<bool>;
///Field `PMTS` writer - PMT status
pub type PMTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
///Field `MMCS` reader - MMC status
pub type MMCS_R = crate::BitReader<bool>;
///Field `MMCS` writer - MMC status
pub type MMCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
///Field `MMCRS` reader - MMC receive status
pub type MMCRS_R = crate::BitReader<bool>;
///Field `MMCRS` writer - MMC receive status
pub type MMCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
///Field `MMCTS` reader - MMC transmit status
pub type MMCTS_R = crate::BitReader<bool>;
///Field `MMCTS` writer - MMC transmit status
pub type MMCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
///Field `TSTS` reader - Time stamp trigger status
pub type TSTS_R = crate::BitReader<bool>;
///Field `TSTS` writer - Time stamp trigger status
pub type TSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACSR_SPEC, bool, O>;
impl R {
    ///Bit 3 - PMT status
    #[inline(always)]
    pub fn pmts(&self) -> PMTS_R {
        PMTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MMC status
    #[inline(always)]
    pub fn mmcs(&self) -> MMCS_R {
        MMCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MMC receive status
    #[inline(always)]
    pub fn mmcrs(&self) -> MMCRS_R {
        MMCRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MMC transmit status
    #[inline(always)]
    pub fn mmcts(&self) -> MMCTS_R {
        MMCTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    pub fn tsts(&self) -> TSTS_R {
        TSTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - PMT status
    #[inline(always)]
    #[must_use]
    pub fn pmts(&mut self) -> PMTS_W<3> {
        PMTS_W::new(self)
    }
    ///Bit 4 - MMC status
    #[inline(always)]
    #[must_use]
    pub fn mmcs(&mut self) -> MMCS_W<4> {
        MMCS_W::new(self)
    }
    ///Bit 5 - MMC receive status
    #[inline(always)]
    #[must_use]
    pub fn mmcrs(&mut self) -> MMCRS_W<5> {
        MMCRS_W::new(self)
    }
    ///Bit 6 - MMC transmit status
    #[inline(always)]
    #[must_use]
    pub fn mmcts(&mut self) -> MMCTS_W<6> {
        MMCTS_W::new(self)
    }
    ///Bit 9 - Time stamp trigger status
    #[inline(always)]
    #[must_use]
    pub fn tsts(&mut self) -> TSTS_W<9> {
        TSTS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC interrupt status register (ETH_MACSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macsr](index.html) module
pub struct MACSR_SPEC;
impl crate::RegisterSpec for MACSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macsr::R](R) reader structure
impl crate::Readable for MACSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macsr::W](W) writer structure
impl crate::Writable for MACSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACSR to value 0
impl crate::Resettable for MACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
