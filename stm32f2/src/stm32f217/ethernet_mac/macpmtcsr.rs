///Register `MACPMTCSR` reader
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACPMTCSR` writer
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD` reader - Power down
pub type PD_R = crate::BitReader<bool>;
///Field `PD` writer - Power down
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `MPE` reader - Magic Packet enable
pub type MPE_R = crate::BitReader<bool>;
///Field `MPE` writer - Magic Packet enable
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `WFE` reader - Wakeup frame enable
pub type WFE_R = crate::BitReader<bool>;
///Field `WFE` writer - Wakeup frame enable
pub type WFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `MPR` reader - Magic packet received
pub type MPR_R = crate::BitReader<bool>;
///Field `MPR` writer - Magic packet received
pub type MPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `WFR` reader - Wakeup frame received
pub type WFR_R = crate::BitReader<bool>;
///Field `WFR` writer - Wakeup frame received
pub type WFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `GU` reader - Global unicast
pub type GU_R = crate::BitReader<bool>;
///Field `GU` writer - Global unicast
pub type GU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `WFFRPR` reader - Wakeup frame filter register pointer reset
pub type WFFRPR_R = crate::BitReader<bool>;
///Field `WFFRPR` writer - Wakeup frame filter register pointer reset
pub type WFFRPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic Packet enable
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power down
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    ///Bit 1 - Magic Packet enable
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<1> {
        MPE_W::new(self)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<2> {
        WFE_W::new(self)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<5> {
        MPR_W::new(self)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<6> {
        WFR_W::new(self)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<9> {
        GU_W::new(self)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    #[must_use]
    pub fn wffrpr(&mut self) -> WFFRPR_W<31> {
        WFFRPR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC PMT control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macpmtcsr](index.html) module
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macpmtcsr::R](R) reader structure
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACPMTCSR to value 0
impl crate::Resettable for MACPMTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
