///Register `L1CACR` reader
pub struct R(crate::R<L1CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1CACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `L1CACR` writer
pub struct W(crate::W<L1CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CACR_SPEC>;
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
impl From<crate::W<L1CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CONSTA` reader - Constant Alpha
pub type CONSTA_R = crate::FieldReader<u8, u8>;
///Field `CONSTA` writer - Constant Alpha
pub type CONSTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L1CACR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Constant Alpha
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Constant Alpha
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<0> {
        CONSTA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layer Constant Alpha Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1cacr](index.html) module
pub struct L1CACR_SPEC;
impl crate::RegisterSpec for L1CACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [l1cacr::R](R) reader structure
impl crate::Readable for L1CACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [l1cacr::W](W) writer structure
impl crate::Writable for L1CACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L1CACR to value 0
impl crate::Resettable for L1CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
