///Register `FPUIMR` reader
pub struct R(crate::R<FPUIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPUIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPUIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPUIMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPUIMR` writer
pub struct W(crate::W<FPUIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPUIMR_SPEC>;
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
impl From<crate::W<FPUIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPUIMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPU_IE` reader - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\[5\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\[4\]: input abnormal interrupt enable FPU_IE\[3\]: overflow interrupt enable FPU_IE\[2\]: underflow interrupt enable FPU_IE\[1\]: divide-by-zero interrupt enable FPU_IE\[0\]: invalid operation interrupt enable
pub type FPU_IE_R = crate::FieldReader<u8, u8>;
///Field `FPU_IE` writer - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\[5\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\[4\]: input abnormal interrupt enable FPU_IE\[3\]: overflow interrupt enable FPU_IE\[2\]: underflow interrupt enable FPU_IE\[1\]: divide-by-zero interrupt enable FPU_IE\[0\]: invalid operation interrupt enable
pub type FPU_IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPUIMR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\[5\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\[4\]: input abnormal interrupt enable FPU_IE\[3\]: overflow interrupt enable FPU_IE\[2\]: underflow interrupt enable FPU_IE\[1\]: divide-by-zero interrupt enable FPU_IE\[0\]: invalid operation interrupt enable
    #[inline(always)]
    pub fn fpu_ie(&self) -> FPU_IE_R {
        FPU_IE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - FPU interrupt enable Set and cleared by software to enable the Cortex-M33 FPU interrupts FPU_IE\[5\]: inexact interrupt enable (interrupt disabled at reset) FPU_IE\[4\]: input abnormal interrupt enable FPU_IE\[3\]: overflow interrupt enable FPU_IE\[2\]: underflow interrupt enable FPU_IE\[1\]: divide-by-zero interrupt enable FPU_IE\[0\]: invalid operation interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie(&mut self) -> FPU_IE_W<0> {
        FPU_IE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS FPU interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpuimr](index.html) module
pub struct FPUIMR_SPEC;
impl crate::RegisterSpec for FPUIMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpuimr::R](R) reader structure
impl crate::Readable for FPUIMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpuimr::W](W) writer structure
impl crate::Writable for FPUIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FPUIMR to value 0x1f
impl crate::Resettable for FPUIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
