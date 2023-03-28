///Register `RCC_MP_TZAHB6LPENSETR` reader
pub struct R(crate::R<RCC_MP_TZAHB6LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_TZAHB6LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_TZAHB6LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_TZAHB6LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_TZAHB6LPENSETR` writer
pub struct W(crate::W<RCC_MP_TZAHB6LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_TZAHB6LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_TZAHB6LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_TZAHB6LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDMALPEN` reader - MDMALPEN
pub type MDMALPEN_R = crate::BitReader<bool>;
///Field `MDMALPEN` writer - MDMALPEN
pub type MDMALPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_TZAHB6LPENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMALPEN
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<0> {
        MDMALPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_tzahb6lpensetr](index.html) module
pub struct RCC_MP_TZAHB6LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_TZAHB6LPENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_tzahb6lpensetr::R](R) reader structure
impl crate::Readable for RCC_MP_TZAHB6LPENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_tzahb6lpensetr::W](W) writer structure
impl crate::Writable for RCC_MP_TZAHB6LPENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_TZAHB6LPENSETR to value 0x01
impl crate::Resettable for RCC_MP_TZAHB6LPENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
