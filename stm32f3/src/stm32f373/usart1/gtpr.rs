///Register `GTPR` reader
pub struct R(crate::R<GTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GTPR` writer
pub struct W(crate::W<GTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPR_SPEC>;
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
impl From<crate::W<GTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PSC` reader - Prescaler value
pub type PSC_R = crate::FieldReader<u8, u8>;
///Field `PSC` writer - Prescaler value
pub type PSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTPR_SPEC, u8, u8, 8, O>;
///Field `GT` reader - Guard time value
pub type GT_R = crate::FieldReader<u8, u8>;
///Field `GT` writer - Guard time value
pub type GT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GTPR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    ///Bits 8:15 - Guard time value
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<8> {
        GT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Guard time and prescaler register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtpr](index.html) module
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtpr::R](R) reader structure
impl crate::Readable for GTPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gtpr::W](W) writer structure
impl crate::Writable for GTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GTPR to value 0
impl crate::Resettable for GTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
