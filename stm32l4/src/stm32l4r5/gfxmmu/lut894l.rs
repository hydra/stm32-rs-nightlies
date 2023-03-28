///Register `LUT894L` reader
pub struct R(crate::R<LUT894L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT894L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT894L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT894L_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LUT894L` writer
pub struct W(crate::W<LUT894L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT894L_SPEC>;
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
impl From<crate::W<LUT894L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT894L_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - Enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUT894L_SPEC, bool, O>;
///Field `FVB` reader - First Valid Block
pub type FVB_R = crate::FieldReader<u8, u8>;
///Field `FVB` writer - First Valid Block
pub type FVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT894L_SPEC, u8, u8, 8, O>;
///Field `LVB` reader - Last Valid Block
pub type LVB_R = crate::FieldReader<u8, u8>;
///Field `LVB` writer - Last Valid Block
pub type LVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT894L_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:15 - First Valid Block
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Last Valid Block
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 8:15 - First Valid Block
    #[inline(always)]
    #[must_use]
    pub fn fvb(&mut self) -> FVB_W<8> {
        FVB_W::new(self)
    }
    ///Bits 16:23 - Last Valid Block
    #[inline(always)]
    #[must_use]
    pub fn lvb(&mut self) -> LVB_W<16> {
        LVB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Graphic MMU LUT entry 894 low
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lut894l](index.html) module
pub struct LUT894L_SPEC;
impl crate::RegisterSpec for LUT894L_SPEC {
    type Ux = u32;
}
///`read()` method returns [lut894l::R](R) reader structure
impl crate::Readable for LUT894L_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lut894l::W](W) writer structure
impl crate::Writable for LUT894L_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LUT894L to value 0
impl crate::Resettable for LUT894L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
