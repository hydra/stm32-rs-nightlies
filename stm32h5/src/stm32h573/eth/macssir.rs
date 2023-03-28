///Register `MACSSIR` reader
pub struct R(crate::R<MACSSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSSIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACSSIR` writer
pub struct W(crate::W<MACSSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSSIR_SPEC>;
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
impl From<crate::W<MACSSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSSIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SSINC` reader - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465.
pub type SSINC_R = crate::FieldReader<u8, u8>;
///Field `SSINC` writer - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465.
pub type SSINC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACSSIR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 16:23 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465.
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 16:23 - Subsecond Increment Value The value programmed in this field is accumulated every clock cycle (of clk_ptp_i) with the contents of the subsecond register. For example, when the PTP clock is 50 MHz (period is 20 ns), you should program 20 (0x14) when the System Time Nanoseconds register has an accuracy of 1 ns \[TSCTRLSSR bit is set in Timestamp control Register (ETH_MACTSCR)\]. When TSCTRLSSR is cleared, the Nanoseconds register has a resolution of ~0.465 ns. In this case, you should program a value of 43 (0x2B) which is derived by 20 ns/0.465.
    #[inline(always)]
    #[must_use]
    pub fn ssinc(&mut self) -> SSINC_W<16> {
        SSINC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Subsecond increment register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macssir](index.html) module
pub struct MACSSIR_SPEC;
impl crate::RegisterSpec for MACSSIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macssir::R](R) reader structure
impl crate::Readable for MACSSIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macssir::W](W) writer structure
impl crate::Writable for MACSSIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACSSIR to value 0
impl crate::Resettable for MACSSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
