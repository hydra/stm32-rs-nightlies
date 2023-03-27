///Register `CCCR` reader
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCCR` writer
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INIT` reader - Initialization
pub type INIT_R = crate::BitReader<bool>;
///Field `INIT` writer - Initialization
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `CCE` reader - Configuration change enable
pub type CCE_R = crate::BitReader<bool>;
///Field `CCE` writer - Configuration change enable
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `ASM` reader - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
pub type ASM_R = crate::BitReader<bool>;
///Field `ASM` writer - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `CSA` reader - Clock stop acknowledge
pub type CSA_R = crate::BitReader<bool>;
///Field `CSR` reader - Clock stop request
pub type CSR_R = crate::BitReader<bool>;
///Field `CSR` writer - Clock stop request
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `MON` reader - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
pub type MON_R = crate::BitReader<bool>;
///Field `MON` writer - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `DAR` reader - Disable automatic retransmission
pub type DAR_R = crate::BitReader<bool>;
///Field `DAR` writer - Disable automatic retransmission
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `TEST` reader - Test mode enable
pub type TEST_R = crate::BitReader<bool>;
///Field `TEST` writer - Test mode enable
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `FDOE` reader - FD operation enable
pub type FDOE_R = crate::BitReader<bool>;
///Field `FDOE` writer - FD operation enable
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `BRSE` reader - FDCAN bit rate switching
pub type BRSE_R = crate::BitReader<bool>;
///Field `BRSE` writer - FDCAN bit rate switching
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `PXHD` reader - Protocol exception handling disable
pub type PXHD_R = crate::BitReader<bool>;
///Field `PXHD` writer - Protocol exception handling disable
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `EFBI` reader - Edge filtering during bus integration
pub type EFBI_R = crate::BitReader<bool>;
///Field `EFBI` writer - Edge filtering during bus integration
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `TXP` reader - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
pub type TXP_R = crate::BitReader<bool>;
///Field `TXP` writer - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
///Field `NISO` reader - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
pub type NISO_R = crate::BitReader<bool>;
///Field `NISO` writer - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Initialization
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clock stop acknowledge
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Initialization
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    ///Bit 1 - Configuration change enable
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    ///Bit 2 - ASM restricted operation mode The restricted operation mode is intended for applications that adapt themselves to different CAN bit rates. The application tests different bit rates and leaves the Restricted Operation Mode after it has received a valid frame. In the optional Restricted Operation Mode the node is able to transmit and receive data and remote frames and it gives acknowledge to valid frames, but it does not send active error frames or overload frames. In case of an error condition or overload condition, it does not send dominant bits, instead it waits for the occurrence of bus idle condition to resynchronize itself to the CAN communication. The error counters are not incremented. Bit ASM can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the software at any time.
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    ///Bit 4 - Clock stop request
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    ///Bit 5 - Bus monitoring mode Bit MON can only be set by software when both CCE and INIT are set to 1. The bit can be reset by the Host at any time.
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    ///Bit 6 - Disable automatic retransmission
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    ///Bit 7 - Test mode enable
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    ///Bit 8 - FD operation enable
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    ///Bit 9 - FDCAN bit rate switching
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    ///Bit 12 - Protocol exception handling disable
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    ///Bit 13 - Edge filtering during bus integration
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    ///Bit 14 - If this bit is set, the FDCAN pauses for two CAN bit times before starting the next transmission after successfully transmitting a frame.
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    ///Bit 15 - Non ISO operation If this bit is set, the FDCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0.
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN CC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cccr](index.html) module
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cccr::R](R) reader structure
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cccr::W](W) writer structure
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCCR to value 0x01
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
