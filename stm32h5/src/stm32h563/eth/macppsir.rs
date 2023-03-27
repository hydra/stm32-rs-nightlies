///Register `MACPPSIR` reader
pub struct R(crate::R<MACPPSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPPSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPPSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPPSIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACPPSIR` writer
pub struct W(crate::W<MACPPSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPPSIR_SPEC>;
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
impl From<crate::W<MACPPSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPPSIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PPSINT0` reader - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register.
pub type PPSINT0_R = crate::FieldReader<u32, u32>;
///Field `PPSINT0` writer - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register.
pub type PPSINT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACPPSIR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register.
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - PPS Output Signal Interval These bits store the interval between the rising edges of PPS signal output. The interval is stored in terms of number of units of subsecond increment value. You need to program one value less than the required interval. For example, if the PTP reference clock is 50 MHz (period of 20 ns), and desired interval between the rising edges of PPS signal output is 100 ns (that is, 5 units of subsecond increment value), you should program value 4 (5-1) in this register.
    #[inline(always)]
    #[must_use]
    pub fn ppsint0(&mut self) -> PPSINT0_W<0> {
        PPSINT0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PPS interval register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macppsir](index.html) module
pub struct MACPPSIR_SPEC;
impl crate::RegisterSpec for MACPPSIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macppsir::R](R) reader structure
impl crate::Readable for MACPPSIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macppsir::W](W) writer structure
impl crate::Writable for MACPPSIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACPPSIR to value 0
impl crate::Resettable for MACPPSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
