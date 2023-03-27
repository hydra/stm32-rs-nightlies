///Register `DDRCTRL_PERFLPR1` reader
pub struct R(crate::R<DDRCTRL_PERFLPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PERFLPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PERFLPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PERFLPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_PERFLPR1` writer
pub struct W(crate::W<DDRCTRL_PERFLPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PERFLPR1_SPEC>;
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
impl From<crate::W<DDRCTRL_PERFLPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PERFLPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPR_MAX_STARVE` reader - LPR_MAX_STARVE
pub type LPR_MAX_STARVE_R = crate::FieldReader<u16, u16>;
///Field `LPR_MAX_STARVE` writer - LPR_MAX_STARVE
pub type LPR_MAX_STARVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFLPR1_SPEC, u16, u16, 16, O>;
///Field `LPR_XACT_RUN_LENGTH` reader - LPR_XACT_RUN_LENGTH
pub type LPR_XACT_RUN_LENGTH_R = crate::FieldReader<u8, u8>;
///Field `LPR_XACT_RUN_LENGTH` writer - LPR_XACT_RUN_LENGTH
pub type LPR_XACT_RUN_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PERFLPR1_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:15 - LPR_MAX_STARVE
    #[inline(always)]
    pub fn lpr_max_starve(&self) -> LPR_MAX_STARVE_R {
        LPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 24:31 - LPR_XACT_RUN_LENGTH
    #[inline(always)]
    pub fn lpr_xact_run_length(&self) -> LPR_XACT_RUN_LENGTH_R {
        LPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:15 - LPR_MAX_STARVE
    #[inline(always)]
    #[must_use]
    pub fn lpr_max_starve(&mut self) -> LPR_MAX_STARVE_W<0> {
        LPR_MAX_STARVE_W::new(self)
    }
    ///Bits 24:31 - LPR_XACT_RUN_LENGTH
    #[inline(always)]
    #[must_use]
    pub fn lpr_xact_run_length(&mut self) -> LPR_XACT_RUN_LENGTH_W<24> {
        LPR_XACT_RUN_LENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL low priority read CAM register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_perflpr1](index.html) module
pub struct DDRCTRL_PERFLPR1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PERFLPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_perflpr1::R](R) reader structure
impl crate::Readable for DDRCTRL_PERFLPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_perflpr1::W](W) writer structure
impl crate::Writable for DDRCTRL_PERFLPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_PERFLPR1 to value 0x0f00_007f
impl crate::Resettable for DDRCTRL_PERFLPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_007f;
}
