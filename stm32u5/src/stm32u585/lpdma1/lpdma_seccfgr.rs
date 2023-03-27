///Register `LPDMA_SECCFGR` reader
pub struct R(crate::R<LPDMA_SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_SECCFGR` writer
pub struct W(crate::W<LPDMA_SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_SECCFGR_SPEC>;
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
impl From<crate::W<LPDMA_SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC0` reader - SEC0
pub type SEC0_R = crate::BitReader<bool>;
///Field `SEC0` writer - SEC0
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC1` reader - SEC1
pub type SEC1_R = crate::BitReader<bool>;
///Field `SEC1` writer - SEC1
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC2` reader - SEC2
pub type SEC2_R = crate::BitReader<bool>;
///Field `SEC2` writer - SEC2
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC3` reader - SEC3
pub type SEC3_R = crate::BitReader<bool>;
///Field `SEC3` writer - SEC3
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_SECCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SEC0
    #[inline(always)]
    pub fn sec0(&self) -> SEC0_R {
        SEC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    pub fn sec2(&self) -> SEC2_R {
        SEC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    pub fn sec3(&self) -> SEC3_R {
        SEC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SEC0
    #[inline(always)]
    #[must_use]
    pub fn sec0(&mut self) -> SEC0_W<0> {
        SEC0_W::new(self)
    }
    ///Bit 1 - SEC1
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<1> {
        SEC1_W::new(self)
    }
    ///Bit 2 - SEC2
    #[inline(always)]
    #[must_use]
    pub fn sec2(&mut self) -> SEC2_W<2> {
        SEC2_W::new(self)
    }
    ///Bit 3 - SEC3
    #[inline(always)]
    #[must_use]
    pub fn sec3(&mut self) -> SEC3_W<3> {
        SEC3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA secure configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_seccfgr](index.html) module
pub struct LPDMA_SECCFGR_SPEC;
impl crate::RegisterSpec for LPDMA_SECCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_seccfgr::R](R) reader structure
impl crate::Readable for LPDMA_SECCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_seccfgr::W](W) writer structure
impl crate::Writable for LPDMA_SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_SECCFGR to value 0
impl crate::Resettable for LPDMA_SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
