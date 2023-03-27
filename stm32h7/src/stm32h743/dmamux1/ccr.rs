///Register `CCR%s` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR%s` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAREQ_ID` reader - Input DMA request line selected
pub type DMAREQ_ID_R = crate::FieldReader<u8, DMAREQ_ID_A>;
///Input DMA request line selected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    ///0: No signal selected as request input
    None = 0,
    ///1: Signal `dmamux1_req_gen0` selected as request input
    Dmamux1ReqGen0 = 1,
    ///2: Signal `dmamux1_req_gen1` selected as request input
    Dmamux1ReqGen1 = 2,
    ///3: Signal `dmamux1_req_gen2` selected as request input
    Dmamux1ReqGen2 = 3,
    ///4: Signal `dmamux1_req_gen3` selected as request input
    Dmamux1ReqGen3 = 4,
    ///5: Signal `dmamux1_req_gen4` selected as request input
    Dmamux1ReqGen4 = 5,
    ///6: Signal `dmamux1_req_gen5` selected as request input
    Dmamux1ReqGen5 = 6,
    ///7: Signal `dmamux1_req_gen6` selected as request input
    Dmamux1ReqGen6 = 7,
    ///8: Signal `dmamux1_req_gen7` selected as request input
    Dmamux1ReqGen7 = 8,
    ///9: Signal `adc1_dma` selected as request input
    Adc1Dma = 9,
    ///10: Signal `adc2_dma` selected as request input
    Adc2Dma = 10,
    ///11: Signal `tim1_ch1` selected as request input
    Tim1Ch1 = 11,
    ///12: Signal `tim1_ch2` selected as request input
    Tim1Ch2 = 12,
    ///13: Signal `tim1_ch3` selected as request input
    Tim1Ch3 = 13,
    ///14: Signal `tim1_ch4` selected as request input
    Tim1Ch4 = 14,
    ///15: Signal `tim1_up` selected as request input
    Tim1Up = 15,
    ///16: Signal `tim1_trig` selected as request input
    Tim1Trig = 16,
    ///17: Signal `tim1_com` selected as request input
    Tim1Com = 17,
    ///18: Signal `tim2_ch1` selected as request input
    Tim2Ch1 = 18,
    ///19: Signal `tim2_ch2` selected as request input
    Tim2Ch2 = 19,
    ///20: Signal `tim2_ch3` selected as request input
    Tim2Ch3 = 20,
    ///21: Signal `tim2_ch4` selected as request input
    Tim2Ch4 = 21,
    ///22: Signal `tim2_up` selected as request input
    Tim2Up = 22,
    ///23: Signal `tim3_ch1` selected as request input
    Tim3Ch1 = 23,
    ///24: Signal `tim3_ch2` selected as request input
    Tim3Ch2 = 24,
    ///25: Signal `tim3_ch3` selected as request input
    Tim3Ch3 = 25,
    ///26: Signal `tim3_ch4` selected as request input
    Tim3Ch4 = 26,
    ///27: Signal `tim3_up` selected as request input
    Tim3Up = 27,
    ///28: Signal `tim3_trig` selected as request input
    Tim3Trig = 28,
    ///29: Signal `tim4_ch1` selected as request input
    Tim4Ch1 = 29,
    ///30: Signal `tim4_ch2` selected as request input
    Tim4Ch2 = 30,
    ///31: Signal `tim4_ch3` selected as request input
    Tim4Ch3 = 31,
    ///32: Signal `tim4_up` selected as request input
    Tim4Up = 32,
    ///33: Signal `i2c1_rx_dma` selected as request input
    I2c1RxDma = 33,
    ///34: Signal `i2c1_tx_dma` selected as request input
    I2c1TxDma = 34,
    ///35: Signal `i2c2_rx_dma` selected as request input
    I2c2RxDma = 35,
    ///36: Signal `i2c2_tx_dma` selected as request input
    I2c2TxDma = 36,
    ///37: Signal `spi1_rx_dma` selected as request input
    Spi1RxDma = 37,
    ///38: Signal `spi1_tx_dma` selected as request input
    Spi1TxDma = 38,
    ///39: Signal `spi2_rx_dma` selected as request input
    Spi2RxDma = 39,
    ///40: Signal `spi2_tx_dma` selected as request input
    Spi2TxDma = 40,
    ///41: Signal `usart1_rx_dma` selected as request input
    Usart1RxDma = 41,
    ///42: Signal `usart1_tx_dma` selected as request input
    Usart1TxDma = 42,
    ///43: Signal `usart2_rx_dma` selected as request input
    Usart2RxDma = 43,
    ///44: Signal `usart2_tx_dma` selected as request input
    Usart2TxDma = 44,
    ///45: Signal `usart3_rx_dma` selected as request input
    Usart3RxDma = 45,
    ///46: Signal `usart3_tx_dma` selected as request input
    Usart3TxDma = 46,
    ///47: Signal `tim8_ch1` selected as request input
    Tim8Ch1 = 47,
    ///48: Signal `tim8_ch2` selected as request input
    Tim8Ch2 = 48,
    ///49: Signal `tim8_ch3` selected as request input
    Tim8Ch3 = 49,
    ///50: Signal `tim8_ch4` selected as request input
    Tim8Ch4 = 50,
    ///51: Signal `tim8_up` selected as request input
    Tim8Up = 51,
    ///52: Signal `tim8_trig` selected as request input
    Tim8Trig = 52,
    ///53: Signal `tim8_com` selected as request input
    Tim8Com = 53,
    ///55: Signal `tim5_ch1` selected as request input
    Tim5Ch1 = 55,
    ///56: Signal `tim5_ch2` selected as request input
    Tim5Ch2 = 56,
    ///57: Signal `tim5_ch3` selected as request input
    Tim5Ch3 = 57,
    ///58: Signal `tim5_ch4` selected as request input
    Tim5Ch4 = 58,
    ///59: Signal `tim5_up` selected as request input
    Tim5Up = 59,
    ///60: Signal `tim5_trig` selected as request input
    Tim5Trig = 60,
    ///61: Signal `spi3_rx_dma` selected as request input
    Spi3RxDma = 61,
    ///62: Signal `spi3_tx_dma` selected as request input
    Spi3TxDma = 62,
    ///63: Signal `uart4_rx_dma` selected as request input
    Uart4RxDma = 63,
    ///64: Signal `uart4_tx_dma` selected as request input
    Uart4TxDma = 64,
    ///65: Signal `uart5_rx_dma` selected as request input
    Uart5RxDma = 65,
    ///66: Signal `uart5_tx_dma` selected as request input
    Uart5TxDma = 66,
    ///67: Signal `dac_ch1_dma` selected as request input
    DacCh1Dma = 67,
    ///68: Signal `dac_ch2_dma` selected as request input
    DacCh2Dma = 68,
    ///69: Signal `tim6_up` selected as request input
    Tim6Up = 69,
    ///70: Signal `tim7_up` selected as request input
    Tim7Up = 70,
    ///71: Signal `usart6_rx_dma` selected as request input
    Usart6RxDma = 71,
    ///72: Signal `usart6_tx_dma` selected as request input
    Usart6TxDma = 72,
    ///73: Signal `i2c3_rx_dma` selected as request input
    I2c3RxDma = 73,
    ///74: Signal `i2c3_tx_dma` selected as request input
    I2c3TxDma = 74,
    ///75: Signal `dcmi_dma` selected as request input
    DcmiDma = 75,
    ///76: Signal `cryp_in_dma` selected as request input
    CrypInDma = 76,
    ///77: Signal `cryp_out_dma` selected as request input
    CrypOutDma = 77,
    ///78: Signal `hash_in_dma` selected as request input
    HashInDma = 78,
    ///79: Signal `uart7_rx_dma` selected as request input
    Uart7RxDma = 79,
    ///80: Signal `uart7_tx_dma` selected as request input
    Uart7TxDma = 80,
    ///81: Signal `uart8_rx_dma` selected as request input
    Uart8RxDma = 81,
    ///82: Signal `uart8_tx_dma` selected as request input
    Uart8TxDma = 82,
    ///83: Signal `spi4_rx_dma` selected as request input
    Spi4RxDma = 83,
    ///84: Signal `spi4_tx_dma` selected as request input
    Spi4TxDma = 84,
    ///85: Signal `spi5_rx_dma` selected as request input
    Spi5RxDma = 85,
    ///86: Signal `spi5_tx_dma` selected as request input
    Spi5TxDma = 86,
    ///87: Signal `sai1a_dma` selected as request input
    Sai1aDma = 87,
    ///88: Signal `sai1b_dma` selected as request input
    Sai1bDma = 88,
    ///89: Signal `sai2a_dma` selected as request input
    Sai2aDma = 89,
    ///90: Signal `sai2b_dma` selected as request input
    Sai2bDma = 90,
    ///91: Signal `swpmi_rx_dma` selected as request input
    SwpmiRxDma = 91,
    ///92: Signal `swpmi_tx_dma` selected as request input
    SwpmiTxDma = 92,
    ///93: Signal `spdifrx_dat_dma` selected as request input
    SpdifrxDatDma = 93,
    ///94: Signal `spdifrx_ctrl_dma` selected as request input
    SpdifrxCtrlDma = 94,
    ///95: Signal `hr_req(1)` selected as request input
    HrReq1 = 95,
    ///96: Signal `hr_req(2)` selected as request input
    HrReq2 = 96,
    ///97: Signal `hr_req(3)` selected as request input
    HrReq3 = 97,
    ///98: Signal `hr_req(4)` selected as request input
    HrReq4 = 98,
    ///99: Signal `hr_req(5)` selected as request input
    HrReq5 = 99,
    ///100: Signal `hr_req(6)` selected as request input
    HrReq6 = 100,
    ///101: Signal `dfsdm1_dma0` selected as request input
    Dfsdm1Dma0 = 101,
    ///102: Signal `dfsdm1_dma1` selected as request input
    Dfsdm1Dma1 = 102,
    ///103: Signal `dfsdm1_dma2` selected as request input
    Dfsdm1Dma2 = 103,
    ///104: Signal `dfsdm1_dma3` selected as request input
    Dfsdm1Dma3 = 104,
    ///105: Signal `tim15_ch1` selected as request input
    Tim15Ch1 = 105,
    ///106: Signal `tim15_up` selected as request input
    Tim15Up = 106,
    ///107: Signal `tim15_trig` selected as request input
    Tim15Trig = 107,
    ///108: Signal `tim15_com` selected as request input
    Tim15Com = 108,
    ///109: Signal `tim16_ch1` selected as request input
    Tim16Ch1 = 109,
    ///110: Signal `tim16_up` selected as request input
    Tim16Up = 110,
    ///111: Signal `tim17_ch1` selected as request input
    Tim17Ch1 = 111,
    ///112: Signal `tim17_up` selected as request input
    Tim17Up = 112,
    ///113: Signal `sai3_a_dma` selected as request input
    Sai3ADma = 113,
    ///114: Signal `sai3_b_dma` selected as request input
    Sai3BDma = 114,
    ///115: Signal `adc3_dma` selected as request input
    Adc3Dma = 115,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
