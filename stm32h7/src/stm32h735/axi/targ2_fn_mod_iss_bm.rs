///Register `TARG2_FN_MOD_ISS_BM` reader
pub struct R(crate::R<TARG2_FN_MOD_ISS_BM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARG2_FN_MOD_ISS_BM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARG2_FN_MOD_ISS_BM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARG2_FN_MOD_ISS_BM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TARG2_FN_MOD_ISS_BM` writer
pub struct W(crate::W<TARG2_FN_MOD_ISS_BM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARG2_FN_MOD_ISS_BM_SPEC>;
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
impl From<crate::W<TARG2_FN_MOD_ISS_BM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARG2_FN_MOD_ISS_BM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_R = crate::BitReader<bool>;
///Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE
pub type READ_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TARG2_FN_MOD_ISS_BM_SPEC, bool, O>;
///Field `WRITE_ISS_OVERRIDE` reader - Switch matrix write issuing override for target
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader<bool>;
///Field `WRITE_ISS_OVERRIDE` writer - Switch matrix write issuing override for target
pub type WRITE_ISS_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TARG2_FN_MOD_ISS_BM_SPEC, bool, O>;
impl R {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Switch matrix write issuing override for target
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - READ_ISS_OVERRIDE
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<0> {
        READ_ISS_OVERRIDE_W::new(self)
    }
    ///Bit 1 - Switch matrix write issuing override for target
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<1> {
        WRITE_ISS_OVERRIDE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AXI interconnect - TARG x bus matrix issuing functionality register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [targ2_fn_mod_iss_bm](index.html) module
pub struct TARG2_FN_MOD_ISS_BM_SPEC;
impl crate::RegisterSpec for TARG2_FN_MOD_ISS_BM_SPEC {
    type Ux = u32;
}
///`read()` method returns [targ2_fn_mod_iss_bm::R](R) reader structure
impl crate::Readable for TARG2_FN_MOD_ISS_BM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [targ2_fn_mod_iss_bm::W](W) writer structure
impl crate::Writable for TARG2_FN_MOD_ISS_BM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TARG2_FN_MOD_ISS_BM to value 0x04
impl crate::Resettable for TARG2_FN_MOD_ISS_BM_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
