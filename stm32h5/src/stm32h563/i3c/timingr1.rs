///Register `TIMINGR1` reader
pub struct R(crate::R<TIMINGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMINGR1` writer
pub struct W(crate::W<TIMINGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGR1_SPEC>;
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
impl From<crate::W<TIMINGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AVAL` reader - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\[7:0\]
///+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\[7:0\]
///+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
///+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\[7:0\]
///+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 50000 x tI3CCLK
pub type AVAL_R = crate::FieldReader<u8, u8>;
///Field `AVAL` writer - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\[7:0\]
///+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\[7:0\]
///+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
///+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\[7:0\]
///+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
///+ 1) x 50000 x tI3CCLK
pub type AVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR1_SPEC, u8, u8, 8, O>;
///Field `ASNCR` reader - activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
pub type ASNCR_R = crate::FieldReader<u8, u8>;
///Field `ASNCR` writer - activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
pub type ASNCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR1_SPEC, u8, u8, 2, O>;
///Field `FREE` reader - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
///+ 1) x 2 - (0,5 + SDA_HD)\]
///x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
///+ 1) x 2 - (0,5 + SDA_HD)\]
///x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
///+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
///+ 1) x tI3CCLK
pub type FREE_R = crate::FieldReader<u8, u8>;
///Field `FREE` writer - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
///+ 1) x 2 - (0,5 + SDA_HD)\]
///x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
///+ 1) x 2 - (0,5 + SDA_HD)\]
///x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
///+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
///+ 1) x tI3CCLK
pub type FREE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR1_SPEC, u8, u8, 7, O>;
///Field `SDA_HD` reader - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
pub type SDA_HD_R = crate::BitReader<bool>;
///Field `SDA_HD` writer - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
pub type SDA_HD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGR1_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\[7:0\]
    ///+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\[7:0\]
    ///+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
    ///+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\[7:0\]
    ///+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 50000 x tI3CCLK
    #[inline(always)]
    pub fn aval(&self) -> AVAL_R {
        AVAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
    #[inline(always)]
    pub fn asncr(&self) -> ASNCR_R {
        ASNCR_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
    ///+ 1) x 2 - (0,5 + SDA_HD)\]
    ///x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
    ///+ 1) x 2 - (0,5 + SDA_HD)\]
    ///x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
    ///+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
    ///+ 1) x tI3CCLK
    #[inline(always)]
    pub fn free(&self) -> FREE_R {
        FREE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
    #[inline(always)]
    pub fn sda_hd(&self) -> SDA_HD_R {
        SDA_HD_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - number of kernel clock cycles, that is used whatever I3C is acting as controller or target, to set the following MIPI I3C timings, like bus available condition time: When the I3C is acting as target: for bus available condition time: it must wait for (bus available condition) time to be elapsed after a stop and before issuing a start request for an IBI or a controller-role request (i.e. bus free condition is sustained for at least tAVAL). refer to MIPI timing tAVAL = 1 �s. This timing is defined by: tAVAL = (AVAL\[7:0\]
    ///+ 2) x tI3CCLK for bus idle condition time: it must wait for (bus idle condition) time to be elapsed after that both SDA and SCL are continuously high and stable before issuing a hot-join event. Refer to MIPI v1.1 timing tIDLE = 200 �s . This timing is defined by: tIDLE = (AVAL\[7:0\]
    ///+ 2) x 200 x tI3CCLK When the I3C is acting as controller, it can not stall the clock beyond a maximum stall time (i.e. stall the SCL clock low), as follows: on first bit of assigned address during dynamic address assignment: it can not stall the clock beyond the MIPI timing tSTALLDAA = 15 ms. This timing is defined by: tSTALLDAA = (AVAL\[7:0\]
    ///+ 1) x 15000 x tI3CCLK on ACK/NACK phase of I3C/I2C transfer, on parity bit of write data transfer, on transition bit of I3C read transfer: it can not stall the clock beyond the MIPI timing tSTALL = 100 �s. This timing is defined by: tSTALL = (AVAL\[7:0\]
    ///+ 1) x 100 x tI3CCLK Whatever the I3C is acting as controller or as (controller-capable) target, during a controller-role hand-off procedure: The new controller must wait for a time (refer to MIPI timing tNEWCRLock) before pulling SDA low (i.e. issuing a start). And the active controller must wait for the same time while monitoring new controller and before testing the new controller by pulling SDA low. This time to wait is dependent on the defined I3C_TIMINGR1.ANSCR\[1:0\], as follows: If ASNCR\[1:0\]=00: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x tI3CCLK If ASNCR\[1:0\]=01: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 100 x tI3CCLK If ASNCR\[1:0\]=10: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 2000 x tI3CCLK If ASNCR\[1:0\]=11: tNEWCRLock = (AVAL\[7:0\]
    ///+ 1) x 50000 x tI3CCLK
    #[inline(always)]
    #[must_use]
    pub fn aval(&mut self) -> AVAL_W<0> {
        AVAL_W::new(self)
    }
    ///Bits 8:9 - activity state of the new controller (when I3C is acting as - active- controller) This field indicates the time to wait before being accessed as new target, refer to the other field AVAL\[7:0\]. This field can be modified only when the I3C is acting as controller.
    #[inline(always)]
    #[must_use]
    pub fn asncr(&mut self) -> ASNCR_W<8> {
        ASNCR_W::new(self)
    }
    ///Bits 16:22 - number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C is acting as controller) When the I3C is acting as controller: for I3C start timing: it must wait for (bus free condition) time to be elapsed after a stop and before a start, refer to MIPI timings (I3C) tCAS and (I2C) tBUF. These timings are defined by: tBUF= tCAS = \[ (FREE\[6:0\]
    ///+ 1) x 2 - (0,5 + SDA_HD)\]
    ///x tI3CCLK Note: for pure I3C bus: tCASmin= 38,4 ns. Note: for pure I3C bus: tCASmax=1�s, 100�s, 2ms, 50ms for respectively ENTAS0,1,2, and 3. Note: for mixed bus with I2C fm+ device: tBUFmin = 0,5 �s. Note: for mixed bus with I2C fm device: tBUFmin = 1,3 �s. for I3C repeated start timing: it must wait for time to be elapsed after a repeated start (i.e. SDA is de-asserted) and before driving SCL low, refer to. MIPI timing tCASr. This timing is defined by: tCASr = \[ (FREE\[6:0\]
    ///+ 1) x 2 - (0,5 + SDA_HD)\]
    ///x tI3CCLK for I3C stop timing: it must wait for time to be elapsed after that the SCL clock is driven high and before the stop condition (i.e. SDA is asserted). This timing is defined by: tCBP = (FREE\[6:0\]
    ///+ 1) x tI3CCLK for I3C repeated start timing (T-bit when controller ends read with repeated start followed by stop): it must wait for time to be elapsed after that the SCL clock is driven high and before the repeated start condition (i.e. SDA is de-asserted). This timing is defined by: tCBSr = (FREE\[6:0\]
    ///+ 1) x tI3CCLK
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FREE_W<16> {
        FREE_W::new(self)
    }
    ///Bit 28 - SDA hold time (when the I3C is acting as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tHD_PP):
    #[inline(always)]
    #[must_use]
    pub fn sda_hd(&mut self) -> SDA_HD_W<28> {
        SDA_HD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C timing register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timingr1](index.html) module
pub struct TIMINGR1_SPEC;
impl crate::RegisterSpec for TIMINGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [timingr1::R](R) reader structure
impl crate::Readable for TIMINGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timingr1::W](W) writer structure
impl crate::Writable for TIMINGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMINGR1 to value 0
impl crate::Resettable for TIMINGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
