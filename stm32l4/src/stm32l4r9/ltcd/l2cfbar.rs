///Register `L2CFBAR` reader
pub struct R(crate::R<L2CFBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CFBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CFBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CFBAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L2CFBAR` writer
pub struct W(crate::W<L2CFBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CFBAR_SPEC>;
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
impl From<crate::W<L2CFBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CFBAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CFBADD` reader - Color Frame Buffer Start Address
pub type CFBADD_R = crate::FieldReader<u32, u32>;
///Field `CFBADD` writer - Color Frame Buffer Start Address
pub type CFBADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2CFBAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Color Frame Buffer Start Address
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<0> {
        CFBADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Color Frame Buffer Address Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l2cfbar](index.html) module
pub struct L2CFBAR_SPEC;
impl crate::RegisterSpec for L2CFBAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l2cfbar::R](R) reader structure
impl crate::Readable for L2CFBAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l2cfbar::W](W) writer structure
impl crate::Writable for L2CFBAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L2CFBAR to value 0
impl crate::Resettable for L2CFBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
