///Register `I3C_DEVR4` reader
pub struct R(crate::R<I3C_DEVR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I3C_DEVR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I3C_DEVR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I3C_DEVR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `I3C_DEVR4` writer
pub struct W(crate::W<I3C_DEVR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I3C_DEVR4_SPEC>;
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
impl From<crate::W<I3C_DEVR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I3C_DEVR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DA` reader - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
pub type DA_R = crate::FieldReader<u8, u8>;
///Field `DA` writer - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
pub type DA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I3C_DEVR4_SPEC, u8, u8, 7, O>;
///Field `IBIACK` reader - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
///into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.
pub type IBIACK_R = crate::BitReader<bool>;
///Field `IBIACK` writer - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
///into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.
pub type IBIACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR4_SPEC, bool, O>;
///Field `CRACK` reader - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
///into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.
pub type CRACK_R = crate::BitReader<bool>;
///Field `CRACK` writer - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
///into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.
pub type CRACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR4_SPEC, bool, O>;
///Field `IBIDEN` reader - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\[2\]
///bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
pub type IBIDEN_R = crate::BitReader<bool>;
///Field `IBIDEN` writer - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\[2\]
///bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
pub type IBIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR4_SPEC, bool, O>;
///Field `SUSP` reader - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\[7:5\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\[7:5\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, I3C_DEVR4_SPEC, bool, O>;
///Field `DIS` reader - DA\[6:0\]
///write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\[6:0\]
///and IBIDEN values. Then, to be able to next modify DA\[6:0\]
///or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\[6:0\]
///or IBIDEN.
pub type DIS_R = crate::BitReader<bool>;
impl R {
    ///Bits 1:7 - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 16 - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
    ///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
    ///into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.
    #[inline(always)]
    pub fn ibiack(&self) -> IBIACK_R {
        IBIACK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
    ///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
    ///into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.
    #[inline(always)]
    pub fn crack(&self) -> CRACK_R {
        CRACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\[2\]
    ///bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    #[inline(always)]
    pub fn ibiden(&self) -> IBIDEN_R {
        IBIDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\[7:5\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 31 - DA\[6:0\]
    ///write disabled (when the I3C is acting as controller) When the I3C is acting as controller, once that software set IBIACK=1 or CRACK=1, this read bit is set by hardware (i.e. DIS=1) to lock the configured DA\[6:0\]
    ///and IBIDEN values. Then, to be able to next modify DA\[6:0\]
    ///or IBIDEN, the software must wait for this field DIS to be de-asserted by hardware (i.e. polling on DIS=0) before modifying these two assigned values to the target x. Indeed, the target may be requesting an IBI or a controller-role meanwhile the controller intends to modify DA\[6:0\]
    ///or IBIDEN.
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 1:7 - assigned I3C dynamic address to target x (when the I3C is acting as controller) When the I3C is acting as controller, this field should be written by software to store the 7-bit dynamic address that the controller sends via a broadcast ENTDAA or a direct SETNEWDA CCC which has been acknowledged by the target x. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    ///Bit 16 - IBI request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a IBI request from target x: - After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another IBI request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
    ///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
    ///into the kernel clock domain. - After the ACK, the controller logs the IBI payload data, if any, depending on I3C_DEVRx.IBIDEN. - The software is notified by the IBI flag (i.e. I3C_EVR.IBIF=1) and/or the corresponding interrupt if enabled; - Independently from IBIACK configuration for this or other devices, further IBI request(s) are NACKed until IBI request flag (i.e. I3C_EVR.IBIF) and controller-role request flag (i.e. I3C_EVR.CRF) are both cleared.
    #[inline(always)]
    #[must_use]
    pub fn ibiack(&mut self) -> IBIACK_W<16> {
        IBIACK_W::new(self)
    }
    ///Bit 17 - controller-role request acknowledge (when the I3C is acting as controller) When the I3C is acting as controller, this bit is written by software to define the acknowledge policy to be applied on the I3C bus on the reception of a controller-role request from target x: After the NACK, the message continues as initially programmed (the target is aware of the NACK and can emit another controller-role request later on) - The field DIS is asserted by hardware to protect DA\[6:0\]
    ///from being modified by software meanwhile the hardware can store internally the current DA\[6:0\]
    ///into the kernel clock domain. - After the ACK, the message continues as initially programmed. The software is notified by the controller-role request flag (i.e. I3C_EVR.CRF=1) and/or the corresponding interrupt if enabled; For effectively granting the controller-role to the requesting secondary controller, software should issue a GETACCCR (formerly known as GETACCMST), followed by a STOP. - Independently of CRACK configuration for this or other devices, further controller-role request(s) are NACKed until controller-role request flag (i.e. I3C_EVR.CRF) and IBI flag (i.e. I3C_EVR.IBIF) are both cleared.
    #[inline(always)]
    #[must_use]
    pub fn crack(&mut self) -> CRACK_W<17> {
        CRACK_W::new(self)
    }
    ///Bit 18 - IBI data enable (when the I3C is acting as controller) When the I3C is acting as controller, this bit should be written by software to store the BCR\[2\]
    ///bit as received from the target x during broadcast ENTDAA or direct GETBCR CCC via the received I3C_RDR. Writing to this field has no impact when the read field I3C_DEVRx.DIS=1.
    #[inline(always)]
    #[must_use]
    pub fn ibiden(&mut self) -> IBIDEN_W<18> {
        IBIDEN_W::new(self)
    }
    ///Bit 19 - suspend/stop I3C transfer on received IBI (when the I3C is acting as controller) When the I3C is acting as controller, this bit is used to receive an IBI from target x with pending read notification feature (i.e. with received MDB\[7:5\]=3’b101). If this bit is set, when an IBI is received (i.e. I3C_EVR.IBIF=1), a Stop is emitted on the I3C bus and the C-FIFO is automatically flushed by hardware; to avoid a next private read communication issue if a previous private read message to the target x was stored in the C-FIFO.
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<19> {
        SUSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C device 4 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [i3c_devr4](index.html) module
pub struct I3C_DEVR4_SPEC;
impl crate::RegisterSpec for I3C_DEVR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [i3c_devr4::R](R) reader structure
impl crate::Readable for I3C_DEVR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [i3c_devr4::W](W) writer structure
impl crate::Writable for I3C_DEVR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets I3C_DEVR4 to value 0
impl crate::Resettable for I3C_DEVR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
