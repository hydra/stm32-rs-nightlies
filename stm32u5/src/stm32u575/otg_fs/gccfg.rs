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
///Field `DCDET` reader - DCDET
pub type DCDET_R = crate::BitReader<bool>;
///Field `PDET` reader - PDET
pub type PDET_R = crate::BitReader<bool>;
///Field `SDET` reader - SDET
pub type SDET_R = crate::BitReader<bool>;
///Field `PS2DET` reader - PS2DET
pub type PS2DET_R = crate::BitReader<bool>;
///Field `PWRDWN` reader - PWRDWN
pub type PWRDWN_R = crate::BitReader<bool>;
///Field `PWRDWN` writer - PWRDWN
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `BCDEN` reader - BCDEN
pub type BCDEN_R = crate::BitReader<bool>;
///Field `BCDEN` writer - BCDEN
pub type BCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `DCDEN` reader - DCDEN
pub type DCDEN_R = crate::BitReader<bool>;
///Field `DCDEN` writer - DCDEN
pub type DCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `PDEN` reader - PDEN
pub type PDEN_R = crate::BitReader<bool>;
///Field `PDEN` writer - PDEN
pub type PDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `SDEN` reader - SDEN
pub type SDEN_R = crate::BitReader<bool>;
///Field `SDEN` writer - SDEN
pub type SDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
///Field `VBDEN` reader - VBDEN
pub type VBDEN_R = crate::BitReader<bool>;
///Field `VBDEN` writer - VBDEN
pub type VBDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCFG_SPEC, bool, O>;
impl R {
    ///Bit 0 - DCDET
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PDET
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SDET
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PS2DET
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - PWRDWN
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - BCDEN
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DCDEN
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PDEN
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SDEN
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - VBDEN
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 16 - PWRDWN
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<16> {
        PWRDWN_W::new(self)
    }
    ///Bit 17 - BCDEN
    #[inline(always)]
    #[must_use]
    pub fn bcden(&mut self) -> BCDEN_W<17> {
        BCDEN_W::new(self)
    }
    ///Bit 18 - DCDEN
    #[inline(always)]
    #[must_use]
    pub fn dcden(&mut self) -> DCDEN_W<18> {
        DCDEN_W::new(self)
    }
    ///Bit 19 - PDEN
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<19> {
        PDEN_W::new(self)
    }
    ///Bit 20 - SDEN
    #[inline(always)]
    #[must_use]
    pub fn sden(&mut self) -> SDEN_W<20> {
        SDEN_W::new(self)
    }
    ///Bit 21 - VBDEN
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
///OTG general core configuration register
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
