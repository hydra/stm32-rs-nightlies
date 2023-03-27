///Register `GPDMA_SECCFGR` reader
pub struct R(crate::R<GPDMA_SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDMA_SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDMA_SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDMA_SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPDMA_SECCFGR` writer
pub struct W(crate::W<GPDMA_SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDMA_SECCFGR_SPEC>;
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
impl From<crate::W<GPDMA_SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDMA_SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC0` reader - SEC0
pub type SEC0_R = crate::BitReader<bool>;
///Field `SEC0` writer - SEC0
pub type SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC1` reader - SEC1
pub type SEC1_R = crate::BitReader<bool>;
///Field `SEC1` writer - SEC1
pub type SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC2` reader - SEC2
pub type SEC2_R = crate::BitReader<bool>;
///Field `SEC2` writer - SEC2
pub type SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC3` reader - SEC3
pub type SEC3_R = crate::BitReader<bool>;
///Field `SEC3` writer - SEC3
pub type SEC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC4` reader - SEC4
pub type SEC4_R = crate::BitReader<bool>;
///Field `SEC4` writer - SEC4
pub type SEC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC5` reader - SEC5
pub type SEC5_R = crate::BitReader<bool>;
///Field `SEC5` writer - SEC5
pub type SEC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC6` reader - SEC6
pub type SEC6_R = crate::BitReader<bool>;
///Field `SEC6` writer - SEC6
pub type SEC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC7` reader - SEC7
pub type SEC7_R = crate::BitReader<bool>;
///Field `SEC7` writer - SEC7
pub type SEC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC8` reader - SEC8
pub type SEC8_R = crate::BitReader<bool>;
///Field `SEC8` writer - SEC8
pub type SEC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC9` reader - SEC9
pub type SEC9_R = crate::BitReader<bool>;
///Field `SEC9` writer - SEC9
pub type SEC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC10` reader - SEC10
pub type SEC10_R = crate::BitReader<bool>;
///Field `SEC10` writer - SEC10
pub type SEC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC11` reader - SEC11
pub type SEC11_R = crate::BitReader<bool>;
///Field `SEC11` writer - SEC11
pub type SEC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC12` reader - SEC12
pub type SEC12_R = crate::BitReader<bool>;
///Field `SEC12` writer - SEC12
pub type SEC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC13` reader - SEC13
pub type SEC13_R = crate::BitReader<bool>;
///Field `SEC13` writer - SEC13
pub type SEC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC14` reader - SEC14
pub type SEC14_R = crate::BitReader<bool>;
///Field `SEC14` writer - SEC14
pub type SEC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
///Field `SEC15` reader - SEC15
pub type SEC15_R = crate::BitReader<bool>;
///Field `SEC15` writer - SEC15
pub type SEC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPDMA_SECCFGR_SPEC, bool, O>;
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
    ///Bit 4 - SEC4
    #[inline(always)]
    pub fn sec4(&self) -> SEC4_R {
        SEC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    pub fn sec5(&self) -> SEC5_R {
        SEC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    pub fn sec6(&self) -> SEC6_R {
        SEC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    pub fn sec7(&self) -> SEC7_R {
        SEC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    pub fn sec8(&self) -> SEC8_R {
        SEC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    pub fn sec9(&self) -> SEC9_R {
        SEC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    pub fn sec11(&self) -> SEC11_R {
        SEC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    pub fn sec12(&self) -> SEC12_R {
        SEC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    pub fn sec13(&self) -> SEC13_R {
        SEC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    pub fn sec14(&self) -> SEC14_R {
        SEC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    pub fn sec15(&self) -> SEC15_R {
        SEC15_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Bit 4 - SEC4
    #[inline(always)]
    #[must_use]
    pub fn sec4(&mut self) -> SEC4_W<4> {
        SEC4_W::new(self)
    }
    ///Bit 5 - SEC5
    #[inline(always)]
    #[must_use]
    pub fn sec5(&mut self) -> SEC5_W<5> {
        SEC5_W::new(self)
    }
    ///Bit 6 - SEC6
    #[inline(always)]
    #[must_use]
    pub fn sec6(&mut self) -> SEC6_W<6> {
        SEC6_W::new(self)
    }
    ///Bit 7 - SEC7
    #[inline(always)]
    #[must_use]
    pub fn sec7(&mut self) -> SEC7_W<7> {
        SEC7_W::new(self)
    }
    ///Bit 8 - SEC8
    #[inline(always)]
    #[must_use]
    pub fn sec8(&mut self) -> SEC8_W<8> {
        SEC8_W::new(self)
    }
    ///Bit 9 - SEC9
    #[inline(always)]
    #[must_use]
    pub fn sec9(&mut self) -> SEC9_W<9> {
        SEC9_W::new(self)
    }
    ///Bit 10 - SEC10
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<10> {
        SEC10_W::new(self)
    }
    ///Bit 11 - SEC11
    #[inline(always)]
    #[must_use]
    pub fn sec11(&mut self) -> SEC11_W<11> {
        SEC11_W::new(self)
    }
    ///Bit 12 - SEC12
    #[inline(always)]
    #[must_use]
    pub fn sec12(&mut self) -> SEC12_W<12> {
        SEC12_W::new(self)
    }
    ///Bit 13 - SEC13
    #[inline(always)]
    #[must_use]
    pub fn sec13(&mut self) -> SEC13_W<13> {
        SEC13_W::new(self)
    }
    ///Bit 14 - SEC14
    #[inline(always)]
    #[must_use]
    pub fn sec14(&mut self) -> SEC14_W<14> {
        SEC14_W::new(self)
    }
    ///Bit 15 - SEC15
    #[inline(always)]
    #[must_use]
    pub fn sec15(&mut self) -> SEC15_W<15> {
        SEC15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPDMA secure configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpdma_seccfgr](index.html) module
pub struct GPDMA_SECCFGR_SPEC;
impl crate::RegisterSpec for GPDMA_SECCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpdma_seccfgr::R](R) reader structure
impl crate::Readable for GPDMA_SECCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpdma_seccfgr::W](W) writer structure
impl crate::Writable for GPDMA_SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPDMA_SECCFGR to value 0
impl crate::Resettable for GPDMA_SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
