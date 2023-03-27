///Register `TZC_REGION_ID_ACCESS8` reader
pub struct R(crate::R<TZC_REGION_ID_ACCESS8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_ID_ACCESS8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_ID_ACCESS8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_ID_ACCESS8_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZC_REGION_ID_ACCESS8` writer
pub struct W(crate::W<TZC_REGION_ID_ACCESS8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_ID_ACCESS8_SPEC>;
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
impl From<crate::W<TZC_REGION_ID_ACCESS8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_ID_ACCESS8_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NSAID_RD_EN` reader - NSAID_RD_EN
pub type NSAID_RD_EN_R = crate::FieldReader<u16, u16>;
///Field `NSAID_RD_EN` writer - NSAID_RD_EN
pub type NSAID_RD_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_ID_ACCESS8_SPEC, u16, u16, 16, O>;
///Field `NSAID_WR_EN` reader - NSAID_WR_EN
pub type NSAID_WR_EN_R = crate::FieldReader<u16, u16>;
///Field `NSAID_WR_EN` writer - NSAID_WR_EN
pub type NSAID_WR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_REGION_ID_ACCESS8_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - NSAID_RD_EN
    #[inline(always)]
    pub fn nsaid_rd_en(&self) -> NSAID_RD_EN_R {
        NSAID_RD_EN_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - NSAID_WR_EN
    #[inline(always)]
    pub fn nsaid_wr_en(&self) -> NSAID_WR_EN_R {
        NSAID_WR_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - NSAID_RD_EN
    #[inline(always)]
    #[must_use]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W<0> {
        NSAID_RD_EN_W::new(self)
    }
    ///Bits 16:31 - NSAID_WR_EN
    #[inline(always)]
    #[must_use]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W<16> {
        NSAID_WR_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Region non-secure access based on NSAID.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_region_id_access8](index.html) module
pub struct TZC_REGION_ID_ACCESS8_SPEC;
impl crate::RegisterSpec for TZC_REGION_ID_ACCESS8_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_region_id_access8::R](R) reader structure
impl crate::Readable for TZC_REGION_ID_ACCESS8_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzc_region_id_access8::W](W) writer structure
impl crate::Writable for TZC_REGION_ID_ACCESS8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZC_REGION_ID_ACCESS8 to value 0
impl crate::Resettable for TZC_REGION_ID_ACCESS8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
