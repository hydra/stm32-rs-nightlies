///Register `AXIMC_M0_WRITE_QOS` reader
pub struct R(crate::R<AXIMC_M0_WRITE_QOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_M0_WRITE_QOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_M0_WRITE_QOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_M0_WRITE_QOS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AXIMC_M0_WRITE_QOS` writer
pub struct W(crate::W<AXIMC_M0_WRITE_QOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXIMC_M0_WRITE_QOS_SPEC>;
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
impl From<crate::W<AXIMC_M0_WRITE_QOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXIMC_M0_WRITE_QOS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AW_QOS` reader - AW_QOS
pub type AW_QOS_R = crate::FieldReader<u8, u8>;
///Field `AW_QOS` writer - AW_QOS
pub type AW_QOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXIMC_M0_WRITE_QOS_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - AW_QOS
    #[inline(always)]
    pub fn aw_qos(&self) -> AW_QOS_R {
        AW_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - AW_QOS
    #[inline(always)]
    #[must_use]
    pub fn aw_qos(&mut self) -> AW_QOS_W<0> {
        AW_QOS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AXIMC master 0 write priority register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aximc_m0_write_qos](index.html) module
pub struct AXIMC_M0_WRITE_QOS_SPEC;
impl crate::RegisterSpec for AXIMC_M0_WRITE_QOS_SPEC {
    type Ux = u32;
}
///`read()` method returns [aximc_m0_write_qos::R](R) reader structure
impl crate::Readable for AXIMC_M0_WRITE_QOS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [aximc_m0_write_qos::W](W) writer structure
impl crate::Writable for AXIMC_M0_WRITE_QOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AXIMC_M0_WRITE_QOS to value 0x06
impl crate::Resettable for AXIMC_M0_WRITE_QOS_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
