///Register `PCGCCTL` reader
pub struct R(crate::R<PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCGCCTL` writer
pub struct W(crate::W<PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCGCCTL_SPEC>;
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
impl From<crate::W<PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `STPPCLK` reader - Stop PHY clock
pub type STPPCLK_R = crate::BitReader<bool>;
///Field `STPPCLK` writer - Stop PHY clock
pub type STPPCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
///Field `GATEHCLK` reader - Gate HCLK
pub type GATEHCLK_R = crate::BitReader<bool>;
///Field `GATEHCLK` writer - Gate HCLK
pub type GATEHCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
///Field `PHYSUSP` reader - PHY Suspended
pub type PHYSUSP_R = crate::BitReader<bool>;
///Field `PHYSUSP` writer - PHY Suspended
pub type PHYSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
///Field `ENL1GTG` reader - Enable sleep clock gating
pub type ENL1GTG_R = crate::BitReader<bool>;
///Field `ENL1GTG` writer - Enable sleep clock gating
pub type ENL1GTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
///Field `PHYSLEEP` reader - PHY in Sleep
pub type PHYSLEEP_R = crate::BitReader<bool>;
///Field `PHYSLEEP` writer - PHY in Sleep
pub type PHYSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
///Field `SUSP` reader - Deep Sleep
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - Deep Sleep
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCGCCTL_SPEC, bool, O>;
impl R {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable sleep clock gating
    #[inline(always)]
    pub fn enl1gtg(&self) -> ENL1GTG_R {
        ENL1GTG_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PHY in Sleep
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Deep Sleep
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Stop PHY clock
    #[inline(always)]
    #[must_use]
    pub fn stppclk(&mut self) -> STPPCLK_W<0> {
        STPPCLK_W::new(self)
    }
    ///Bit 1 - Gate HCLK
    #[inline(always)]
    #[must_use]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<1> {
        GATEHCLK_W::new(self)
    }
    ///Bit 4 - PHY Suspended
    #[inline(always)]
    #[must_use]
    pub fn physusp(&mut self) -> PHYSUSP_W<4> {
        PHYSUSP_W::new(self)
    }
    ///Bit 5 - Enable sleep clock gating
    #[inline(always)]
    #[must_use]
    pub fn enl1gtg(&mut self) -> ENL1GTG_W<5> {
        ENL1GTG_W::new(self)
    }
    ///Bit 6 - PHY in Sleep
    #[inline(always)]
    #[must_use]
    pub fn physleep(&mut self) -> PHYSLEEP_W<6> {
        PHYSLEEP_W::new(self)
    }
    ///Bit 7 - Deep Sleep
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcgcctl](index.html) module
pub struct PCGCCTL_SPEC;
impl crate::RegisterSpec for PCGCCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcgcctl::R](R) reader structure
impl crate::Readable for PCGCCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcgcctl::W](W) writer structure
impl crate::Writable for PCGCCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCGCCTL to value 0
impl crate::Resettable for PCGCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
