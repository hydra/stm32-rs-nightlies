///Register `TARG1_FN_MOD2` reader
pub struct R(crate::R<TARG1_FN_MOD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARG1_FN_MOD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARG1_FN_MOD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARG1_FN_MOD2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TARG1_FN_MOD2` writer
pub struct W(crate::W<TARG1_FN_MOD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARG1_FN_MOD2_SPEC>;
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
impl From<crate::W<TARG1_FN_MOD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARG1_FN_MOD2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BYPASS_MERGE` reader - Disable packing of beats to match the output data width
pub type BYPASS_MERGE_R = crate::BitReader<bool>;
///Field `BYPASS_MERGE` writer - Disable packing of beats to match the output data width
pub type BYPASS_MERGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TARG1_FN_MOD2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Disable packing of beats to match the output data width
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Disable packing of beats to match the output data width
    #[inline(always)]
    #[must_use]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<0> {
        BYPASS_MERGE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AXI interconnect - TARG x bus matrix functionality 2 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [targ1_fn_mod2](index.html) module
pub struct TARG1_FN_MOD2_SPEC;
impl crate::RegisterSpec for TARG1_FN_MOD2_SPEC {
    type Ux = u32;
}
///`read()` method returns [targ1_fn_mod2::R](R) reader structure
impl crate::Readable for TARG1_FN_MOD2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [targ1_fn_mod2::W](W) writer structure
impl crate::Writable for TARG1_FN_MOD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TARG1_FN_MOD2 to value 0x04
impl crate::Resettable for TARG1_FN_MOD2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
