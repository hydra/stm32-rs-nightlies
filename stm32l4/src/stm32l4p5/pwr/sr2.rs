///Register `SR2` reader
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `REGLPS` reader - Low-power regulator started
pub type REGLPS_R = crate::BitReader<REGLPS_A>;
///Low-power regulator started
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS_A {
    ///0: The low-power regulator is not ready
    NotReady = 0,
    ///1: The low-power regulator is ready
    Ready = 1,
}
impl From<REGLPS_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPS_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REGLPS_A {
        match self.bits {
            false => REGLPS_A::NotReady,
            true => REGLPS_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS_A::Ready
    }
}
///Field `REGLPF` reader - Low-power regulator flag
pub type REGLPF_R = crate::BitReader<REGLPF_A>;
///Low-power regulator flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF_A {
    ///0: The regulator is ready in main mode (MR)
    Mr = 0,
    ///1: The regulator is in low-power mode (LPR)
    Lpr = 1,
}
impl From<REGLPF_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPF_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REGLPF_A {
        match self.bits {
            false => REGLPF_A::Mr,
            true => REGLPF_A::Lpr,
        }
    }
    ///Checks if the value of the field is `Mr`
    #[inline(always)]
    pub fn is_mr(&self) -> bool {
        *self == REGLPF_A::Mr
    }
    ///Checks if the value of the field is `Lpr`
    #[inline(always)]
    pub fn is_lpr(&self) -> bool {
        *self == REGLPF_A::Lpr
    }
}
///Field `VOSF` reader - Voltage scaling flag
pub type VOSF_R = crate::BitReader<VOSF_A>;
///Voltage scaling flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF_A {
    ///0: The regulator is ready in the selected voltage range
    Ready = 0,
    ///1: The regulator output voltage is changing to the required voltage level
    NotReady = 1,
}
impl From<VOSF_A> for bool {
    #[inline(always)]
    fn from(variant: VOSF_A) -> Self {
        variant as u8 != 0
    }
}
impl VOSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VOSF_A {
        match self.bits {
            false => VOSF_A::Ready,
            true => VOSF_A::NotReady,
        }
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF_A::Ready
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSF_A::NotReady
    }
}
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader<PVDO_A>;
///Power voltage detector output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO_A {
    ///0: VDD is above the selected PVD threshold
    Above = 0,
    ///1: VDD is below the selected PVD threshold
    Below = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::Above,
            true => PVDO_A::Below,
        }
    }
    ///Checks if the value of the field is `Above`
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO_A::Above
    }
    ///Checks if the value of the field is `Below`
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO_A::Below
    }
}
///Field `PVMO1` reader - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
pub type PVMO1_R = crate::BitReader<PVMO1_A>;
///Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO1_A {
    ///0: VDDUSB voltage is above PVM1 threshold (around 1.2 V)
    Above = 0,
    ///1: VDDUSB voltage is below PVM1 threshold (around 1.2 V)
    Below = 1,
}
impl From<PVMO1_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO1_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVMO1_A {
        match self.bits {
            false => PVMO1_A::Above,
            true => PVMO1_A::Below,
        }
    }
    ///Checks if the value of the field is `Above`
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO1_A::Above
    }
    ///Checks if the value of the field is `Below`
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO1_A::Below
    }
}
///Field `PVMO2` reader - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V
pub type PVMO2_R = crate::BitReader<PVMO2_A>;
///Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO2_A {
    ///0: VDDIO2 voltage is above PVM2 threshold (around 0.9 V)
    Above = 0,
    ///1: VDDIO2 voltage is below PVM2 threshold (around 0.9 V)
    Below = 1,
}
impl From<PVMO2_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO2_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVMO2_A {
        match self.bits {
            false => PVMO2_A::Above,
            true => PVMO2_A::Below,
        }
    }
    ///Checks if the value of the field is `Above`
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO2_A::Above
    }
    ///Checks if the value of the field is `Below`
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO2_A::Below
    }
}
///Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V
pub type PVMO3_R = crate::BitReader<PVMO3_A>;
///Peripheral voltage monitoring output: VDDA vs. 1.62 V
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3_A {
    ///0: VDDA voltage is above PVM3 threshold (around 1.62 V)
    Above = 0,
    ///1: VDDA voltage is below PVM3 threshold (around 1.62 V)
    Below = 1,
}
impl From<PVMO3_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO3_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVMO3_A {
        match self.bits {
            false => PVMO3_A::Above,
            true => PVMO3_A::Below,
        }
    }
    ///Checks if the value of the field is `Above`
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3_A::Above
    }
    ///Checks if the value of the field is `Below`
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3_A::Below
    }
}
///Field `PVMO4` reader - Peripheral voltage monitoring output: VDDA vs. 2.2 V
pub type PVMO4_R = crate::BitReader<PVMO4_A>;
///Peripheral voltage monitoring output: VDDA vs. 2.2 V
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO4_A {
    ///0: VDDA voltage is above PVM4 threshold (around 2.2 V)
    Above = 0,
    ///1: VDDA voltage is below PVM4 threshold (around 2.2 V)
    Below = 1,
}
impl From<PVMO4_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO4_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PVMO4_A {
        match self.bits {
            false => PVMO4_A::Above,
            true => PVMO4_A::Below,
        }
    }
    ///Checks if the value of the field is `Above`
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO4_A::Above
    }
    ///Checks if the value of the field is `Below`
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO4_A::Below
    }
}
impl R {
    ///Bit 8 - Low-power regulator started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V
    #[inline(always)]
    pub fn pvmo4(&self) -> PVMO4_R {
        PVMO4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///Power status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr2](index.html) module
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr2::R](R) reader structure
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
