///Register `AXIMC_FN_MOD_LB` reader
pub struct R(crate::R<AXIMC_FN_MOD_LB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_FN_MOD_LB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_FN_MOD_LB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_FN_MOD_LB_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AXIMC_FN_MOD_LB` writer
pub struct W(crate::W<AXIMC_FN_MOD_LB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXIMC_FN_MOD_LB_SPEC>;
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
impl From<crate::W<AXIMC_FN_MOD_LB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXIMC_FN_MOD_LB_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FN_MOD_LB` reader - FN_MOD_LB
pub type FN_MOD_LB_R = crate::BitReader<bool>;
///Field `FN_MOD_LB` writer - FN_MOD_LB
pub type FN_MOD_LB_W<'a, const O: u8> = crate::BitWriter<'a, u32, AXIMC_FN_MOD_LB_SPEC, bool, O>;
impl R {
    ///Bit 0 - FN_MOD_LB
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FN_MOD_LB
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<0> {
        FN_MOD_LB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AXIMC long burst capability inhibition register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_fn_mod_lb](index.html) module
pub struct AXIMC_FN_MOD_LB_SPEC;
impl crate::RegisterSpec for AXIMC_FN_MOD_LB_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_fn_mod_lb::R](R) reader structure
impl crate::Readable for AXIMC_FN_MOD_LB_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [aximc_fn_mod_lb::W](W) writer structure
impl crate::Writable for AXIMC_FN_MOD_LB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AXIMC_FN_MOD_LB to value 0
impl crate::Resettable for AXIMC_FN_MOD_LB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
