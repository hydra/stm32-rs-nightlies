///Register `UCPD_CFGR3` reader
pub struct R(crate::R<UCPD_CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCPD_CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCPD_CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCPD_CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `UCPD_CFGR3` writer
pub struct W(crate::W<UCPD_CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCPD_CFGR3_SPEC>;
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
impl From<crate::W<UCPD_CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCPD_CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIM1_NG_CCRPD` reader - SW trim value for RPD resistors on the CC1 line
pub type TRIM1_NG_CCRPD_R = crate::FieldReader<u8, u8>;
///Field `TRIM1_NG_CCRPD` writer - SW trim value for RPD resistors on the CC1 line
pub type TRIM1_NG_CCRPD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_CFGR3_SPEC, u8, u8, 4, O>;
///Field `TRIM1_NG_CC3A0` reader - SW trim value for Iref on the CC1 line
pub type TRIM1_NG_CC3A0_R = crate::FieldReader<u8, u8>;
///Field `TRIM1_NG_CC3A0` writer - SW trim value for Iref on the CC1 line
pub type TRIM1_NG_CC3A0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_CFGR3_SPEC, u8, u8, 4, O>;
///Field `TRIM2_NG_CCRPD` reader - SW trim value for RPD resistors on the CC2 line
pub type TRIM2_NG_CCRPD_R = crate::FieldReader<u8, u8>;
///Field `TRIM2_NG_CCRPD` writer - SW trim value for RPD resistors on the CC2 line
pub type TRIM2_NG_CCRPD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_CFGR3_SPEC, u8, u8, 4, O>;
///Field `TRIM2_NG_CC3A0` reader - SW trim value for Iref on the CC2 line
pub type TRIM2_NG_CC3A0_R = crate::FieldReader<u8, u8>;
///Field `TRIM2_NG_CC3A0` writer - SW trim value for Iref on the CC2 line
pub type TRIM2_NG_CC3A0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_CFGR3_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - SW trim value for RPD resistors on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 9:12 - SW trim value for Iref on the CC1 line
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 16:19 - SW trim value for RPD resistors on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 25:28 - SW trim value for Iref on the CC2 line
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - SW trim value for RPD resistors on the CC1 line
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W<0> {
        TRIM1_NG_CCRPD_W::new(self)
    }
    ///Bits 9:12 - SW trim value for Iref on the CC1 line
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W<9> {
        TRIM1_NG_CC3A0_W::new(self)
    }
    ///Bits 16:19 - SW trim value for RPD resistors on the CC2 line
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W<16> {
        TRIM2_NG_CCRPD_W::new(self)
    }
    ///Bits 25:28 - SW trim value for Iref on the CC2 line
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W<25> {
        TRIM2_NG_CC3A0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ucpd_cfgr3](index.html) module
pub struct UCPD_CFGR3_SPEC;
impl crate::RegisterSpec for UCPD_CFGR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ucpd_cfgr3::R](R) reader structure
impl crate::Readable for UCPD_CFGR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ucpd_cfgr3::W](W) writer structure
impl crate::Writable for UCPD_CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets UCPD_CFGR3 to value 0
impl crate::Resettable for UCPD_CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
