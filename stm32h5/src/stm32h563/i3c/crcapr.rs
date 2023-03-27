///Register `CRCAPR` reader
pub struct R(crate::R<CRCAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCAPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRCAPR` writer
pub struct W(crate::W<CRCAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCAPR_SPEC>;
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
impl From<crate::W<CRCAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCAPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAPDHOFF` reader - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
pub type CAPDHOFF_R = crate::BitReader<bool>;
///Field `CAPDHOFF` writer - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
pub type CAPDHOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCAPR_SPEC, bool, O>;
///Field `CAPGRP` reader - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
pub type CAPGRP_R = crate::BitReader<bool>;
///Field `CAPGRP` writer - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
pub type CAPGRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRCAPR_SPEC, bool, O>;
impl R {
    ///Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    pub fn capdhoff(&self) -> CAPDHOFF_R {
        CAPDHOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    pub fn capgrp(&self) -> CAPGRP_R {
        CAPGRP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - delayed controller-role hand-off This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if this target I3C may need additional time to process a controller-role hand-off requested by the current controller. This bit is used to return the CRCAP2 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    #[must_use]
    pub fn capdhoff(&mut self) -> CAPDHOFF_W<3> {
        CAPDHOFF_W::new(self)
    }
    ///Bit 9 - group management support (when acting as controller) This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates if the I3C is able to support group management when it acts as a controller (after controller-role hand-off) via emitted DEFGRPA, RSTGRPA, and SETGRPA CCC. This bit is used to return the CRCAP1 byte in response to the GETCAPS CCC format 2.
    #[inline(always)]
    #[must_use]
    pub fn capgrp(&mut self) -> CAPGRP_W<9> {
        CAPGRP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C controller-role capability register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crcapr](index.html) module
pub struct CRCAPR_SPEC;
impl crate::RegisterSpec for CRCAPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [crcapr::R](R) reader structure
impl crate::Readable for CRCAPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crcapr::W](W) writer structure
impl crate::Writable for CRCAPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRCAPR to value 0
impl crate::Resettable for CRCAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
