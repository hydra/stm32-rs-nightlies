///Register `FMC_BCHIER` reader
pub struct R(crate::R<FMC_BCHIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FMC_BCHIER` writer
pub struct W(crate::W<FMC_BCHIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BCHIER_SPEC>;
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
impl From<crate::W<FMC_BCHIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BCHIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DUEIE` reader - DUEIE
pub type DUEIE_R = crate::BitReader<bool>;
///Field `DUEIE` writer - DUEIE
pub type DUEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHIER_SPEC, bool, O>;
///Field `DERIE` reader - DERIE
pub type DERIE_R = crate::BitReader<bool>;
///Field `DERIE` writer - DERIE
pub type DERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHIER_SPEC, bool, O>;
///Field `DEFIE` reader - DEFIE
pub type DEFIE_R = crate::BitReader<bool>;
///Field `DEFIE` writer - DEFIE
pub type DEFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHIER_SPEC, bool, O>;
///Field `DSRIE` reader - DSRIE
pub type DSRIE_R = crate::BitReader<bool>;
///Field `DSRIE` writer - DSRIE
pub type DSRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHIER_SPEC, bool, O>;
///Field `EPBRIE` reader - EPBRIE
pub type EPBRIE_R = crate::BitReader<bool>;
///Field `EPBRIE` writer - EPBRIE
pub type EPBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMC_BCHIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - DUEIE
    #[inline(always)]
    pub fn dueie(&self) -> DUEIE_R {
        DUEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DERIE
    #[inline(always)]
    pub fn derie(&self) -> DERIE_R {
        DERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DEFIE
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DSRIE
    #[inline(always)]
    pub fn dsrie(&self) -> DSRIE_R {
        DSRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - EPBRIE
    #[inline(always)]
    pub fn epbrie(&self) -> EPBRIE_R {
        EPBRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DUEIE
    #[inline(always)]
    #[must_use]
    pub fn dueie(&mut self) -> DUEIE_W<0> {
        DUEIE_W::new(self)
    }
    ///Bit 1 - DERIE
    #[inline(always)]
    #[must_use]
    pub fn derie(&mut self) -> DERIE_W<1> {
        DERIE_W::new(self)
    }
    ///Bit 2 - DEFIE
    #[inline(always)]
    #[must_use]
    pub fn defie(&mut self) -> DEFIE_W<2> {
        DEFIE_W::new(self)
    }
    ///Bit 3 - DSRIE
    #[inline(always)]
    #[must_use]
    pub fn dsrie(&mut self) -> DSRIE_W<3> {
        DSRIE_W::new(self)
    }
    ///Bit 4 - EPBRIE
    #[inline(always)]
    #[must_use]
    pub fn epbrie(&mut self) -> EPBRIE_W<4> {
        EPBRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FMC BCH Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_bchier](index.html) module
pub struct FMC_BCHIER_SPEC;
impl crate::RegisterSpec for FMC_BCHIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_bchier::R](R) reader structure
impl crate::Readable for FMC_BCHIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fmc_bchier::W](W) writer structure
impl crate::Writable for FMC_BCHIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FMC_BCHIER to value 0
impl crate::Resettable for FMC_BCHIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
