///Register `FDCAN_ECR` reader
pub struct R(crate::R<FDCAN_ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_ECR` writer
pub struct W(crate::W<FDCAN_ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_ECR_SPEC>;
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
impl From<crate::W<FDCAN_ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_ECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TEC` reader - Transmit Error Counter
pub type TEC_R = crate::FieldReader<u8, u8>;
///Field `REC` reader - Receive Error Counter
pub type REC_R = crate::FieldReader<u8, u8>;
///Field `RP` reader - Receive Error Passive
pub type RP_R = crate::BitReader<bool>;
///Field `RP` writer - Receive Error Passive
pub type RP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_ECR_SPEC, bool, O>;
///Field `CEL` reader - AN Error Logging
pub type CEL_R = crate::FieldReader<u8, u8>;
///Field `CEL` writer - AN Error Logging
pub type CEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_ECR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Transmit Error Counter
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Receive Error Counter
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 15 - Receive Error Passive
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 15 - Receive Error Passive
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<15> {
        RP_W::new(self)
    }
    ///Bits 16:23 - AN Error Logging
    #[inline(always)]
    #[must_use]
    pub fn cel(&mut self) -> CEL_W<16> {
        CEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Error Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ecr](index.html) module
pub struct FDCAN_ECR_SPEC;
impl crate::RegisterSpec for FDCAN_ECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ecr::R](R) reader structure
impl crate::Readable for FDCAN_ECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_ecr::W](W) writer structure
impl crate::Writable for FDCAN_ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_ECR to value 0
impl crate::Resettable for FDCAN_ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
