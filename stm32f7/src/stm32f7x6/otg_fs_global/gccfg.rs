///Register `GCCFG` reader
pub struct R(crate::R<GCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GCCFG` writer
pub struct W(crate::W<GCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCFG_SPEC>;
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
impl From<crate::W<GCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DCDET` reader - Data contact detection (DCD) status
pub type DCDET_R = crate::BitReader<bool>;
///Field `DCDET` writer - Data contact detection (DCD) status
pub type DCDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `PDET` reader - Primary detection (PD) status
pub type PDET_R = crate::BitReader<bool>;
///Field `PDET` writer - Primary detection (PD) status
pub type PDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `SDET` reader - Secondary detection (SD) status
pub type SDET_R = crate::BitReader<bool>;
///Field `SDET` writer - Secondary detection (SD) status
pub type SDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `PS2DET` reader - DM pull-up detection status
pub type PS2DET_R = crate::BitReader<bool>;
///Field `PS2DET` writer - DM pull-up detection status
pub type PS2DET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `PWRDWN` reader - Power down
pub type PWRDWN_R = crate::BitReader<bool>;
///Field `PWRDWN` writer - Power down
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `BCDEN` reader - Battery charging detector (BCD) enable
pub type BCDEN_R = crate::BitReader<bool>;
///Field `BCDEN` writer - Battery charging detector (BCD) enable
pub type BCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `DCDEN` reader - Data contact detection (DCD) mode enable
pub type DCDEN_R = crate::BitReader<bool>;
///Field `DCDEN` writer - Data contact detection (DCD) mode enable
pub type DCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `PDEN` reader - Primary detection (PD) mode enable
pub type PDEN_R = crate::BitReader<bool>;
///Field `PDEN` writer - Primary detection (PD) mode enable
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `SDEN` reader - Secondary detection (SD) mode enable
pub type SDEN_R = crate::BitReader<bool>;
///Field `SDEN` writer - Secondary detection (SD) mode enable
pub type SDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `VBDEN` reader - USB VBUS detection enable
pub type VBDEN_R = crate::BitReader<bool>;
///Field `VBDEN` writer - USB VBUS detection enable
pub type VBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
impl R {
    ///Bit 0 - Data contact detection (DCD) status
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Primary detection (PD) status
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Secondary detection (SD) status
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DM pull-up detection status
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Power down
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Battery charging detector (BCD) enable
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Data contact detection (DCD) mode enable
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Primary detection (PD) mode enable
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Secondary detection (SD) mode enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Data contact detection (DCD) status
    #[inline(always)]
    #[must_use]
    pub fn dcdet(&mut self) -> DCDET_W<0> {
        DCDET_W::new(self)
    }
    ///Bit 1 - Primary detection (PD) status
    #[inline(always)]
    #[must_use]
    pub fn pdet(&mut self) -> PDET_W<1> {
        PDET_W::new(self)
    }
    ///Bit 2 - Secondary detection (SD) status
    #[inline(always)]
    #[must_use]
    pub fn sdet(&mut self) -> SDET_W<2> {
        SDET_W::new(self)
    }
    ///Bit 3 - DM pull-up detection status
    #[inline(always)]
    #[must_use]
    pub fn ps2det(&mut self) -> PS2DET_W<3> {
        PS2DET_W::new(self)
    }
    ///Bit 16 - Power down
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<16> {
        PWRDWN_W::new(self)
    }
    ///Bit 17 - Battery charging detector (BCD) enable
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<17> {
        BCDEN_W::new(self)
    }
    ///Bit 18 - Data contact detection (DCD) mode enable
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<18> {
        DCDEN_W::new(self)
    }
    ///Bit 19 - Primary detection (PD) mode enable
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<19> {
        PDEN_W::new(self)
    }
    ///Bit 20 - Secondary detection (SD) mode enable
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<20> {
        SDEN_W::new(self)
    }
    ///Bit 21 - USB VBUS detection enable
    #[inline(always)]
    #[must_use]
    pub fn vbden(&mut self) -> VBDEN_W<21> {
        VBDEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS general core configuration register (OTG_FS_GCCFG)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gccfg](index.html) module
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [gccfg::R](R) reader structure
impl crate::Readable for GCCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gccfg::W](W) writer structure
impl crate::Writable for GCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GCCFG to value 0
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
