///Register `ACR` reader
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ACR` writer
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<u8, u8>;
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, u8, 4, O>;
///Field `RUN_PD` reader - Flash Power-down mode during Low-power run mode
pub type RUN_PD_R = crate::BitReader<bool>;
///Field `RUN_PD` writer - Flash Power-down mode during Low-power run mode
pub type RUN_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `SLEEP_PD` reader - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_R = crate::BitReader<bool>;
///Field `SLEEP_PD` writer - Flash Power-down mode during Low-power sleep mode
pub type SLEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `LVEN` reader - LVEN
pub type LVEN_R = crate::BitReader<bool>;
///Field `LVEN` writer - LVEN
pub type LVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - LVEN
    #[inline(always)]
    pub fn lven(&self) -> LVEN_R {
        LVEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    ///Bit 13 - Flash Power-down mode during Low-power run mode
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<13> {
        RUN_PD_W::new(self)
    }
    ///Bit 14 - Flash Power-down mode during Low-power sleep mode
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<14> {
        SLEEP_PD_W::new(self)
    }
    ///Bit 15 - LVEN
    #[inline(always)]
    #[must_use]
    pub fn lven(&mut self) -> LVEN_W<15> {
        LVEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr](index.html) module
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [acr::R](R) reader structure
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [acr::W](W) writer structure
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
