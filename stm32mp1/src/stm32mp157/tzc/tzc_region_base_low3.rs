///Register `TZC_REGION_BASE_LOW3` reader
pub struct R(crate::R<TZC_REGION_BASE_LOW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_BASE_LOW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_BASE_LOW3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_BASE_LOW3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZC_REGION_BASE_LOW3` writer
pub struct W(crate::W<TZC_REGION_BASE_LOW3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_BASE_LOW3_SPEC>;
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
impl From<crate::W<TZC_REGION_BASE_LOW3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_BASE_LOW3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BASE_ADDRESS_LOW` reader - BASE_ADDRESS_LOW
pub type BASE_ADDRESS_LOW_R = crate::FieldReader<u32, u32>;
///Field `BASE_ADDRESS_LOW` writer - BASE_ADDRESS_LOW
pub type BASE_ADDRESS_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_BASE_LOW3_SPEC, u32, u32, 20, O>;
impl R {
    ///Bits 12:31 - BASE_ADDRESS_LOW
    #[inline(always)]
    pub fn base_address_low(&self) -> BASE_ADDRESS_LOW_R {
        BASE_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    ///Bits 12:31 - BASE_ADDRESS_LOW
    #[inline(always)]
    #[must_use]
    pub fn base_address_low(&mut self) -> BASE_ADDRESS_LOW_W<12> {
        BASE_ADDRESS_LOW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Base address low for regions 1 to 8.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_region_base_low3](index.html) module
pub struct TZC_REGION_BASE_LOW3_SPEC;
impl crate::RegisterSpec for TZC_REGION_BASE_LOW3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_region_base_low3::R](R) reader structure
impl crate::Readable for TZC_REGION_BASE_LOW3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzc_region_base_low3::W](W) writer structure
impl crate::Writable for TZC_REGION_BASE_LOW3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZC_REGION_BASE_LOW3 to value 0
impl crate::Resettable for TZC_REGION_BASE_LOW3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
