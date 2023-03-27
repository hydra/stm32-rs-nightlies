///Register `DMACRXIWTR` reader
pub struct R(crate::R<DMACRXIWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRXIWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRXIWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRXIWTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACRXIWTR` writer
pub struct W(crate::W<DMACRXIWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRXIWTR_SPEC>;
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
impl From<crate::W<DMACRXIWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRXIWTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RWT` reader - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\[30\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\[30\]
///of any received packet.
pub type RWT_R = crate::FieldReader<u8, u8>;
///Field `RWT` writer - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\[30\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\[30\]
///of any received packet.
pub type RWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRXIWTR_SPEC, u8, u8, 8, O>;
///Field `RWTU` reader - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\[7:0\]
///field. For example, when RWT\[7:0\]�=�2 and RWTU\[1:0\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles.
pub type RWTU_R = crate::FieldReader<u8, u8>;
///Field `RWTU` writer - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\[7:0\]
///field. For example, when RWT\[7:0\]�=�2 and RWTU\[1:0\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles.
pub type RWTU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACRXIWTR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\[30\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\[30\]
    ///of any received packet.
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:17 - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\[7:0\]
    ///field. For example, when RWT\[7:0\]�=�2 and RWTU\[1:0\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles.
    #[inline(always)]
    pub fn rwtu(&self) -> RWTU_R {
        RWTU_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count This field indicates the number of system clock cycles, multiplied by factor indicated in RWTU field, for which the watchdog timer is set. The watchdog timer is triggered with the programmed value after the Rx DMA completes the transfer of a packet for which the RI bit is not set in the ETH_DMACSR, because of the setting of Interrupt Enable bit in the corresponding descriptor RDES3\[30\]. When the watchdog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when the RI bit is set high because of automatic setting of RI as per the Interrupt Enable bit RDES3\[30\]
    ///of any received packet.
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<0> {
        RWT_W::new(self)
    }
    ///Bits 16:17 - Receive Interrupt Watchdog Timer Count Units This field indicates the number of system clock cycles corresponding to one unit in RWT\[7:0\]
    ///field. For example, when RWT\[7:0\]�=�2 and RWTU\[1:0\]�=�1, the watchdog timer is set for 2�*�512�=�1024 system clock cycles.
    #[inline(always)]
    #[must_use]
    pub fn rwtu(&mut self) -> RWTU_W<16> {
        RWTU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Rx interrupt watchdog timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacrxiwtr](index.html) module
pub struct DMACRXIWTR_SPEC;
impl crate::RegisterSpec for DMACRXIWTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacrxiwtr::R](R) reader structure
impl crate::Readable for DMACRXIWTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacrxiwtr::W](W) writer structure
impl crate::Writable for DMACRXIWTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACRXIWTR to value 0
impl crate::Resettable for DMACRXIWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
