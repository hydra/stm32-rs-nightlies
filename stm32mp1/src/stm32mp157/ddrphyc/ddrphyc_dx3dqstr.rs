///Register `DDRPHYC_DX3DQSTR` reader
pub struct R(crate::R<DDRPHYC_DX3DQSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3DQSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3DQSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3DQSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_DX3DQSTR` writer
pub struct W(crate::W<DDRPHYC_DX3DQSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX3DQSTR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX3DQSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX3DQSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `R0DGSL` reader - R0DGSL
pub type R0DGSL_R = crate::FieldReader<u8, u8>;
///Field `R0DGSL` writer - R0DGSL
pub type R0DGSL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX3DQSTR_SPEC, u8, u8, 3, O>;
///Field `R0DGPS` reader - R0DGPS
pub type R0DGPS_R = crate::FieldReader<u8, u8>;
///Field `R0DGPS` writer - R0DGPS
pub type R0DGPS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX3DQSTR_SPEC, u8, u8, 2, O>;
///Field `DQSDLY` reader - DQSDLY
pub type DQSDLY_R = crate::FieldReader<u8, u8>;
///Field `DQSDLY` writer - DQSDLY
pub type DQSDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX3DQSTR_SPEC, u8, u8, 3, O>;
///Field `DQSNDLY` reader - DQSNDLY
pub type DQSNDLY_R = crate::FieldReader<u8, u8>;
///Field `DQSNDLY` writer - DQSNDLY
pub type DQSNDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX3DQSTR_SPEC, u8, u8, 3, O>;
///Field `DMDLY` reader - DMDLY
pub type DMDLY_R = crate::FieldReader<u8, u8>;
///Field `DMDLY` writer - DMDLY
pub type DMDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_DX3DQSTR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:2 - R0DGSL
    #[inline(always)]
    pub fn r0dgsl(&self) -> R0DGSL_R {
        R0DGSL_R::new((self.bits & 7) as u8)
    }
    ///Bits 12:13 - R0DGPS
    #[inline(always)]
    pub fn r0dgps(&self) -> R0DGPS_R {
        R0DGPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 20:22 - DQSDLY
    #[inline(always)]
    pub fn dqsdly(&self) -> DQSDLY_R {
        DQSDLY_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - DQSNDLY
    #[inline(always)]
    pub fn dqsndly(&self) -> DQSNDLY_R {
        DQSNDLY_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:29 - DMDLY
    #[inline(always)]
    pub fn dmdly(&self) -> DMDLY_R {
        DMDLY_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:2 - R0DGSL
    #[inline(always)]
    #[must_use]
    pub fn r0dgsl(&mut self) -> R0DGSL_W<0> {
        R0DGSL_W::new(self)
    }
    ///Bits 12:13 - R0DGPS
    #[inline(always)]
    #[must_use]
    pub fn r0dgps(&mut self) -> R0DGPS_W<12> {
        R0DGPS_W::new(self)
    }
    ///Bits 20:22 - DQSDLY
    #[inline(always)]
    #[must_use]
    pub fn dqsdly(&mut self) -> DQSDLY_W<20> {
        DQSDLY_W::new(self)
    }
    ///Bits 23:25 - DQSNDLY
    #[inline(always)]
    #[must_use]
    pub fn dqsndly(&mut self) -> DQSNDLY_W<23> {
        DQSNDLY_W::new(self)
    }
    ///Bits 26:29 - DMDLY
    #[inline(always)]
    #[must_use]
    pub fn dmdly(&mut self) -> DMDLY_W<26> {
        DMDLY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC byte lane 3 DQST register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dx3dqstr](index.html) module
pub struct DDRPHYC_DX3DQSTR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3DQSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dx3dqstr::R](R) reader structure
impl crate::Readable for DDRPHYC_DX3DQSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_dx3dqstr::W](W) writer structure
impl crate::Writable for DDRPHYC_DX3DQSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_DX3DQSTR to value 0x3db0_2000
impl crate::Resettable for DDRPHYC_DX3DQSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3db0_2000;
}
