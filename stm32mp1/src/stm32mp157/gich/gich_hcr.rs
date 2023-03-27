///Register `GICH_HCR` reader
pub struct R(crate::R<GICH_HCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_HCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_HCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_HCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICH_HCR` writer
pub struct W(crate::W<GICH_HCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_HCR_SPEC>;
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
impl From<crate::W<GICH_HCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_HCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - EN
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `UIE` reader - UIE
pub type UIE_R = crate::BitReader<bool>;
///Field `UIE` writer - UIE
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `LRENPIE` reader - LRENPIE
pub type LRENPIE_R = crate::BitReader<bool>;
///Field `LRENPIE` writer - LRENPIE
pub type LRENPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `NPIE` reader - NPIE
pub type NPIE_R = crate::BitReader<bool>;
///Field `NPIE` writer - NPIE
pub type NPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `VGRP0EIE` reader - VGRP0EIE
pub type VGRP0EIE_R = crate::BitReader<bool>;
///Field `VGRP0EIE` writer - VGRP0EIE
pub type VGRP0EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `VGRP0DIE` reader - VGRP0DIE
pub type VGRP0DIE_R = crate::BitReader<bool>;
///Field `VGRP0DIE` writer - VGRP0DIE
pub type VGRP0DIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `VGRP1EIE` reader - VGRP1EIE
pub type VGRP1EIE_R = crate::BitReader<bool>;
///Field `VGRP1EIE` writer - VGRP1EIE
pub type VGRP1EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `VGRP1DIE` reader - VGRP1DIE
pub type VGRP1DIE_R = crate::BitReader<bool>;
///Field `VGRP1DIE` writer - VGRP1DIE
pub type VGRP1DIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICH_HCR_SPEC, bool, O>;
///Field `EOICOUNT` reader - EOICOUNT
pub type EOICOUNT_R = crate::FieldReader<u8, u8>;
///Field `EOICOUNT` writer - EOICOUNT
pub type EOICOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_HCR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UIE
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LRENPIE
    #[inline(always)]
    pub fn lrenpie(&self) -> LRENPIE_R {
        LRENPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NPIE
    #[inline(always)]
    pub fn npie(&self) -> NPIE_R {
        NPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VGRP0EIE
    #[inline(always)]
    pub fn vgrp0eie(&self) -> VGRP0EIE_R {
        VGRP0EIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - VGRP0DIE
    #[inline(always)]
    pub fn vgrp0die(&self) -> VGRP0DIE_R {
        VGRP0DIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - VGRP1EIE
    #[inline(always)]
    pub fn vgrp1eie(&self) -> VGRP1EIE_R {
        VGRP1EIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - VGRP1DIE
    #[inline(always)]
    pub fn vgrp1die(&self) -> VGRP1DIE_R {
        VGRP1DIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 27:31 - EOICOUNT
    #[inline(always)]
    pub fn eoicount(&self) -> EOICOUNT_R {
        EOICOUNT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - UIE
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<1> {
        UIE_W::new(self)
    }
    ///Bit 2 - LRENPIE
    #[inline(always)]
    #[must_use]
    pub fn lrenpie(&mut self) -> LRENPIE_W<2> {
        LRENPIE_W::new(self)
    }
    ///Bit 3 - NPIE
    #[inline(always)]
    #[must_use]
    pub fn npie(&mut self) -> NPIE_W<3> {
        NPIE_W::new(self)
    }
    ///Bit 4 - VGRP0EIE
    #[inline(always)]
    #[must_use]
    pub fn vgrp0eie(&mut self) -> VGRP0EIE_W<4> {
        VGRP0EIE_W::new(self)
    }
    ///Bit 5 - VGRP0DIE
    #[inline(always)]
    #[must_use]
    pub fn vgrp0die(&mut self) -> VGRP0DIE_W<5> {
        VGRP0DIE_W::new(self)
    }
    ///Bit 6 - VGRP1EIE
    #[inline(always)]
    #[must_use]
    pub fn vgrp1eie(&mut self) -> VGRP1EIE_W<6> {
        VGRP1EIE_W::new(self)
    }
    ///Bit 7 - VGRP1DIE
    #[inline(always)]
    #[must_use]
    pub fn vgrp1die(&mut self) -> VGRP1DIE_W<7> {
        VGRP1DIE_W::new(self)
    }
    ///Bits 27:31 - EOICOUNT
    #[inline(always)]
    #[must_use]
    pub fn eoicount(&mut self) -> EOICOUNT_W<27> {
        EOICOUNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICH hypervisor control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_hcr](index.html) module
pub struct GICH_HCR_SPEC;
impl crate::RegisterSpec for GICH_HCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_hcr::R](R) reader structure
impl crate::Readable for GICH_HCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gich_hcr::W](W) writer structure
impl crate::Writable for GICH_HCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICH_HCR to value 0
impl crate::Resettable for GICH_HCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
