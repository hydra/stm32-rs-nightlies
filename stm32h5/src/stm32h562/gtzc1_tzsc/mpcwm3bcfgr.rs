///Register `MPCWM3BCFGR` reader
pub struct R(crate::R<MPCWM3BCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM3BCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM3BCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM3BCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCWM3BCFGR` writer
pub struct W(crate::W<MPCWM3BCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM3BCFGR_SPEC>;
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
impl From<crate::W<MPCWM3BCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM3BCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SREN` reader - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value).
pub type SREN_R = crate::BitReader<bool>;
///Field `SREN` writer - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value).
pub type SREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCWM3BCFGR_SPEC, bool, O>;
///Field `SRLOCK` reader - Sub-region B lock This bit, once set, can be cleared only by a system reset.
pub type SRLOCK_R = crate::BitReader<bool>;
///Field `SRLOCK` writer - Sub-region B lock This bit, once set, can be cleared only by a system reset.
pub type SRLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCWM3BCFGR_SPEC, bool, O>;
///Field `SEC` reader - Secure sub-region B of base region x This bit is taken into account only if SREN is set.
pub type SEC_R = crate::BitReader<bool>;
///Field `SEC` writer - Secure sub-region B of base region x This bit is taken into account only if SREN is set.
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCWM3BCFGR_SPEC, bool, O>;
///Field `PRIV` reader - Privileged sub-region B of base region x This bit is taken into account only if SREN is set.
pub type PRIV_R = crate::BitReader<bool>;
///Field `PRIV` writer - Privileged sub-region B of base region x This bit is taken into account only if SREN is set.
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCWM3BCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value).
    #[inline(always)]
    pub fn sren(&self) -> SREN_R {
        SREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Sub-region B lock This bit, once set, can be cleared only by a system reset.
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Secure sub-region B of base region x This bit is taken into account only if SREN is set.
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privileged sub-region B of base region x This bit is taken into account only if SREN is set.
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Sub-region B enable Note: External memories that are watermark controlled start fully non-secure/unprivileged at reset when TZEN = 0xC3. When TZEN = 0xB4, external memories start fully secure/fully privileged (inverted reset-value).
    #[inline(always)]
    #[must_use]
    pub fn sren(&mut self) -> SREN_W<0> {
        SREN_W::new(self)
    }
    ///Bit 1 - Sub-region B lock This bit, once set, can be cleared only by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn srlock(&mut self) -> SRLOCK_W<1> {
        SRLOCK_W::new(self)
    }
    ///Bit 8 - Secure sub-region B of base region x This bit is taken into account only if SREN is set.
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<8> {
        SEC_W::new(self)
    }
    ///Bit 9 - Privileged sub-region B of base region x This bit is taken into account only if SREN is set.
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<9> {
        PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GTZC1 TZSC memory 3 sub-region B watermark configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcwm3bcfgr](index.html) module
pub struct MPCWM3BCFGR_SPEC;
impl crate::RegisterSpec for MPCWM3BCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcwm3bcfgr::R](R) reader structure
impl crate::Readable for MPCWM3BCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcwm3bcfgr::W](W) writer structure
impl crate::Writable for MPCWM3BCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCWM3BCFGR to value 0
impl crate::Resettable for MPCWM3BCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
