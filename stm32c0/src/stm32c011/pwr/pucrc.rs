///Register `PUCRC` reader
pub struct R(crate::R<PUCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUCRC` writer
pub struct W(crate::W<PUCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRC_SPEC>;
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
impl From<crate::W<PUCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PU14` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
///I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU14_R = crate::BitReader<bool>;
///Field `PU14` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
///I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
///Field `PU15` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
///I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU15_R = crate::BitReader<bool>;
///Field `PU15` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
///I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
impl R {
    ///Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    ///I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    ///I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    ///I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    #[must_use]
    pub fn pu14(&mut self) -> PU14_W<14> {
        PU14_W::new(self)
    }
    ///Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    ///I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> PU15_W<15> {
        PU15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR Port C pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucrc](index.html) module
pub struct PUCRC_SPEC;
impl crate::RegisterSpec for PUCRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [pucrc::R](R) reader structure
impl crate::Readable for PUCRC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pucrc::W](W) writer structure
impl crate::Writable for PUCRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PUCRC to value 0
impl crate::Resettable for PUCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
