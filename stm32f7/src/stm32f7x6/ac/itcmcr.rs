///Register `ITCMCR` reader
pub struct R(crate::R<ITCMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITCMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITCMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ITCMCR` writer
pub struct W(crate::W<ITCMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITCMCR_SPEC>;
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
impl From<crate::W<ITCMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITCMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - EN
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITCMCR_SPEC, bool, O>;
///Field `RMW` reader - RMW
pub type RMW_R = crate::BitReader<bool>;
///Field `RMW` writer - RMW
pub type RMW_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITCMCR_SPEC, bool, O>;
///Field `RETEN` reader - RETEN
pub type RETEN_R = crate::BitReader<bool>;
///Field `RETEN` writer - RETEN
pub type RETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ITCMCR_SPEC, bool, O>;
///Field `SZ` reader - SZ
pub type SZ_R = crate::FieldReader<u8, u8>;
///Field `SZ` writer - SZ
pub type SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ITCMCR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RMW
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - RMW
    #[inline(always)]
    #[must_use]
    pub fn rmw(&mut self) -> RMW_W<1> {
        RMW_W::new(self)
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    #[must_use]
    pub fn reten(&mut self) -> RETEN_W<2> {
        RETEN_W::new(self)
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<3> {
        SZ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Instruction and Data Tightly-Coupled Memory Control Registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itcmcr](index.html) module
pub struct ITCMCR_SPEC;
impl crate::RegisterSpec for ITCMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [itcmcr::R](R) reader structure
impl crate::Readable for ITCMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [itcmcr::W](W) writer structure
impl crate::Writable for ITCMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ITCMCR to value 0
impl crate::Resettable for ITCMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
