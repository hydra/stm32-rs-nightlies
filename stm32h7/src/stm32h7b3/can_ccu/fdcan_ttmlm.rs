///Register `FDCAN_TTMLM` reader
pub struct R(crate::R<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTMLM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTMLM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTMLM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TTMLM` writer
pub struct W(crate::W<FDCAN_TTMLM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTMLM_SPEC>;
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
impl From<crate::W<FDCAN_TTMLM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTMLM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCM` reader - Cycle Count Max
pub type CCM_R = crate::FieldReader<u8, u8>;
///Field `CCM` writer - Cycle Count Max
pub type CCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 6, O>;
///Field `CSS` reader - Cycle Start Synchronization
pub type CSS_R = crate::FieldReader<u8, u8>;
///Field `CSS` writer - Cycle Start Synchronization
pub type CSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 2, O>;
///Field `TXEW` reader - Tx Enable Window
pub type TXEW_R = crate::FieldReader<u8, u8>;
///Field `TXEW` writer - Tx Enable Window
pub type TXEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u8, u8, 4, O>;
///Field `ENTT` reader - Expected Number of Tx Triggers
pub type ENTT_R = crate::FieldReader<u16, u16>;
///Field `ENTT` writer - Expected Number of Tx Triggers
pub type ENTT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTMLM_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:5 - Cycle Count Max
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Cycle Start Synchronization
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Tx Enable Window
    #[inline(always)]
    pub fn txew(&self) -> TXEW_R {
        TXEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:27 - Expected Number of Tx Triggers
    #[inline(always)]
    pub fn entt(&self) -> ENTT_R {
        ENTT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:5 - Cycle Count Max
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<0> {
        CCM_W::new(self)
    }
    ///Bits 6:7 - Cycle Start Synchronization
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<6> {
        CSS_W::new(self)
    }
    ///Bits 8:11 - Tx Enable Window
    #[inline(always)]
    #[must_use]
    pub fn txew(&mut self) -> TXEW_W<8> {
        TXEW_W::new(self)
    }
    ///Bits 16:27 - Expected Number of Tx Triggers
    #[inline(always)]
    #[must_use]
    pub fn entt(&mut self) -> ENTT_W<16> {
        ENTT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT Matrix Limits Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_ttmlm](index.html) module
pub struct FDCAN_TTMLM_SPEC;
impl crate::RegisterSpec for FDCAN_TTMLM_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_ttmlm::R](R) reader structure
impl crate::Readable for FDCAN_TTMLM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_ttmlm::W](W) writer structure
impl crate::Writable for FDCAN_TTMLM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TTMLM to value 0
impl crate::Resettable for FDCAN_TTMLM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
