///Register `EWCR` reader
pub struct R(crate::R<EWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EWCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EWCR` writer
pub struct W(crate::W<EWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EWCR_SPEC>;
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
impl From<crate::W<EWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EWCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWIT` reader - Watchdog counter window value
pub type EWIT_R = crate::FieldReader<u16, u16>;
///Field `EWIT` writer - Watchdog counter window value
pub type EWIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EWCR_SPEC, u16, u16, 12, O>;
///Field `EWIC` reader - Watchdog early interrupt acknowledge
pub type EWIC_R = crate::BitReader<bool>;
///Field `EWIC` writer - Watchdog early interrupt acknowledge
pub type EWIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EWCR_SPEC, bool, O>;
///Field `EWIE` reader - Watchdog early interrupt enable
pub type EWIE_R = crate::BitReader<bool>;
///Field `EWIE` writer - Watchdog early interrupt enable
pub type EWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EWCR_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 14 - Watchdog early interrupt acknowledge
    #[inline(always)]
    pub fn ewic(&self) -> EWIC_R {
        EWIC_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Watchdog early interrupt enable
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Watchdog counter window value
    #[inline(always)]
    #[must_use]
    pub fn ewit(&mut self) -> EWIT_W<0> {
        EWIT_W::new(self)
    }
    ///Bit 14 - Watchdog early interrupt acknowledge
    #[inline(always)]
    #[must_use]
    pub fn ewic(&mut self) -> EWIC_W<14> {
        EWIC_W::new(self)
    }
    ///Bit 15 - Watchdog early interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<15> {
        EWIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///IWDG early wakeup interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ewcr](index.html) module
pub struct EWCR_SPEC;
impl crate::RegisterSpec for EWCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ewcr::R](R) reader structure
impl crate::Readable for EWCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ewcr::W](W) writer structure
impl crate::Writable for EWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EWCR to value 0
impl crate::Resettable for EWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
