///Register `I2SCFGR` reader
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I2SCFGR` writer
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CHLEN` reader - CHLEN
pub type CHLEN_R = crate::BitReader<bool>;
///Field `CHLEN` writer - CHLEN
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
///Field `DATLEN` reader - DATLEN
pub type DATLEN_R = crate::FieldReader<u8, u8>;
///Field `DATLEN` writer - DATLEN
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 2, O>;
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::BitReader<bool>;
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
///Field `I2SSTD` reader - I2SSTD
pub type I2SSTD_R = crate::FieldReader<u8, u8>;
///Field `I2SSTD` writer - I2SSTD
pub type I2SSTD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 2, O>;
///Field `PCMSYNC` reader - PCMSYNC
pub type PCMSYNC_R = crate::BitReader<bool>;
///Field `PCMSYNC` writer - PCMSYNC
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
///Field `I2SCFG` reader - I2SCFG
pub type I2SCFG_R = crate::FieldReader<u8, u8>;
///Field `I2SCFG` writer - I2SCFG
pub type I2SCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SCFGR_SPEC, u8, u8, 2, O>;
///Field `I2SE` reader - I2SE
pub type I2SE_R = crate::BitReader<bool>;
///Field `I2SE` writer - I2SE
pub type I2SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
///Field `I2SMOD` reader - I2SMOD
pub type I2SMOD_R = crate::BitReader<bool>;
///Field `I2SMOD` writer - I2SMOD
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CHLEN
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CHLEN
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<0> {
        CHLEN_W::new(self)
    }
    ///Bits 1:2 - DATLEN
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    ///Bit 3 - CKPOL
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<3> {
        CKPOL_W::new(self)
    }
    ///Bits 4:5 - I2SSTD
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    ///Bit 7 - PCMSYNC
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    ///Bits 8:9 - I2SCFG
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<8> {
        I2SCFG_W::new(self)
    }
    ///Bit 10 - I2SE
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<10> {
        I2SE_W::new(self)
    }
    ///Bit 11 - I2SMOD
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<11> {
        I2SMOD_W::new(self)
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
///For information about available fields see [i2scfgr](index.html) module
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [i2scfgr::R](R) reader structure
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i2scfgr::W](W) writer structure
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I2SCFGR to value 0
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
