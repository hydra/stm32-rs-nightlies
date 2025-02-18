///Register `TIM1_DCR` reader
pub struct R(crate::R<TIM1_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_DCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM1_DCR` writer
pub struct W(crate::W<TIM1_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_DCR_SPEC>;
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
impl From<crate::W<TIM1_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_DCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBA` reader - DBA
pub type DBA_R = crate::FieldReader<u8, u8>;
///Field `DBA` writer - DBA
pub type DBA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM1_DCR_SPEC, u8, u8, 5, O>;
///Field `DBL` reader - DBL
pub type DBL_R = crate::FieldReader<u8, u8>;
///Field `DBL` writer - DBL
pub type DBL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM1_DCR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - DBA
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - DBA
    #[inline(always)]
    #[must_use]
    pub fn dba(&mut self) -> DBA_W<0> {
        DBA_W::new(self)
    }
    ///Bits 8:12 - DBL
    #[inline(always)]
    #[must_use]
    pub fn dbl(&mut self) -> DBL_W<8> {
        DBL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 DMA control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim1_dcr](index.html) module
pub struct TIM1_DCR_SPEC;
impl crate::RegisterSpec for TIM1_DCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim1_dcr::R](R) reader structure
impl crate::Readable for TIM1_DCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim1_dcr::W](W) writer structure
impl crate::Writable for TIM1_DCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM1_DCR to value 0
impl crate::Resettable for TIM1_DCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
