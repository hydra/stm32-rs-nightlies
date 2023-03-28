///Register `PDCRF` reader
pub struct R(crate::R<PDCRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRF_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRF` writer
pub struct W(crate::W<PDCRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRF_SPEC>;
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
impl From<crate::W<PDCRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRF_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD2` reader - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\]
///I/O. On STM32C011xx, only PD2 is available.
pub type PD2_R = crate::BitReader<bool>;
///Field `PD2` writer - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\]
///I/O. On STM32C011xx, only PD2 is available.
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRF_SPEC, bool, O>;
impl R {
    ///Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\]
    ///I/O. On STM32C011xx, only PD2 is available.
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Port F pull-down bit i (i = 2 to 0) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PF\[i\]
    ///I/O. On STM32C011xx, only PD2 is available.
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR Port F pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrf](index.html) module
pub struct PDCRF_SPEC;
impl crate::RegisterSpec for PDCRF_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcrf::R](R) reader structure
impl crate::Readable for PDCRF_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcrf::W](W) writer structure
impl crate::Writable for PDCRF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDCRF to value 0
impl crate::Resettable for PDCRF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