impl DMAREQ_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DMAREQ_ID_A> {
        match self.bits {
            0 => Some(DMAREQ_ID_A::None),
            1 => Some(DMAREQ_ID_A::Dmamux1ReqGen0),
            2 => Some(DMAREQ_ID_A::Dmamux1ReqGen1),
            3 => Some(DMAREQ_ID_A::Dmamux1ReqGen2),
            4 => Some(DMAREQ_ID_A::Dmamux1ReqGen3),
            5 => Some(DMAREQ_ID_A::Dmamux1ReqGen4),
            6 => Some(DMAREQ_ID_A::Dmamux1ReqGen5),
            7 => Some(DMAREQ_ID_A::Dmamux1ReqGen6),
            8 => Some(DMAREQ_ID_A::Dmamux1ReqGen7),
            9 => Some(DMAREQ_ID_A::Adc1Dma),
            10 => Some(DMAREQ_ID_A::Adc2Dma),
            11 => Some(DMAREQ_ID_A::Tim1Ch1),
            12 => Some(DMAREQ_ID_A::Tim1Ch2),
            13 => Some(DMAREQ_ID_A::Tim1Ch3),
            14 => Some(DMAREQ_ID_A::Tim1Ch4),
            15 => Some(DMAREQ_ID_A::Tim1Up),
            16 => Some(DMAREQ_ID_A::Tim1Trig),
            17 => Some(DMAREQ_ID_A::Tim1Com),
            18 => Some(DMAREQ_ID_A::Tim2Ch1),
            19 => Some(DMAREQ_ID_A::Tim2Ch2),
            20 => Some(DMAREQ_ID_A::Tim2Ch3),
            21 => Some(DMAREQ_ID_A::Tim2Ch4),
            22 => Some(DMAREQ_ID_A::Tim2Up),
            23 => Some(DMAREQ_ID_A::Tim3Ch1),
            24 => Some(DMAREQ_ID_A::Tim3Ch2),
            25 => Some(DMAREQ_ID_A::Tim3Ch3),
            26 => Some(DMAREQ_ID_A::Tim3Ch4),
            27 => Some(DMAREQ_ID_A::Tim3Up),
            28 => Some(DMAREQ_ID_A::Tim3Trig),
            29 => Some(DMAREQ_ID_A::Tim4Ch1),
            30 => Some(DMAREQ_ID_A::Tim4Ch2),
            31 => Some(DMAREQ_ID_A::Tim4Ch3),
            32 => Some(DMAREQ_ID_A::Tim4Up),
            33 => Some(DMAREQ_ID_A::I2c1RxDma),
            34 => Some(DMAREQ_ID_A::I2c1TxDma),
            35 => Some(DMAREQ_ID_A::I2c2RxDma),
            36 => Some(DMAREQ_ID_A::I2c2TxDma),
            37 => Some(DMAREQ_ID_A::Spi1RxDma),
            38 => Some(DMAREQ_ID_A::Spi1TxDma),
            39 => Some(DMAREQ_ID_A::Spi2RxDma),
            40 => Some(DMAREQ_ID_A::Spi2TxDma),
            41 => Some(DMAREQ_ID_A::Usart1RxDma),
            42 => Some(DMAREQ_ID_A::Usart1TxDma),
            43 => Some(DMAREQ_ID_A::Usart2RxDma),
            44 => Some(DMAREQ_ID_A::Usart2TxDma),
            45 => Some(DMAREQ_ID_A::Usart3RxDma),
            46 => Some(DMAREQ_ID_A::Usart3TxDma),
            47 => Some(DMAREQ_ID_A::Tim8Ch1),
            48 => Some(DMAREQ_ID_A::Tim8Ch2),
            49 => Some(DMAREQ_ID_A::Tim8Ch3),
            50 => Some(DMAREQ_ID_A::Tim8Ch4),
            51 => Some(DMAREQ_ID_A::Tim8Up),
            52 => Some(DMAREQ_ID_A::Tim8Trig),
            53 => Some(DMAREQ_ID_A::Tim8Com),
            55 => Some(DMAREQ_ID_A::Tim5Ch1),
            56 => Some(DMAREQ_ID_A::Tim5Ch2),
            57 => Some(DMAREQ_ID_A::Tim5Ch3),
            58 => Some(DMAREQ_ID_A::Tim5Ch4),
            59 => Some(DMAREQ_ID_A::Tim5Up),
            60 => Some(DMAREQ_ID_A::Tim5Trig),
            61 => Some(DMAREQ_ID_A::Spi3RxDma),
            62 => Some(DMAREQ_ID_A::Spi3TxDma),
            63 => Some(DMAREQ_ID_A::Uart4RxDma),
            64 => Some(DMAREQ_ID_A::Uart4TxDma),
            65 => Some(DMAREQ_ID_A::Uart5RxDma),
            66 => Some(DMAREQ_ID_A::Uart5TxDma),
            67 => Some(DMAREQ_ID_A::DacCh1Dma),
            68 => Some(DMAREQ_ID_A::DacCh2Dma),
            69 => Some(DMAREQ_ID_A::Tim6Up),
            70 => Some(DMAREQ_ID_A::Tim7Up),
            71 => Some(DMAREQ_ID_A::Usart6RxDma),
            72 => Some(DMAREQ_ID_A::Usart6TxDma),
            73 => Some(DMAREQ_ID_A::I2c3RxDma),
            74 => Some(DMAREQ_ID_A::I2c3TxDma),
            75 => Some(DMAREQ_ID_A::DcmiDma),
            76 => Some(DMAREQ_ID_A::CrypInDma),
            77 => Some(DMAREQ_ID_A::CrypOutDma),
            78 => Some(DMAREQ_ID_A::HashInDma),
            79 => Some(DMAREQ_ID_A::Uart7RxDma),
            80 => Some(DMAREQ_ID_A::Uart7TxDma),
            81 => Some(DMAREQ_ID_A::Uart8RxDma),
            82 => Some(DMAREQ_ID_A::Uart8TxDma),
            83 => Some(DMAREQ_ID_A::Spi4RxDma),
            84 => Some(DMAREQ_ID_A::Spi4TxDma),
            85 => Some(DMAREQ_ID_A::Spi5RxDma),
            86 => Some(DMAREQ_ID_A::Spi5TxDma),
            87 => Some(DMAREQ_ID_A::Sai1aDma),
            88 => Some(DMAREQ_ID_A::Sai1bDma),
            89 => Some(DMAREQ_ID_A::Sai2aDma),
            90 => Some(DMAREQ_ID_A::Sai2bDma),
            91 => Some(DMAREQ_ID_A::SwpmiRxDma),
            92 => Some(DMAREQ_ID_A::SwpmiTxDma),
            93 => Some(DMAREQ_ID_A::SpdifrxDatDma),
            94 => Some(DMAREQ_ID_A::SpdifrxCtrlDma),
            95 => Some(DMAREQ_ID_A::HrReq1),
            96 => Some(DMAREQ_ID_A::HrReq2),
            97 => Some(DMAREQ_ID_A::HrReq3),
            98 => Some(DMAREQ_ID_A::HrReq4),
            99 => Some(DMAREQ_ID_A::HrReq5),
            100 => Some(DMAREQ_ID_A::HrReq6),
            101 => Some(DMAREQ_ID_A::Dfsdm1Dma0),
            102 => Some(DMAREQ_ID_A::Dfsdm1Dma1),
            103 => Some(DMAREQ_ID_A::Dfsdm1Dma2),
            104 => Some(DMAREQ_ID_A::Dfsdm1Dma3),
            105 => Some(DMAREQ_ID_A::Tim15Ch1),
            106 => Some(DMAREQ_ID_A::Tim15Up),
            107 => Some(DMAREQ_ID_A::Tim15Trig),
            108 => Some(DMAREQ_ID_A::Tim15Com),
            109 => Some(DMAREQ_ID_A::Tim16Ch1),
            110 => Some(DMAREQ_ID_A::Tim16Up),
            111 => Some(DMAREQ_ID_A::Tim17Ch1),
            112 => Some(DMAREQ_ID_A::Tim17Up),
            113 => Some(DMAREQ_ID_A::Sai3ADma),
            114 => Some(DMAREQ_ID_A::Sai3BDma),
            115 => Some(DMAREQ_ID_A::Adc3Dma),
            _ => None,
        }
    }
    ///Checks if the value of the field is `None`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID_A::None
    }
    ///Checks if the value of the field is `Dmamux1ReqGen0`
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen0
    }
    ///Checks if the value of the field is `Dmamux1ReqGen1`
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen1
    }
    ///Checks if the value of the field is `Dmamux1ReqGen2`
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen2
    }
    ///Checks if the value of the field is `Dmamux1ReqGen3`
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen3
    }
    ///Checks if the value of the field is `Dmamux1ReqGen4`
    #[inline(always)]
    pub fn is_dmamux1_req_gen4(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen4
    }
    ///Checks if the value of the field is `Dmamux1ReqGen5`
    #[inline(always)]
    pub fn is_dmamux1_req_gen5(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen5
    }
    ///Checks if the value of the field is `Dmamux1ReqGen6`
    #[inline(always)]
    pub fn is_dmamux1_req_gen6(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen6
    }
    ///Checks if the value of the field is `Dmamux1ReqGen7`
    #[inline(always)]
    pub fn is_dmamux1_req_gen7(&self) -> bool {
        *self == DMAREQ_ID_A::Dmamux1ReqGen7
    }
    ///Checks if the value of the field is `Adc1Dma`
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Adc1Dma
    }
    ///Checks if the value of the field is `Adc2Dma`
    #[inline(always)]
    pub fn is_adc2_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Adc2Dma
    }
    ///Checks if the value of the field is `Tim1Ch1`
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch1
    }
    ///Checks if the value of the field is `Tim1Ch2`
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch2
    }
    ///Checks if the value of the field is `Tim1Ch3`
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch3
    }
    ///Checks if the value of the field is `Tim1Ch4`
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Ch4
    }
    ///Checks if the value of the field is `Tim1Up`
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Up
    }
    ///Checks if the value of the field is `Tim1Trig`
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Trig
    }
    ///Checks if the value of the field is `Tim1Com`
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID_A::Tim1Com
    }
    ///Checks if the value of the field is `Tim2Ch1`
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch1
    }
    ///Checks if the value of the field is `Tim2Ch2`
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch2
    }
    ///Checks if the value of the field is `Tim2Ch3`
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch3
    }
    ///Checks if the value of the field is `Tim2Ch4`
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Ch4
    }
    ///Checks if the value of the field is `Tim2Up`
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim2Up
    }
    ///Checks if the value of the field is `Tim3Ch1`
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Ch1
    }
    ///Checks if the value of the field is `Tim3Ch2`
    #[inline(always)]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Ch2
    }
    ///Checks if the value of the field is `Tim3Ch3`
    #[inline(always)]
    pub fn is_tim3_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Ch3
    }
    ///Checks if the value of the field is `Tim3Ch4`
    #[inline(always)]
    pub fn is_tim3_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Ch4
    }
    ///Checks if the value of the field is `Tim3Up`
    #[inline(always)]
    pub fn is_tim3_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Up
    }
    ///Checks if the value of the field is `Tim3Trig`
    #[inline(always)]
    pub fn is_tim3_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim3Trig
    }
    ///Checks if the value of the field is `Tim4Ch1`
    #[inline(always)]
    pub fn is_tim4_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim4Ch1
    }
    ///Checks if the value of the field is `Tim4Ch2`
    #[inline(always)]
    pub fn is_tim4_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim4Ch2
    }
    ///Checks if the value of the field is `Tim4Ch3`
    #[inline(always)]
    pub fn is_tim4_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim4Ch3
    }
    ///Checks if the value of the field is `Tim4Up`
    #[inline(always)]
    pub fn is_tim4_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim4Up
    }
    ///Checks if the value of the field is `I2c1RxDma`
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c1RxDma
    }
    ///Checks if the value of the field is `I2c1TxDma`
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c1TxDma
    }
    ///Checks if the value of the field is `I2c2RxDma`
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c2RxDma
    }
    ///Checks if the value of the field is `I2c2TxDma`
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c2TxDma
    }
    ///Checks if the value of the field is `Spi1RxDma`
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi1RxDma
    }
    ///Checks if the value of the field is `Spi1TxDma`
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi1TxDma
    }
    ///Checks if the value of the field is `Spi2RxDma`
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi2RxDma
    }
    ///Checks if the value of the field is `Spi2TxDma`
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi2TxDma
    }
    ///Checks if the value of the field is `Usart1RxDma`
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart1RxDma
    }
    ///Checks if the value of the field is `Usart1TxDma`
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart1TxDma
    }
    ///Checks if the value of the field is `Usart2RxDma`
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart2RxDma
    }
    ///Checks if the value of the field is `Usart2TxDma`
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart2TxDma
    }
    ///Checks if the value of the field is `Usart3RxDma`
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart3RxDma
    }
    ///Checks if the value of the field is `Usart3TxDma`
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart3TxDma
    }
    ///Checks if the value of the field is `Tim8Ch1`
    #[inline(always)]
    pub fn is_tim8_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Ch1
    }
    ///Checks if the value of the field is `Tim8Ch2`
    #[inline(always)]
    pub fn is_tim8_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Ch2
    }
    ///Checks if the value of the field is `Tim8Ch3`
    #[inline(always)]
    pub fn is_tim8_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Ch3
    }
    ///Checks if the value of the field is `Tim8Ch4`
    #[inline(always)]
    pub fn is_tim8_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Ch4
    }
    ///Checks if the value of the field is `Tim8Up`
    #[inline(always)]
    pub fn is_tim8_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Up
    }
    ///Checks if the value of the field is `Tim8Trig`
    #[inline(always)]
    pub fn is_tim8_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Trig
    }
    ///Checks if the value of the field is `Tim8Com`
    #[inline(always)]
    pub fn is_tim8_com(&self) -> bool {
        *self == DMAREQ_ID_A::Tim8Com
    }
    ///Checks if the value of the field is `Tim5Ch1`
    #[inline(always)]
    pub fn is_tim5_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Ch1
    }
    ///Checks if the value of the field is `Tim5Ch2`
    #[inline(always)]
    pub fn is_tim5_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Ch2
    }
    ///Checks if the value of the field is `Tim5Ch3`
    #[inline(always)]
    pub fn is_tim5_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Ch3
    }
    ///Checks if the value of the field is `Tim5Ch4`
    #[inline(always)]
    pub fn is_tim5_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Ch4
    }
    ///Checks if the value of the field is `Tim5Up`
    #[inline(always)]
    pub fn is_tim5_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Up
    }
    ///Checks if the value of the field is `Tim5Trig`
    #[inline(always)]
    pub fn is_tim5_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim5Trig
    }
    ///Checks if the value of the field is `Spi3RxDma`
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi3RxDma
    }
    ///Checks if the value of the field is `Spi3TxDma`
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi3TxDma
    }
    ///Checks if the value of the field is `Uart4RxDma`
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart4RxDma
    }
    ///Checks if the value of the field is `Uart4TxDma`
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart4TxDma
    }
    ///Checks if the value of the field is `Uart5RxDma`
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart5RxDma
    }
    ///Checks if the value of the field is `Uart5TxDma`
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart5TxDma
    }
    ///Checks if the value of the field is `DacCh1Dma`
    #[inline(always)]
    pub fn is_dac_ch1_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DacCh1Dma
    }
    ///Checks if the value of the field is `DacCh2Dma`
    #[inline(always)]
    pub fn is_dac_ch2_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DacCh2Dma
    }
    ///Checks if the value of the field is `Tim6Up`
    #[inline(always)]
    pub fn is_tim6_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim6Up
    }
    ///Checks if the value of the field is `Tim7Up`
    #[inline(always)]
    pub fn is_tim7_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim7Up
    }
    ///Checks if the value of the field is `Usart6RxDma`
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart6RxDma
    }
    ///Checks if the value of the field is `Usart6TxDma`
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Usart6TxDma
    }
    ///Checks if the value of the field is `I2c3RxDma`
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c3RxDma
    }
    ///Checks if the value of the field is `I2c3TxDma`
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2c3TxDma
    }
    ///Checks if the value of the field is `DcmiDma`
    #[inline(always)]
    pub fn is_dcmi_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DcmiDma
    }
    ///Checks if the value of the field is `CrypInDma`
    #[inline(always)]
    pub fn is_cryp_in_dma(&self) -> bool {
        *self == DMAREQ_ID_A::CrypInDma
    }
    ///Checks if the value of the field is `CrypOutDma`
    #[inline(always)]
    pub fn is_cryp_out_dma(&self) -> bool {
        *self == DMAREQ_ID_A::CrypOutDma
    }
    ///Checks if the value of the field is `HashInDma`
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == DMAREQ_ID_A::HashInDma
    }
    ///Checks if the value of the field is `Uart7RxDma`
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart7RxDma
    }
    ///Checks if the value of the field is `Uart7TxDma`
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart7TxDma
    }
    ///Checks if the value of the field is `Uart8RxDma`
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart8RxDma
    }
    ///Checks if the value of the field is `Uart8TxDma`
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Uart8TxDma
    }
    ///Checks if the value of the field is `Spi4RxDma`
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi4RxDma
    }
    ///Checks if the value of the field is `Spi4TxDma`
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi4TxDma
    }
    ///Checks if the value of the field is `Spi5RxDma`
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi5RxDma
    }
    ///Checks if the value of the field is `Spi5TxDma`
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Spi5TxDma
    }
    ///Checks if the value of the field is `Sai1aDma`
    #[inline(always)]
    pub fn is_sai1a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai1aDma
    }
    ///Checks if the value of the field is `Sai1bDma`
    #[inline(always)]
    pub fn is_sai1b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai1bDma
    }
    ///Checks if the value of the field is `Sai2aDma`
    #[inline(always)]
    pub fn is_sai2a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai2aDma
    }
    ///Checks if the value of the field is `Sai2bDma`
    #[inline(always)]
    pub fn is_sai2b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai2bDma
    }
    ///Checks if the value of the field is `SwpmiRxDma`
    #[inline(always)]
    pub fn is_swpmi_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SwpmiRxDma
    }
    ///Checks if the value of the field is `SwpmiTxDma`
    #[inline(always)]
    pub fn is_swpmi_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SwpmiTxDma
    }
    ///Checks if the value of the field is `SpdifrxDatDma`
    #[inline(always)]
    pub fn is_spdifrx_dat_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SpdifrxDatDma
    }
    ///Checks if the value of the field is `SpdifrxCtrlDma`
    #[inline(always)]
    pub fn is_spdifrx_ctrl_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SpdifrxCtrlDma
    }
    ///Checks if the value of the field is `HrReq1`
    #[inline(always)]
    pub fn is_hr_req1(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq1
    }
    ///Checks if the value of the field is `HrReq2`
    #[inline(always)]
    pub fn is_hr_req2(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq2
    }
    ///Checks if the value of the field is `HrReq3`
    #[inline(always)]
    pub fn is_hr_req3(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq3
    }
    ///Checks if the value of the field is `HrReq4`
    #[inline(always)]
    pub fn is_hr_req4(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq4
    }
    ///Checks if the value of the field is `HrReq5`
    #[inline(always)]
    pub fn is_hr_req5(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq5
    }
    ///Checks if the value of the field is `HrReq6`
    #[inline(always)]
    pub fn is_hr_req6(&self) -> bool {
        *self == DMAREQ_ID_A::HrReq6
    }
    ///Checks if the value of the field is `Dfsdm1Dma0`
    #[inline(always)]
    pub fn is_dfsdm1_dma0(&self) -> bool {
        *self == DMAREQ_ID_A::Dfsdm1Dma0
    }
    ///Checks if the value of the field is `Dfsdm1Dma1`
    #[inline(always)]
    pub fn is_dfsdm1_dma1(&self) -> bool {
        *self == DMAREQ_ID_A::Dfsdm1Dma1
    }
    ///Checks if the value of the field is `Dfsdm1Dma2`
    #[inline(always)]
    pub fn is_dfsdm1_dma2(&self) -> bool {
        *self == DMAREQ_ID_A::Dfsdm1Dma2
    }
    ///Checks if the value of the field is `Dfsdm1Dma3`
    #[inline(always)]
    pub fn is_dfsdm1_dma3(&self) -> bool {
        *self == DMAREQ_ID_A::Dfsdm1Dma3
    }
    ///Checks if the value of the field is `Tim15Ch1`
    #[inline(always)]
    pub fn is_tim15_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim15Ch1
    }
    ///Checks if the value of the field is `Tim15Up`
    #[inline(always)]
    pub fn is_tim15_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim15Up
    }
    ///Checks if the value of the field is `Tim15Trig`
    #[inline(always)]
    pub fn is_tim15_trig(&self) -> bool {
        *self == DMAREQ_ID_A::Tim15Trig
    }
    ///Checks if the value of the field is `Tim15Com`
    #[inline(always)]
    pub fn is_tim15_com(&self) -> bool {
        *self == DMAREQ_ID_A::Tim15Com
    }
    ///Checks if the value of the field is `Tim16Ch1`
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim16Ch1
    }
    ///Checks if the value of the field is `Tim16Up`
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim16Up
    }
    ///Checks if the value of the field is `Tim17Ch1`
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::Tim17Ch1
    }
    ///Checks if the value of the field is `Tim17Up`
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID_A::Tim17Up
    }
    ///Checks if the value of the field is `Sai3ADma`
    #[inline(always)]
    pub fn is_sai3_a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai3ADma
    }
    ///Checks if the value of the field is `Sai3BDma`
    #[inline(always)]
    pub fn is_sai3_b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Sai3BDma
    }
    ///Checks if the value of the field is `Adc3Dma`
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID_A::Adc3Dma
    }
}
///Field `DMAREQ_ID` writer - Input DMA request line selected
pub type DMAREQ_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCR_SPEC, u8, DMAREQ_ID_A, 8, O>;
impl<'a, const O: u8> DMAREQ_ID_W<'a, O> {
    ///No signal selected as request input
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::None)
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen0)
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen1)
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen2)
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen3)
    }
    ///Signal `dmamux1_req_gen4` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen4)
    }
    ///Signal `dmamux1_req_gen5` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen5)
    }
    ///Signal `dmamux1_req_gen6` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen6)
    }
    ///Signal `dmamux1_req_gen7` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen7(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dmamux1ReqGen7)
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Adc1Dma)
    }
    ///Signal `adc2_dma` selected as request input
    #[inline(always)]
    pub fn adc2_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Adc2Dma)
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch1)
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch2)
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch3)
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Ch4)
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Up)
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Trig)
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim1Com)
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch1)
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch2)
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch3)
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Ch4)
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim2Up)
    }
    ///Signal `tim3_ch1` selected as request input
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Ch1)
    }
    ///Signal `tim3_ch2` selected as request input
    #[inline(always)]
    pub fn tim3_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Ch2)
    }
    ///Signal `tim3_ch3` selected as request input
    #[inline(always)]
    pub fn tim3_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Ch3)
    }
    ///Signal `tim3_ch4` selected as request input
    #[inline(always)]
    pub fn tim3_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Ch4)
    }
    ///Signal `tim3_up` selected as request input
    #[inline(always)]
    pub fn tim3_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Up)
    }
    ///Signal `tim3_trig` selected as request input
    #[inline(always)]
    pub fn tim3_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim3Trig)
    }
    ///Signal `tim4_ch1` selected as request input
    #[inline(always)]
    pub fn tim4_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim4Ch1)
    }
    ///Signal `tim4_ch2` selected as request input
    #[inline(always)]
    pub fn tim4_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim4Ch2)
    }
    ///Signal `tim4_ch3` selected as request input
    #[inline(always)]
    pub fn tim4_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim4Ch3)
    }
    ///Signal `tim4_up` selected as request input
    #[inline(always)]
    pub fn tim4_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim4Up)
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c1RxDma)
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c1TxDma)
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c2RxDma)
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c2TxDma)
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi1RxDma)
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi1TxDma)
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi2RxDma)
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi2TxDma)
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart1RxDma)
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart1TxDma)
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart2RxDma)
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart2TxDma)
    }
    ///Signal `usart3_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart3RxDma)
    }
    ///Signal `usart3_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart3TxDma)
    }
    ///Signal `tim8_ch1` selected as request input
    #[inline(always)]
    pub fn tim8_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Ch1)
    }
    ///Signal `tim8_ch2` selected as request input
    #[inline(always)]
    pub fn tim8_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Ch2)
    }
    ///Signal `tim8_ch3` selected as request input
    #[inline(always)]
    pub fn tim8_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Ch3)
    }
    ///Signal `tim8_ch4` selected as request input
    #[inline(always)]
    pub fn tim8_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Ch4)
    }
    ///Signal `tim8_up` selected as request input
    #[inline(always)]
    pub fn tim8_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Up)
    }
    ///Signal `tim8_trig` selected as request input
    #[inline(always)]
    pub fn tim8_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Trig)
    }
    ///Signal `tim8_com` selected as request input
    #[inline(always)]
    pub fn tim8_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim8Com)
    }
    ///Signal `tim5_ch1` selected as request input
    #[inline(always)]
    pub fn tim5_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Ch1)
    }
    ///Signal `tim5_ch2` selected as request input
    #[inline(always)]
    pub fn tim5_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Ch2)
    }
    ///Signal `tim5_ch3` selected as request input
    #[inline(always)]
    pub fn tim5_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Ch3)
    }
    ///Signal `tim5_ch4` selected as request input
    #[inline(always)]
    pub fn tim5_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Ch4)
    }
    ///Signal `tim5_up` selected as request input
    #[inline(always)]
    pub fn tim5_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Up)
    }
    ///Signal `tim5_trig` selected as request input
    #[inline(always)]
    pub fn tim5_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim5Trig)
    }
    ///Signal `spi3_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi3RxDma)
    }
    ///Signal `spi3_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi3TxDma)
    }
    ///Signal `uart4_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart4RxDma)
    }
    ///Signal `uart4_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart4TxDma)
    }
    ///Signal `uart5_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart5RxDma)
    }
    ///Signal `uart5_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart5TxDma)
    }
    ///Signal `dac_ch1_dma` selected as request input
    #[inline(always)]
    pub fn dac_ch1_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DacCh1Dma)
    }
    ///Signal `dac_ch2_dma` selected as request input
    #[inline(always)]
    pub fn dac_ch2_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DacCh2Dma)
    }
    ///Signal `tim6_up` selected as request input
    #[inline(always)]
    pub fn tim6_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim6Up)
    }
    ///Signal `tim7_up` selected as request input
    #[inline(always)]
    pub fn tim7_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim7Up)
    }
    ///Signal `usart6_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart6RxDma)
    }
    ///Signal `usart6_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Usart6TxDma)
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c3RxDma)
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2c3TxDma)
    }
    ///Signal `dcmi_dma` selected as request input
    #[inline(always)]
    pub fn dcmi_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DcmiDma)
    }
    ///Signal `cryp_in_dma` selected as request input
    #[inline(always)]
    pub fn cryp_in_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::CrypInDma)
    }
    ///Signal `cryp_out_dma` selected as request input
    #[inline(always)]
    pub fn cryp_out_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::CrypOutDma)
    }
    ///Signal `hash_in_dma` selected as request input
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HashInDma)
    }
    ///Signal `uart7_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart7RxDma)
    }
    ///Signal `uart7_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart7TxDma)
    }
    ///Signal `uart8_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart8RxDma)
    }
    ///Signal `uart8_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Uart8TxDma)
    }
    ///Signal `spi4_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi4RxDma)
    }
    ///Signal `spi4_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi4TxDma)
    }
    ///Signal `spi5_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi5RxDma)
    }
    ///Signal `spi5_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Spi5TxDma)
    }
    ///Signal `sai1a_dma` selected as request input
    #[inline(always)]
    pub fn sai1a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai1aDma)
    }
    ///Signal `sai1b_dma` selected as request input
    #[inline(always)]
    pub fn sai1b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai1bDma)
    }
    ///Signal `sai2a_dma` selected as request input
    #[inline(always)]
    pub fn sai2a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai2aDma)
    }
    ///Signal `sai2b_dma` selected as request input
    #[inline(always)]
    pub fn sai2b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai2bDma)
    }
    ///Signal `swpmi_rx_dma` selected as request input
    #[inline(always)]
    pub fn swpmi_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SwpmiRxDma)
    }
    ///Signal `swpmi_tx_dma` selected as request input
    #[inline(always)]
    pub fn swpmi_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SwpmiTxDma)
    }
    ///Signal `spdifrx_dat_dma` selected as request input
    #[inline(always)]
    pub fn spdifrx_dat_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SpdifrxDatDma)
    }
    ///Signal `spdifrx_ctrl_dma` selected as request input
    #[inline(always)]
    pub fn spdifrx_ctrl_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SpdifrxCtrlDma)
    }
    ///Signal `hr_req(1)` selected as request input
    #[inline(always)]
    pub fn hr_req1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq1)
    }
    ///Signal `hr_req(2)` selected as request input
    #[inline(always)]
    pub fn hr_req2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq2)
    }
    ///Signal `hr_req(3)` selected as request input
    #[inline(always)]
    pub fn hr_req3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq3)
    }
    ///Signal `hr_req(4)` selected as request input
    #[inline(always)]
    pub fn hr_req4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq4)
    }
    ///Signal `hr_req(5)` selected as request input
    #[inline(always)]
    pub fn hr_req5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq5)
    }
    ///Signal `hr_req(6)` selected as request input
    #[inline(always)]
    pub fn hr_req6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HrReq6)
    }
    ///Signal `dfsdm1_dma0` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dfsdm1Dma0)
    }
    ///Signal `dfsdm1_dma1` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dfsdm1Dma1)
    }
    ///Signal `dfsdm1_dma2` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dfsdm1Dma2)
    }
    ///Signal `dfsdm1_dma3` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Dfsdm1Dma3)
    }
    ///Signal `tim15_ch1` selected as request input
    #[inline(always)]
    pub fn tim15_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim15Ch1)
    }
    ///Signal `tim15_up` selected as request input
    #[inline(always)]
    pub fn tim15_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim15Up)
    }
    ///Signal `tim15_trig` selected as request input
    #[inline(always)]
    pub fn tim15_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim15Trig)
    }
    ///Signal `tim15_com` selected as request input
    #[inline(always)]
    pub fn tim15_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim15Com)
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim16Ch1)
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim16Up)
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim17Ch1)
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Tim17Up)
    }
    ///Signal `sai3_a_dma` selected as request input
    #[inline(always)]
    pub fn sai3_a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai3ADma)
    }
    ///Signal `sai3_b_dma` selected as request input
    #[inline(always)]
    pub fn sai3_b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Sai3BDma)
    }
    ///Signal `adc3_dma` selected as request input
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::Adc3Dma)
    }
}
///Field `SOIE` reader - Interrupt enable at synchronization event overrun
pub type SOIE_R = crate::BitReader<SOIE_A>;
///Interrupt enable at synchronization event overrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE_A {
    ///0: Synchronization overrun interrupt disabled
    Disabled = 0,
    ///1: Synchronization overrun interrupt enabled
    Enabled = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::Disabled,
            true => SOIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE_A::Enabled
    }
}
///Field `SOIE` writer - Interrupt enable at synchronization event overrun
pub type SOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, SOIE_A, O>;
impl<'a, const O: u8> SOIE_W<'a, O> {
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::Disabled)
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::Enabled)
    }
}
///Field `EGE` reader - Event generation enable/disable
pub type EGE_R = crate::BitReader<EGE_A>;
///Event generation enable/disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE_A {
    ///0: Event generation disabled
    Disabled = 0,
    ///1: Event generation enabled
    Enabled = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::Disabled,
            true => EGE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE_A::Enabled
    }
}
///Field `EGE` writer - Event generation enable/disable
pub type EGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, EGE_A, O>;
impl<'a, const O: u8> EGE_W<'a, O> {
    ///Event generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::Disabled)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::Enabled)
    }
}
///Field `SE` reader - Synchronous operating mode enable/disable
pub type SE_R = crate::BitReader<SE_A>;
///Synchronous operating mode enable/disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    ///0: Synchronization disabled
    Disabled = 0,
    ///1: Synchronization enabled
    Enabled = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::Disabled,
            true => SE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE_A::Enabled
    }
}
///Field `SE` writer - Synchronous operating mode enable/disable
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, SE_A, O>;
impl<'a, const O: u8> SE_W<'a, O> {
    ///Synchronization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::Disabled)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::Enabled)
    }
}
///Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_R = crate::FieldReader<u8, SPOL_A>;
///Synchronization event type selector Defines the synchronization event on the selected synchronization input:
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL_A {
    ///0: No event, i.e. no synchronization nor detection
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
    BothEdges = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
impl SPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NoEdge,
            1 => SPOL_A::RisingEdge,
            2 => SPOL_A::FallingEdge,
            3 => SPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoEdge`
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL_A::NoEdge
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL_A::BothEdges
    }
}
///Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
pub type SPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, SPOL_A, 2, O>;
impl<'a, const O: u8> SPOL_W<'a, O> {
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BothEdges)
    }
}
///Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_R = crate::FieldReader<u8, u8>;
///Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
pub type NBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 5, O>;
///Field `SYNC_ID` reader - Synchronization input selected
pub type SYNC_ID_R = crate::FieldReader<u8, SYNC_ID_A>;
///Synchronization input selected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    ///0: Signal `dmamux1_evt0` selected as synchronization input
    Dmamux1Evt0 = 0,
    ///1: Signal `dmamux1_evt1` selected as synchronization input
    Dmamux1Evt1 = 1,
    ///2: Signal `dmamux1_evt2` selected as synchronization input
    Dmamux1Evt2 = 2,
    ///3: Signal `lptim1_out` selected as synchronization input
    Lptim1Out = 3,
    ///4: Signal `lptim2_out` selected as synchronization input
    Lptim2Out = 4,
    ///5: Signal `lptim3_out` selected as synchronization input
    Lptim3Out = 5,
    ///6: Signal `extit0` selected as synchronization input
    Extit0 = 6,
    ///7: Signal `tim12_trgo` selected as synchronization input
    Tim12Trgo = 7,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
impl SYNC_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNC_ID_A> {
        match self.bits {
            0 => Some(SYNC_ID_A::Dmamux1Evt0),
            1 => Some(SYNC_ID_A::Dmamux1Evt1),
            2 => Some(SYNC_ID_A::Dmamux1Evt2),
            3 => Some(SYNC_ID_A::Lptim1Out),
            4 => Some(SYNC_ID_A::Lptim2Out),
            5 => Some(SYNC_ID_A::Lptim3Out),
            6 => Some(SYNC_ID_A::Extit0),
            7 => Some(SYNC_ID_A::Tim12Trgo),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Dmamux1Evt0`
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID_A::Dmamux1Evt0
    }
    ///Checks if the value of the field is `Dmamux1Evt1`
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID_A::Dmamux1Evt1
    }
    ///Checks if the value of the field is `Dmamux1Evt2`
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SYNC_ID_A::Dmamux1Evt2
    }
    ///Checks if the value of the field is `Lptim1Out`
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim1Out
    }
    ///Checks if the value of the field is `Lptim2Out`
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim2Out
    }
    ///Checks if the value of the field is `Lptim3Out`
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID_A::Lptim3Out
    }
    ///Checks if the value of the field is `Extit0`
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SYNC_ID_A::Extit0
    }
    ///Checks if the value of the field is `Tim12Trgo`
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SYNC_ID_A::Tim12Trgo
    }
}
///Field `SYNC_ID` writer - Synchronization input selected
pub type SYNC_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, SYNC_ID_A, 5, O>;
impl<'a, const O: u8> SYNC_ID_W<'a, O> {
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux1Evt1)
    }
    ///Signal `dmamux1_evt2` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Dmamux1Evt2)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Lptim3Out)
    }
    ///Signal `extit0` selected as synchronization input
    #[inline(always)]
    pub fn extit0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Extit0)
    }
    ///Signal `tim12_trgo` selected as synchronization input
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut W {
        self.variant(SYNC_ID_A::Tim12Trgo)
    }
}
impl R {
    ///Bits 0:7 - Input DMA request line selected
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:7 - Input DMA request line selected
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<0> {
        DMAREQ_ID_W::new(self)
    }
    ///Bit 8 - Interrupt enable at synchronization event overrun
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<8> {
        SOIE_W::new(self)
    }
    ///Bit 9 - Event generation enable/disable
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<9> {
        EGE_W::new(self)
    }
    ///Bit 16 - Synchronous operating mode enable/disable
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<16> {
        SE_W::new(self)
    }
    ///Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<17> {
        SPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset.
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<19> {
        NBREQ_W::new(self)
    }
    ///Bits 24:28 - Synchronization input selected
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<24> {
        SYNC_ID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMAMux - DMA request line multiplexer channel x control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR%s to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
