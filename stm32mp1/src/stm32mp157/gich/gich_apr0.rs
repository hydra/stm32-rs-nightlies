///Register `GICH_APR0` reader
pub struct R(crate::R<GICH_APR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_APR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_APR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_APR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICH_APR0` writer
pub struct W(crate::W<GICH_APR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICH_APR0_SPEC>;
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
impl From<crate::W<GICH_APR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICH_APR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `APR0` reader - APR0
pub type APR0_R = crate::FieldReader<u32, u32>;
///Field `APR0` writer - APR0
pub type APR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICH_APR0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - APR0
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - APR0
    #[inline(always)]
    #[must_use]
    pub fn apr0(&mut self) -> APR0_W<0> {
        APR0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GICH active priority register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_apr0](index.html) module
pub struct GICH_APR0_SPEC;
impl crate::RegisterSpec for GICH_APR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_apr0::R](R) reader structure
impl crate::Readable for GICH_APR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gich_apr0::W](W) writer structure
impl crate::Writable for GICH_APR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICH_APR0 to value 0
impl crate::Resettable for GICH_APR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
