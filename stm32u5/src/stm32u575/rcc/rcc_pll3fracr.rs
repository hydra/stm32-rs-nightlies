///Register `RCC_PLL3FRACR` reader
pub struct R(crate::R<RCC_PLL3FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL3FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL3FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL3FRACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL3FRACR` writer
pub struct W(crate::W<RCC_PLL3FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL3FRACR_SPEC>;
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
impl From<crate::W<RCC_PLL3FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL3FRACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLL3FRACN` reader - Fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 213 - 1. The input frequency Fref3_ck must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
pub type PLL3FRACN_R = crate::FieldReader<u16, u16>;
///Field `PLL3FRACN` writer - Fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 213 - 1. The input frequency Fref3_ck must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
pub type PLL3FRACN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_PLL3FRACR_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 213 - 1. The input frequency Fref3_ck must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
    #[inline(always)]
    pub fn pll3fracn(&self) -> PLL3FRACN_R {
        PLL3FRACN_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO. VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with: PLL3N must be between 4 and 512. PLL3FRACN can be between 0 and 213 - 1. The input frequency Fref3_ck must be between 4 and 16 MHz. In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL3FRACEN to 0. Write the new fractional value into PLL3FRACN. Set the bit PLL3FRACEN to 1.
    #[inline(always)]
    #[must_use]
    pub fn pll3fracn(&mut self) -> PLL3FRACN_W<3> {
        PLL3FRACN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL3 fractional divider register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll3fracr](index.html) module
pub struct RCC_PLL3FRACR_SPEC;
impl crate::RegisterSpec for RCC_PLL3FRACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll3fracr::R](R) reader structure
impl crate::Readable for RCC_PLL3FRACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll3fracr::W](W) writer structure
impl crate::Writable for RCC_PLL3FRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL3FRACR to value 0
impl crate::Resettable for RCC_PLL3FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
