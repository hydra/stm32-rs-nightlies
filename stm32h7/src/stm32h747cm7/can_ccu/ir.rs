///Register `IR` reader
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IR` writer
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CWE` reader - Calibration Watchdog Event
pub type CWE_R = crate::BitReader<bool>;
///Field `CWE` writer - Calibration Watchdog Event
pub type CWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
///Field `CSC` reader - Calibration State Changed
pub type CSC_R = crate::BitReader<bool>;
///Field `CSC` writer - Calibration State Changed
pub type CSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Calibration Watchdog Event
    #[inline(always)]
    pub fn cwe(&self) -> CWE_R {
        CWE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Calibration State Changed
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Calibration Watchdog Event
    #[inline(always)]
    #[must_use]
    pub fn cwe(&mut self) -> CWE_W<0> {
        CWE_W::new(self)
    }
    ///Bit 1 - Calibration State Changed
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<1> {
        CSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock Calibration Unit Interrupt Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ir](index.html) module
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ir::R](R) reader structure
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ir::W](W) writer structure
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IR to value 0
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
