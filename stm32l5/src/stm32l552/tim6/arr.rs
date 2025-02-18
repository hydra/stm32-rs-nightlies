///Register `ARR` reader
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ARR` writer
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARR_bit0` reader - ARR_bit0
pub type ARR_BIT0_R = crate::FieldReader<u16, u16>;
///Field `ARR_bit0` writer - ARR_bit0
pub type ARR_BIT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - ARR_bit0
    #[inline(always)]
    pub fn arr_bit0(&self) -> ARR_BIT0_R {
        ARR_BIT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - ARR_bit0
    #[inline(always)]
    #[must_use]
    pub fn arr_bit0(&mut self) -> ARR_BIT0_W<0> {
        ARR_BIT0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///auto-reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [arr](index.html) module
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
///`read()` method returns [arr::R](R) reader structure
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [arr::W](W) writer structure
impl crate::Writable for ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ARR to value 0xffff_ffff
impl crate::Resettable for ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
