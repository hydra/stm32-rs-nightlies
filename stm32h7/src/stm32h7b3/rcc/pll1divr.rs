///Register `PLL1DIVR` reader
pub struct R(crate::R<PLL1DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1DIVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLL1DIVR` writer
pub struct W(crate::W<PLL1DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1DIVR_SPEC>;
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
impl From<crate::W<PLL1DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1DIVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIVN1` reader - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL1VCOSEL = 0 150 to 420Â MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x DIVN1, when fractional value 0 has been loaded into FRACN1, with: DIVN1 between 8 and 420 The input frequency Fref1_ck must be between 1 and 16Â MHz.
pub type DIVN1_R = crate::FieldReader<u16, u16>;
///Field `DIVN1` writer - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL1VCOSEL = 0 150 to 420Â MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x DIVN1, when fractional value 0 has been loaded into FRACN1, with: DIVN1 between 8 and 420 The input frequency Fref1_ck must be between 1 and 16Â MHz.
pub type DIVN1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u16, u16, 9, O>;
///Field `DIVP1` reader - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...
pub type DIVP1_R = crate::FieldReader<u8, DIVP1_A>;
///PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVP1_A {
    ///0: pll_p_ck = vco_ck
    Div1 = 0,
    ///1: pll_p_ck = vco_ck / 2
    Div2 = 1,
    ///3: pll_p_ck = vco_ck / 4
    Div4 = 3,
    ///5: pll_p_ck = vco_ck / 6
    Div6 = 5,
    ///7: pll_p_ck = vco_ck / 8
    Div8 = 7,
    ///9: pll_p_ck = vco_ck / 10
    Div10 = 9,
    ///11: pll_p_ck = vco_ck / 12
    Div12 = 11,
    ///13: pll_p_ck = vco_ck / 14
    Div14 = 13,
    ///15: pll_p_ck = vco_ck / 16
    Div16 = 15,
    ///17: pll_p_ck = vco_ck / 18
    Div18 = 17,
    ///19: pll_p_ck = vco_ck / 20
    Div20 = 19,
    ///21: pll_p_ck = vco_ck / 22
    Div22 = 21,
    ///23: pll_p_ck = vco_ck / 24
    Div24 = 23,
    ///25: pll_p_ck = vco_ck / 26
    Div26 = 25,
    ///27: pll_p_ck = vco_ck / 28
    Div28 = 27,
    ///29: pll_p_ck = vco_ck / 30
    Div30 = 29,
    ///31: pll_p_ck = vco_ck / 32
    Div32 = 31,
    ///33: pll_p_ck = vco_ck / 34
    Div34 = 33,
    ///35: pll_p_ck = vco_ck / 36
    Div36 = 35,
    ///37: pll_p_ck = vco_ck / 38
    Div38 = 37,
    ///39: pll_p_ck = vco_ck / 40
    Div40 = 39,
    ///41: pll_p_ck = vco_ck / 42
    Div42 = 41,
    ///43: pll_p_ck = vco_ck / 44
    Div44 = 43,
    ///45: pll_p_ck = vco_ck / 46
    Div46 = 45,
    ///47: pll_p_ck = vco_ck / 48
    Div48 = 47,
    ///49: pll_p_ck = vco_ck / 50
    Div50 = 49,
    ///51: pll_p_ck = vco_ck / 52
    Div52 = 51,
    ///53: pll_p_ck = vco_ck / 54
    Div54 = 53,
    ///55: pll_p_ck = vco_ck / 56
    Div56 = 55,
    ///57: pll_p_ck = vco_ck / 58
    Div58 = 57,
    ///59: pll_p_ck = vco_ck / 60
    Div60 = 59,
    ///61: pll_p_ck = vco_ck / 62
    Div62 = 61,
    ///63: pll_p_ck = vco_ck / 64
    Div64 = 63,
    ///65: pll_p_ck = vco_ck / 66
    Div66 = 65,
    ///67: pll_p_ck = vco_ck / 68
    Div68 = 67,
    ///69: pll_p_ck = vco_ck / 70
    Div70 = 69,
    ///71: pll_p_ck = vco_ck / 72
    Div72 = 71,
    ///73: pll_p_ck = vco_ck / 74
    Div74 = 73,
    ///75: pll_p_ck = vco_ck / 76
    Div76 = 75,
    ///77: pll_p_ck = vco_ck / 78
    Div78 = 77,
    ///79: pll_p_ck = vco_ck / 80
    Div80 = 79,
    ///81: pll_p_ck = vco_ck / 82
    Div82 = 81,
    ///83: pll_p_ck = vco_ck / 84
    Div84 = 83,
    ///85: pll_p_ck = vco_ck / 86
    Div86 = 85,
    ///87: pll_p_ck = vco_ck / 88
    Div88 = 87,
    ///89: pll_p_ck = vco_ck / 90
    Div90 = 89,
    ///91: pll_p_ck = vco_ck / 92
    Div92 = 91,
    ///93: pll_p_ck = vco_ck / 94
    Div94 = 93,
    ///95: pll_p_ck = vco_ck / 96
    Div96 = 95,
    ///97: pll_p_ck = vco_ck / 98
    Div98 = 97,
    ///99: pll_p_ck = vco_ck / 100
    Div100 = 99,
    ///101: pll_p_ck = vco_ck / 102
    Div102 = 101,
    ///103: pll_p_ck = vco_ck / 104
    Div104 = 103,
    ///105: pll_p_ck = vco_ck / 106
    Div106 = 105,
    ///107: pll_p_ck = vco_ck / 108
    Div108 = 107,
    ///109: pll_p_ck = vco_ck / 110
    Div110 = 109,
    ///111: pll_p_ck = vco_ck / 112
    Div112 = 111,
    ///113: pll_p_ck = vco_ck / 114
    Div114 = 113,
    ///115: pll_p_ck = vco_ck / 116
    Div116 = 115,
    ///117: pll_p_ck = vco_ck / 118
    Div118 = 117,
    ///119: pll_p_ck = vco_ck / 120
    Div120 = 119,
    ///121: pll_p_ck = vco_ck / 122
    Div122 = 121,
    ///123: pll_p_ck = vco_ck / 124
    Div124 = 123,
    ///125: pll_p_ck = vco_ck / 126
    Div126 = 125,
    ///127: pll_p_ck = vco_ck / 128
    Div128 = 127,
}
impl From<DIVP1_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVP1_A) -> Self {
        variant as _
    }
}
impl DIVP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVP1_A> {
        match self.bits {
            0 => Some(DIVP1_A::Div1),
            1 => Some(DIVP1_A::Div2),
            3 => Some(DIVP1_A::Div4),
            5 => Some(DIVP1_A::Div6),
            7 => Some(DIVP1_A::Div8),
            9 => Some(DIVP1_A::Div10),
            11 => Some(DIVP1_A::Div12),
            13 => Some(DIVP1_A::Div14),
            15 => Some(DIVP1_A::Div16),
            17 => Some(DIVP1_A::Div18),
            19 => Some(DIVP1_A::Div20),
            21 => Some(DIVP1_A::Div22),
            23 => Some(DIVP1_A::Div24),
            25 => Some(DIVP1_A::Div26),
            27 => Some(DIVP1_A::Div28),
            29 => Some(DIVP1_A::Div30),
            31 => Some(DIVP1_A::Div32),
            33 => Some(DIVP1_A::Div34),
            35 => Some(DIVP1_A::Div36),
            37 => Some(DIVP1_A::Div38),
            39 => Some(DIVP1_A::Div40),
            41 => Some(DIVP1_A::Div42),
            43 => Some(DIVP1_A::Div44),
            45 => Some(DIVP1_A::Div46),
            47 => Some(DIVP1_A::Div48),
            49 => Some(DIVP1_A::Div50),
            51 => Some(DIVP1_A::Div52),
            53 => Some(DIVP1_A::Div54),
            55 => Some(DIVP1_A::Div56),
            57 => Some(DIVP1_A::Div58),
            59 => Some(DIVP1_A::Div60),
            61 => Some(DIVP1_A::Div62),
            63 => Some(DIVP1_A::Div64),
            65 => Some(DIVP1_A::Div66),
            67 => Some(DIVP1_A::Div68),
            69 => Some(DIVP1_A::Div70),
            71 => Some(DIVP1_A::Div72),
            73 => Some(DIVP1_A::Div74),
            75 => Some(DIVP1_A::Div76),
            77 => Some(DIVP1_A::Div78),
            79 => Some(DIVP1_A::Div80),
            81 => Some(DIVP1_A::Div82),
            83 => Some(DIVP1_A::Div84),
            85 => Some(DIVP1_A::Div86),
            87 => Some(DIVP1_A::Div88),
            89 => Some(DIVP1_A::Div90),
            91 => Some(DIVP1_A::Div92),
            93 => Some(DIVP1_A::Div94),
            95 => Some(DIVP1_A::Div96),
            97 => Some(DIVP1_A::Div98),
            99 => Some(DIVP1_A::Div100),
            101 => Some(DIVP1_A::Div102),
            103 => Some(DIVP1_A::Div104),
            105 => Some(DIVP1_A::Div106),
            107 => Some(DIVP1_A::Div108),
            109 => Some(DIVP1_A::Div110),
            111 => Some(DIVP1_A::Div112),
            113 => Some(DIVP1_A::Div114),
            115 => Some(DIVP1_A::Div116),
            117 => Some(DIVP1_A::Div118),
            119 => Some(DIVP1_A::Div120),
            121 => Some(DIVP1_A::Div122),
            123 => Some(DIVP1_A::Div124),
            125 => Some(DIVP1_A::Div126),
            127 => Some(DIVP1_A::Div128),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVP1_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVP1_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVP1_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == DIVP1_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVP1_A::Div8
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == DIVP1_A::Div10
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == DIVP1_A::Div12
    }
    ///Checks if the value of the field is `Div14`
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == DIVP1_A::Div14
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVP1_A::Div16
    }
    ///Checks if the value of the field is `Div18`
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == DIVP1_A::Div18
    }
    ///Checks if the value of the field is `Div20`
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == DIVP1_A::Div20
    }
    ///Checks if the value of the field is `Div22`
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == DIVP1_A::Div22
    }
    ///Checks if the value of the field is `Div24`
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == DIVP1_A::Div24
    }
    ///Checks if the value of the field is `Div26`
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == DIVP1_A::Div26
    }
    ///Checks if the value of the field is `Div28`
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == DIVP1_A::Div28
    }
    ///Checks if the value of the field is `Div30`
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == DIVP1_A::Div30
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVP1_A::Div32
    }
    ///Checks if the value of the field is `Div34`
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == DIVP1_A::Div34
    }
    ///Checks if the value of the field is `Div36`
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == DIVP1_A::Div36
    }
    ///Checks if the value of the field is `Div38`
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == DIVP1_A::Div38
    }
    ///Checks if the value of the field is `Div40`
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == DIVP1_A::Div40
    }
    ///Checks if the value of the field is `Div42`
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == DIVP1_A::Div42
    }
    ///Checks if the value of the field is `Div44`
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == DIVP1_A::Div44
    }
    ///Checks if the value of the field is `Div46`
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == DIVP1_A::Div46
    }
    ///Checks if the value of the field is `Div48`
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == DIVP1_A::Div48
    }
    ///Checks if the value of the field is `Div50`
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == DIVP1_A::Div50
    }
    ///Checks if the value of the field is `Div52`
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == DIVP1_A::Div52
    }
    ///Checks if the value of the field is `Div54`
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == DIVP1_A::Div54
    }
    ///Checks if the value of the field is `Div56`
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == DIVP1_A::Div56
    }
    ///Checks if the value of the field is `Div58`
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == DIVP1_A::Div58
    }
    ///Checks if the value of the field is `Div60`
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == DIVP1_A::Div60
    }
    ///Checks if the value of the field is `Div62`
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == DIVP1_A::Div62
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIVP1_A::Div64
    }
    ///Checks if the value of the field is `Div66`
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == DIVP1_A::Div66
    }
    ///Checks if the value of the field is `Div68`
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == DIVP1_A::Div68
    }
    ///Checks if the value of the field is `Div70`
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == DIVP1_A::Div70
    }
    ///Checks if the value of the field is `Div72`
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == DIVP1_A::Div72
    }
    ///Checks if the value of the field is `Div74`
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == DIVP1_A::Div74
    }
    ///Checks if the value of the field is `Div76`
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == DIVP1_A::Div76
    }
    ///Checks if the value of the field is `Div78`
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == DIVP1_A::Div78
    }
    ///Checks if the value of the field is `Div80`
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == DIVP1_A::Div80
    }
    ///Checks if the value of the field is `Div82`
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == DIVP1_A::Div82
    }
    ///Checks if the value of the field is `Div84`
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == DIVP1_A::Div84
    }
    ///Checks if the value of the field is `Div86`
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == DIVP1_A::Div86
    }
    ///Checks if the value of the field is `Div88`
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == DIVP1_A::Div88
    }
    ///Checks if the value of the field is `Div90`
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == DIVP1_A::Div90
    }
    ///Checks if the value of the field is `Div92`
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == DIVP1_A::Div92
    }
    ///Checks if the value of the field is `Div94`
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == DIVP1_A::Div94
    }
    ///Checks if the value of the field is `Div96`
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == DIVP1_A::Div96
    }
    ///Checks if the value of the field is `Div98`
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == DIVP1_A::Div98
    }
    ///Checks if the value of the field is `Div100`
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == DIVP1_A::Div100
    }
    ///Checks if the value of the field is `Div102`
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == DIVP1_A::Div102
    }
    ///Checks if the value of the field is `Div104`
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == DIVP1_A::Div104
    }
    ///Checks if the value of the field is `Div106`
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == DIVP1_A::Div106
    }
    ///Checks if the value of the field is `Div108`
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == DIVP1_A::Div108
    }
    ///Checks if the value of the field is `Div110`
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == DIVP1_A::Div110
    }
    ///Checks if the value of the field is `Div112`
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == DIVP1_A::Div112
    }
    ///Checks if the value of the field is `Div114`
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == DIVP1_A::Div114
    }
    ///Checks if the value of the field is `Div116`
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == DIVP1_A::Div116
    }
    ///Checks if the value of the field is `Div118`
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == DIVP1_A::Div118
    }
    ///Checks if the value of the field is `Div120`
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == DIVP1_A::Div120
    }
    ///Checks if the value of the field is `Div122`
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == DIVP1_A::Div122
    }
    ///Checks if the value of the field is `Div124`
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == DIVP1_A::Div124
    }
    ///Checks if the value of the field is `Div126`
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == DIVP1_A::Div126
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DIVP1_A::Div128
    }
}
///Field `DIVP1` writer - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...
pub type DIVP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1DIVR_SPEC, u8, DIVP1_A, 7, O>;
impl<'a, const O: u8> DIVP1_W<'a, O> {
    ///pll_p_ck = vco_ck
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVP1_A::Div1)
    }
    ///pll_p_ck = vco_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVP1_A::Div2)
    }
    ///pll_p_ck = vco_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVP1_A::Div4)
    }
    ///pll_p_ck = vco_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(DIVP1_A::Div6)
    }
    ///pll_p_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVP1_A::Div8)
    }
    ///pll_p_ck = vco_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(DIVP1_A::Div10)
    }
    ///pll_p_ck = vco_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(DIVP1_A::Div12)
    }
    ///pll_p_ck = vco_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(DIVP1_A::Div14)
    }
    ///pll_p_ck = vco_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIVP1_A::Div16)
    }
    ///pll_p_ck = vco_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(DIVP1_A::Div18)
    }
    ///pll_p_ck = vco_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(DIVP1_A::Div20)
    }
    ///pll_p_ck = vco_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(DIVP1_A::Div22)
    }
    ///pll_p_ck = vco_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(DIVP1_A::Div24)
    }
    ///pll_p_ck = vco_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(DIVP1_A::Div26)
    }
    ///pll_p_ck = vco_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(DIVP1_A::Div28)
    }
    ///pll_p_ck = vco_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(DIVP1_A::Div30)
    }
    ///pll_p_ck = vco_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIVP1_A::Div32)
    }
    ///pll_p_ck = vco_ck / 34
    #[inline(always)]
    pub fn div34(self) -> &'a mut W {
        self.variant(DIVP1_A::Div34)
    }
    ///pll_p_ck = vco_ck / 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut W {
        self.variant(DIVP1_A::Div36)
    }
    ///pll_p_ck = vco_ck / 38
    #[inline(always)]
    pub fn div38(self) -> &'a mut W {
        self.variant(DIVP1_A::Div38)
    }
    ///pll_p_ck = vco_ck / 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut W {
        self.variant(DIVP1_A::Div40)
    }
    ///pll_p_ck = vco_ck / 42
    #[inline(always)]
    pub fn div42(self) -> &'a mut W {
        self.variant(DIVP1_A::Div42)
    }
    ///pll_p_ck = vco_ck / 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut W {
        self.variant(DIVP1_A::Div44)
    }
    ///pll_p_ck = vco_ck / 46
    #[inline(always)]
    pub fn div46(self) -> &'a mut W {
        self.variant(DIVP1_A::Div46)
    }
    ///pll_p_ck = vco_ck / 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(DIVP1_A::Div48)
    }
    ///pll_p_ck = vco_ck / 50
    #[inline(always)]
    pub fn div50(self) -> &'a mut W {
        self.variant(DIVP1_A::Div50)
    }
    ///pll_p_ck = vco_ck / 52
    #[inline(always)]
    pub fn div52(self) -> &'a mut W {
        self.variant(DIVP1_A::Div52)
    }
    ///pll_p_ck = vco_ck / 54
    #[inline(always)]
    pub fn div54(self) -> &'a mut W {
        self.variant(DIVP1_A::Div54)
    }
    ///pll_p_ck = vco_ck / 56
    #[inline(always)]
    pub fn div56(self) -> &'a mut W {
        self.variant(DIVP1_A::Div56)
    }
    ///pll_p_ck = vco_ck / 58
    #[inline(always)]
    pub fn div58(self) -> &'a mut W {
        self.variant(DIVP1_A::Div58)
    }
    ///pll_p_ck = vco_ck / 60
    #[inline(always)]
    pub fn div60(self) -> &'a mut W {
        self.variant(DIVP1_A::Div60)
    }
    ///pll_p_ck = vco_ck / 62
    #[inline(always)]
    pub fn div62(self) -> &'a mut W {
        self.variant(DIVP1_A::Div62)
    }
    ///pll_p_ck = vco_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DIVP1_A::Div64)
    }
    ///pll_p_ck = vco_ck / 66
    #[inline(always)]
    pub fn div66(self) -> &'a mut W {
        self.variant(DIVP1_A::Div66)
    }
    ///pll_p_ck = vco_ck / 68
    #[inline(always)]
    pub fn div68(self) -> &'a mut W {
        self.variant(DIVP1_A::Div68)
    }
    ///pll_p_ck = vco_ck / 70
    #[inline(always)]
    pub fn div70(self) -> &'a mut W {
        self.variant(DIVP1_A::Div70)
    }
    ///pll_p_ck = vco_ck / 72
    #[inline(always)]
    pub fn div72(self) -> &'a mut W {
        self.variant(DIVP1_A::Div72)
    }
    ///pll_p_ck = vco_ck / 74
    #[inline(always)]
    pub fn div74(self) -> &'a mut W {
        self.variant(DIVP1_A::Div74)
    }
    ///pll_p_ck = vco_ck / 76
    #[inline(always)]
    pub fn div76(self) -> &'a mut W {
        self.variant(DIVP1_A::Div76)
    }
    ///pll_p_ck = vco_ck / 78
    #[inline(always)]
    pub fn div78(self) -> &'a mut W {
        self.variant(DIVP1_A::Div78)
    }
    ///pll_p_ck = vco_ck / 80
    #[inline(always)]
    pub fn div80(self) -> &'a mut W {
        self.variant(DIVP1_A::Div80)
    }
    ///pll_p_ck = vco_ck / 82
    #[inline(always)]
    pub fn div82(self) -> &'a mut W {
        self.variant(DIVP1_A::Div82)
    }
    ///pll_p_ck = vco_ck / 84
    #[inline(always)]
    pub fn div84(self) -> &'a mut W {
        self.variant(DIVP1_A::Div84)
    }
    ///pll_p_ck = vco_ck / 86
    #[inline(always)]
    pub fn div86(self) -> &'a mut W {
        self.variant(DIVP1_A::Div86)
    }
    ///pll_p_ck = vco_ck / 88
    #[inline(always)]
    pub fn div88(self) -> &'a mut W {
        self.variant(DIVP1_A::Div88)
    }
    ///pll_p_ck = vco_ck / 90
    #[inline(always)]
    pub fn div90(self) -> &'a mut W {
        self.variant(DIVP1_A::Div90)
    }
    ///pll_p_ck = vco_ck / 92
    #[inline(always)]
    pub fn div92(self) -> &'a mut W {
        self.variant(DIVP1_A::Div92)
    }
    ///pll_p_ck = vco_ck / 94
    #[inline(always)]
    pub fn div94(self) -> &'a mut W {
        self.variant(DIVP1_A::Div94)
    }
    ///pll_p_ck = vco_ck / 96
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(DIVP1_A::Div96)
    }
    ///pll_p_ck = vco_ck / 98
    #[inline(always)]
    pub fn div98(self) -> &'a mut W {
        self.variant(DIVP1_A::Div98)
    }
    ///pll_p_ck = vco_ck / 100
    #[inline(always)]
    pub fn div100(self) -> &'a mut W {
        self.variant(DIVP1_A::Div100)
    }
    ///pll_p_ck = vco_ck / 102
    #[inline(always)]
    pub fn div102(self) -> &'a mut W {
        self.variant(DIVP1_A::Div102)
    }
    ///pll_p_ck = vco_ck / 104
    #[inline(always)]
    pub fn div104(self) -> &'a mut W {
        self.variant(DIVP1_A::Div104)
    }
    ///pll_p_ck = vco_ck / 106
    #[inline(always)]
    pub fn div106(self) -> &'a mut W {
        self.variant(DIVP1_A::Div106)
    }
    ///pll_p_ck = vco_ck / 108
    #[inline(always)]
    pub fn div108(self) -> &'a mut W {
        self.variant(DIVP1_A::Div108)
    }
    ///pll_p_ck = vco_ck / 110
    #[inline(always)]
    pub fn div110(self) -> &'a mut W {
        self.variant(DIVP1_A::Div110)
    }
    ///pll_p_ck = vco_ck / 112
    #[inline(always)]
    pub fn div112(self) -> &'a mut W {
        self.variant(DIVP1_A::Div112)
    }
    ///pll_p_ck = vco_ck / 114
    #[inline(always)]
    pub fn div114(self) -> &'a mut W {
        self.variant(DIVP1_A::Div114)
    }
    ///pll_p_ck = vco_ck / 116
    #[inline(always)]
    pub fn div116(self) -> &'a mut W {
        self.variant(DIVP1_A::Div116)
    }
    ///pll_p_ck = vco_ck / 118
    #[inline(always)]
    pub fn div118(self) -> &'a mut W {
        self.variant(DIVP1_A::Div118)
    }
    ///pll_p_ck = vco_ck / 120
    #[inline(always)]
    pub fn div120(self) -> &'a mut W {
        self.variant(DIVP1_A::Div120)
    }
    ///pll_p_ck = vco_ck / 122
    #[inline(always)]
    pub fn div122(self) -> &'a mut W {
        self.variant(DIVP1_A::Div122)
    }
    ///pll_p_ck = vco_ck / 124
    #[inline(always)]
    pub fn div124(self) -> &'a mut W {
        self.variant(DIVP1_A::Div124)
    }
    ///pll_p_ck = vco_ck / 126
    #[inline(always)]
    pub fn div126(self) -> &'a mut W {
        self.variant(DIVP1_A::Div126)
    }
    ///pll_p_ck = vco_ck / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DIVP1_A::Div128)
    }
}
///Field `DIVQ1` reader - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type DIVQ1_R = crate::FieldReader<u8, u8>;
///Field `DIVQ1` writer - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type DIVQ1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL1DIVR_SPEC, u8, u8, 7, O>;
///Field `DIVR1` reader - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type DIVR1_R = crate::FieldReader<u8, u8>;
///Field `DIVR1` writer - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type DIVR1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL1DIVR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:8 - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL1VCOSEL = 0 150 to 420Â MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x DIVN1, when fractional value 0 has been loaded into FRACN1, with: DIVN1 between 8 and 420 The input frequency Fref1_ck must be between 1 and 16Â MHz.
    #[inline(always)]
    pub fn divn1(&self) -> DIVN1_R {
        DIVN1_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...
    #[inline(always)]
    pub fn divp1(&self) -> DIVP1_R {
        DIVP1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn divq1(&self) -> DIVQ1_R {
        DIVQ1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn divr1(&self) -> DIVR1_R {
        DIVR1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:8 - multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = PLL1RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL1VCOSEL = 0 150 to 420Â MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x DIVN1, when fractional value 0 has been loaded into FRACN1, with: DIVN1 between 8 and 420 The input frequency Fref1_ck must be between 1 and 16Â MHz.
    #[inline(always)]
    #[must_use]
    pub fn divn1(&mut self) -> DIVN1_W<0> {
        DIVN1_W::new(self)
    }
    ///Bits 9:15 - PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ...
    #[inline(always)]
    #[must_use]
    pub fn divp1(&mut self) -> DIVP1_W<9> {
        DIVP1_W::new(self)
    }
    ///Bits 16:22 - PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn divq1(&mut self) -> DIVQ1_W<16> {
        DIVQ1_W::new(self)
    }
    ///Bits 24:30 - PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn divr1(&mut self) -> DIVR1_W<24> {
        DIVR1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pll1divr](index.html) module
pub struct PLL1DIVR_SPEC;
impl crate::RegisterSpec for PLL1DIVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pll1divr::R](R) reader structure
impl crate::Readable for PLL1DIVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pll1divr::W](W) writer structure
impl crate::Writable for PLL1DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLL1DIVR to value 0x0101_0280
impl crate::Resettable for PLL1DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
