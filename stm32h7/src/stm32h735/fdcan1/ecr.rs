///Register `ECR` reader
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ECR` writer
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TEC` reader - Transmit Error Counter
pub type TEC_R = crate::FieldReader<u8, u8>;
///Field `TEC` writer - Transmit Error Counter
pub type TEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
///Field `REC` reader - Receive Error Counter
pub type REC_R = crate::FieldReader<u8, u8>;
///Field `REC` writer - Receive Error Counter
pub type REC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 7, O>;
///Field `RP` reader - Receive Error Passive
pub type RP_R = crate::BitReader<bool>;
///Field `RP` writer - Receive Error Passive
pub type RP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
///Field `CEL` reader - AN Error Logging
pub type CEL_R = crate::FieldReader<u8, u8>;
///Field `CEL` writer - AN Error Logging
pub type CEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
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
    ///Bits 0:7 - Transmit Error Counter
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<0> {
        TEC_W::new(self)
    }
    ///Bits 8:14 - Receive Error Counter
    #[inline(always)]
    #[must_use]
    pub fn rec(&mut self) -> REC_W<8> {
        REC_W::new(self)
    }
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
///For information about available fields see [ecr](index.html) module
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ecr::R](R) reader structure
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ecr::W](W) writer structure
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
