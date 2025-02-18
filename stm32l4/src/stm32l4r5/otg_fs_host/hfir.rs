///Register `HFIR` reader
pub struct R(crate::R<HFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HFIR` writer
pub struct W(crate::W<HFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFIR_SPEC>;
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
impl From<crate::W<HFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FRIVL` reader - Frame interval
pub type FRIVL_R = crate::FieldReader<u16, u16>;
///Field `FRIVL` writer - Frame interval
pub type FRIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFIR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Frame interval
    #[inline(always)]
    #[must_use]
    pub fn frivl(&mut self) -> FRIVL_W<0> {
        FRIVL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS Host frame interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hfir](index.html) module
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hfir::R](R) reader structure
impl crate::Readable for HFIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hfir::W](W) writer structure
impl crate::Writable for HFIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HFIR to value 0xea60
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0xea60;
}
