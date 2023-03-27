///Register `DPACFGR` reader
pub struct R(crate::R<DPACFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPACFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPACFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPACFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DPACFGR` writer
pub struct W(crate::W<DPACFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPACFGR_SPEC>;
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
impl From<crate::W<DPACFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPACFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REDCFG` reader - REDCFG
pub type REDCFG_R = crate::BitReader<bool>;
///Field `REDCFG` writer - REDCFG
pub type REDCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPACFGR_SPEC, bool, O>;
///Field `RESEED` reader - RESEED
pub type RESEED_R = crate::BitReader<bool>;
///Field `RESEED` writer - RESEED
pub type RESEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DPACFGR_SPEC, bool, O>;
///Field `TRIMCFG` reader - TRIMCFG
pub type TRIMCFG_R = crate::FieldReader<u8, u8>;
///Field `TRIMCFG` writer - TRIMCFG
pub type TRIMCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPACFGR_SPEC, u8, u8, 2, O>;
///Field `CONFIGLOCK` reader - CONFIGLOCK
pub type CONFIGLOCK_R = crate::BitReader<bool>;
impl R {
    ///Bit 1 - REDCFG
    #[inline(always)]
    pub fn redcfg(&self) -> REDCFG_R {
        REDCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RESEED
    #[inline(always)]
    pub fn reseed(&self) -> RESEED_R {
        RESEED_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - TRIMCFG
    #[inline(always)]
    pub fn trimcfg(&self) -> TRIMCFG_R {
        TRIMCFG_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 31 - CONFIGLOCK
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - REDCFG
    #[inline(always)]
    #[must_use]
    pub fn redcfg(&mut self) -> REDCFG_W<1> {
        REDCFG_W::new(self)
    }
    ///Bit 2 - RESEED
    #[inline(always)]
    #[must_use]
    pub fn reseed(&mut self) -> RESEED_W<2> {
        RESEED_W::new(self)
    }
    ///Bits 3:4 - TRIMCFG
    #[inline(always)]
    #[must_use]
    pub fn trimcfg(&mut self) -> TRIMCFG_W<3> {
        TRIMCFG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dpacfgr](index.html) module
pub struct DPACFGR_SPEC;
impl crate::RegisterSpec for DPACFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dpacfgr::R](R) reader structure
impl crate::Readable for DPACFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dpacfgr::W](W) writer structure
impl crate::Writable for DPACFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DPACFGR to value 0x08
impl crate::Resettable for DPACFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
