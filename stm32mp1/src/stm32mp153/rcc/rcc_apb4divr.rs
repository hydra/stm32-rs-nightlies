///Register `RCC_APB4DIVR` reader
pub struct R(crate::R<RCC_APB4DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB4DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB4DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB4DIVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB4DIVR` writer
pub struct W(crate::W<RCC_APB4DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB4DIVR_SPEC>;
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
impl From<crate::W<RCC_APB4DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB4DIVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `APB4DIV` reader - APB4DIV
pub type APB4DIV_R = crate::FieldReader<u8, u8>;
///Field `APB4DIV` writer - APB4DIV
pub type APB4DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_APB4DIVR_SPEC, u8, u8, 3, O>;
///Field `APB4DIVRDY` reader - APB4DIVRDY
pub type APB4DIVRDY_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - APB4DIV
    #[inline(always)]
    pub fn apb4div(&self) -> APB4DIV_R {
        APB4DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB4DIVRDY
    #[inline(always)]
    pub fn apb4divrdy(&self) -> APB4DIVRDY_R {
        APB4DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - APB4DIV
    #[inline(always)]
    #[must_use]
    pub fn apb4div(&mut self) -> APB4DIV_W<0> {
        APB4DIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb4divr](index.html) module
pub struct RCC_APB4DIVR_SPEC;
impl crate::RegisterSpec for RCC_APB4DIVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb4divr::R](R) reader structure
impl crate::Readable for RCC_APB4DIVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb4divr::W](W) writer structure
impl crate::Writable for RCC_APB4DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB4DIVR to value 0x8000_0000
impl crate::Resettable for RCC_APB4DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
