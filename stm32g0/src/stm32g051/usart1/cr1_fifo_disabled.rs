///Register `CR1_FIFO_DISABLED` reader
pub struct R(crate::R<CR1_FIFO_DISABLED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_FIFO_DISABLED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_FIFO_DISABLED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_FIFO_DISABLED_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1_FIFO_DISABLED` writer
pub struct W(crate::W<CR1_FIFO_DISABLED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_FIFO_DISABLED_SPEC>;
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
impl From<crate::W<CR1_FIFO_DISABLED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_FIFO_DISABLED_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UE` reader - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_R = crate::BitReader<bool>;
///Field `UE` writer - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `UESM` reader - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub type UESM_R = crate::BitReader<bool>;
///Field `UESM` writer - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_R = crate::BitReader<bool>;
///Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0â followed by '1â) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1â. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_R = crate::BitReader<bool>;
///Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0â followed by '1â) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1â. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_R = crate::BitReader<bool>;
///Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `RXNEIE` reader - Receive data register not empty This bit is set and cleared by software.
pub type RXNEIE_R = crate::BitReader<bool>;
///Field `RXNEIE` writer - Receive data register not empty This bit is set and cleared by software.
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `TXEIE` reader - Transmit data register empty This bit is set and cleared by software.
pub type TXEIE_R = crate::BitReader<bool>;
///Field `TXEIE` writer - Transmit data register empty This bit is set and cleared by software.
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_R = crate::BitReader<bool>;
///Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type PS_R = crate::BitReader<bool>;
///Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if MÂ =Â 1; 8th bit if MÂ =Â 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type PCE_R = crate::BitReader<bool>;
///Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if MÂ =Â 1; 8th bit if MÂ =Â 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `WAKE` reader - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type WAKE_R = crate::BitReader<bool>;
///Field `WAKE` writer - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UEÂ =Â 0).
pub type M0_R = crate::BitReader<bool>;
///Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UEÂ =Â 0).
pub type M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `MME` reader - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_R = crate::BitReader<bool>;
///Field `MME` writer - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_R = crate::BitReader<bool>;
///Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `OVER8` reader - Oversampling mode This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_R = crate::BitReader<bool>;
///Field `OVER8` writer - Oversampling mode This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEDT_R = crate::FieldReader<u8, u8>;
///Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEDT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, u8, u8, 5, O>;
///Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEAT_R = crate::FieldReader<u8, u8>;
///Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEAT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, u8, u8, 5, O>;
///Field `RTOIE` reader - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .
pub type RTOIE_R = crate::BitReader<bool>;
///Field `RTOIE` writer - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `EOBIE` reader - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type EOBIE_R = crate::BitReader<bool>;
///Field `EOBIE` writer - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type EOBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
///= '00â: 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
///= '01â: 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
///= '10â: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported.
pub type M1_R = crate::BitReader<bool>;
///Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
///= '00â: 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
///= '01â: 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
///= '10â: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported.
pub type M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
///Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_R = crate::BitReader<bool>;
///Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_FIFO_DISABLED_SPEC, bool, O>;
impl R {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0â followed by '1â) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1â. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive data register not empty This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty This bit is set and cleared by software.
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if MÂ =Â 1; 8th bit if MÂ =Â 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    ///= '00â: 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
    ///= '01â: 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
    ///= '10â: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported.
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. In Smartcard mode, (SCEN = 1), the SCLK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ('0â followed by '1â) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to '1â. To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    ///Bit 5 - Receive data register not empty This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    ///Bit 7 - Transmit data register empty This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if MÂ =Â 1; 8th bit if MÂ =Â 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    ///Bit 11 - Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UEÂ =Â 0).
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<12> {
        M0_W::new(self)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<15> {
        OVER8_W::new(self)
    }
    ///Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    ///Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. .
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<26> {
        RTOIE_W::new(self)
    }
    ///Bit 27 - End of Block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<27> {
        EOBIE_W::new(self)
    }
    ///Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    ///= '00â: 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
    ///= '01â: 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
    ///= '10â: 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and Auto baud rate (0x7F and 0x55 frames detection) are not supported.
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<28> {
        M1_W::new(self)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<29> {
        FIFOEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1_fifo_disabled](index.html) module
pub struct CR1_FIFO_DISABLED_SPEC;
impl crate::RegisterSpec for CR1_FIFO_DISABLED_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1_fifo_disabled::R](R) reader structure
impl crate::Readable for CR1_FIFO_DISABLED_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1_fifo_disabled::W](W) writer structure
impl crate::Writable for CR1_FIFO_DISABLED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1_FIFO_DISABLED to value 0
impl crate::Resettable for CR1_FIFO_DISABLED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
