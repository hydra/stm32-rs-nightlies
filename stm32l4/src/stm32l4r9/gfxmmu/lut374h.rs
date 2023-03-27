///Register `LUT374H` reader
pub struct R(crate::R<LUT374H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT374H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT374H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT374H_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LUT374H` writer
pub struct W(crate::W<LUT374H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT374H_SPEC>;
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
impl From<crate::W<LUT374H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT374H_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LO` reader - Line offset
pub type LO_R = crate::FieldReader<u32, u32>;
///Field `LO` writer - Line offset
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT374H_SPEC, u32, u32, 18, O>;
impl R {
    ///Bits 4:21 - Line offset
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits >> 4) & 0x0003_ffff)
    }
}
impl W {
    ///Bits 4:21 - Line offset
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<4> {
        LO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Graphic MMU LUT entry 374 high
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lut374h](index.html) module
pub struct LUT374H_SPEC;
impl crate::RegisterSpec for LUT374H_SPEC {
    type Ux = u32;
}
///`read()` method returns [lut374h::R](R) reader structure
impl crate::Readable for LUT374H_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lut374h::W](W) writer structure
impl crate::Writable for LUT374H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LUT374H to value 0
impl crate::Resettable for LUT374H_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
