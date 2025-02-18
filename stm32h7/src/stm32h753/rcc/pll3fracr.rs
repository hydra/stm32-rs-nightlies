///Register `PLL3FRACR` reader
pub struct R(crate::R<PLL3FRACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL3FRACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL3FRACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL3FRACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLL3FRACR` writer
pub struct W(crate::W<PLL3FRACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL3FRACR_SPEC>;
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
impl From<crate::W<PLL3FRACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL3FRACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FRACN3` reader - Fractional part of the multiplication factor for PLL3 VCO
pub type FRACN3_R = crate::FieldReader<u16, u16>;
///Field `FRACN3` writer - Fractional part of the multiplication factor for PLL3 VCO
pub type FRACN3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL3FRACR_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO
    #[inline(always)]
    pub fn fracn3(&self) -> FRACN3_R {
        FRACN3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO
    #[inline(always)]
    #[must_use]
    pub fn fracn3(&mut self) -> FRACN3_W<3> {
        FRACN3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL3 Fractional Divider Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pll3fracr](index.html) module
pub struct PLL3FRACR_SPEC;
impl crate::RegisterSpec for PLL3FRACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pll3fracr::R](R) reader structure
impl crate::Readable for PLL3FRACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pll3fracr::W](W) writer structure
impl crate::Writable for PLL3FRACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLL3FRACR to value 0
impl crate::Resettable for PLL3FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
