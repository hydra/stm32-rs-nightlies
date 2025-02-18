///Register `LPOTR` reader
pub struct R(crate::R<LPOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPOTR` writer
pub struct W(crate::W<LPOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPOTR_SPEC>;
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
impl From<crate::W<LPOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AO1_OPT_OFFSET_TRIM_LP` reader - OPAMP1, 10-bit offset trim value for low power mode
pub type AO1_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16, u16>;
///Field `AO1_OPT_OFFSET_TRIM_LP` writer - OPAMP1, 10-bit offset trim value for low power mode
pub type AO1_OPT_OFFSET_TRIM_LP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPOTR_SPEC, u16, u16, 10, O>;
///Field `AO2_OPT_OFFSET_TRIM_LP` reader - OPAMP2, 10-bit offset trim value for low power mode
pub type AO2_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16, u16>;
///Field `AO2_OPT_OFFSET_TRIM_LP` writer - OPAMP2, 10-bit offset trim value for low power mode
pub type AO2_OPT_OFFSET_TRIM_LP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPOTR_SPEC, u16, u16, 10, O>;
///Field `AO3_OPT_OFFSET_TRIM_LP` reader - OPAMP3, 10-bit offset trim value for low power mode
pub type AO3_OPT_OFFSET_TRIM_LP_R = crate::FieldReader<u16, u16>;
///Field `AO3_OPT_OFFSET_TRIM_LP` writer - OPAMP3, 10-bit offset trim value for low power mode
pub type AO3_OPT_OFFSET_TRIM_LP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPOTR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&self) -> AO1_OPT_OFFSET_TRIM_LP_R {
        AO1_OPT_OFFSET_TRIM_LP_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&self) -> AO2_OPT_OFFSET_TRIM_LP_R {
        AO2_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&self) -> AO3_OPT_OFFSET_TRIM_LP_R {
        AO3_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode
    #[inline(always)]
    #[must_use]
    pub fn ao1_opt_offset_trim_lp(&mut self) -> AO1_OPT_OFFSET_TRIM_LP_W<0> {
        AO1_OPT_OFFSET_TRIM_LP_W::new(self)
    }
    ///Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode
    #[inline(always)]
    #[must_use]
    pub fn ao2_opt_offset_trim_lp(&mut self) -> AO2_OPT_OFFSET_TRIM_LP_W<10> {
        AO2_OPT_OFFSET_TRIM_LP_W::new(self)
    }
    ///Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode
    #[inline(always)]
    #[must_use]
    pub fn ao3_opt_offset_trim_lp(&mut self) -> AO3_OPT_OFFSET_TRIM_LP_W<20> {
        AO3_OPT_OFFSET_TRIM_LP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP offset trimming register for low power mode
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpotr](index.html) module
pub struct LPOTR_SPEC;
impl crate::RegisterSpec for LPOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpotr::R](R) reader structure
impl crate::Readable for LPOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpotr::W](W) writer structure
impl crate::Writable for LPOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPOTR to value 0
impl crate::Resettable for LPOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
