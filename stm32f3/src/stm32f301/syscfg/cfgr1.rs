///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MEM_MODE` reader - Memory mapping selection bits
pub type MEM_MODE_R = crate::FieldReader<u8, MEM_MODE_A>;
///Memory mapping selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    ///0: Main Flash memory mapped at 0x0000_0000
    MainFlash = 0,
    ///1: System Flash memory mapped at 0x0000_0000
    SystemFlash = 1,
    ///2: Main Flash memory mapped at 0x0000_0000
    MainFlash2 = 2,
    ///3: Embedded SRAM mapped at 0x0000_0000
    Sram = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MEM_MODE_A {
        match self.bits {
            0 => MEM_MODE_A::MainFlash,
            1 => MEM_MODE_A::SystemFlash,
            2 => MEM_MODE_A::MainFlash2,
            3 => MEM_MODE_A::Sram,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MainFlash`
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MainFlash
    }
    ///Checks if the value of the field is `SystemFlash`
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SystemFlash
    }
    ///Checks if the value of the field is `MainFlash2`
    #[inline(always)]
    pub fn is_main_flash2(&self) -> bool {
        *self == MEM_MODE_A::MainFlash2
    }
    ///Checks if the value of the field is `Sram`
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::Sram
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, MEM_MODE_A, 2, O>;
impl<'a, const O: u8> MEM_MODE_W<'a, O> {
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash)
    }
    ///System Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SystemFlash)
    }
    ///Main Flash memory mapped at 0x0000_0000
    #[inline(always)]
    pub fn main_flash2(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash2)
    }
    ///Embedded SRAM mapped at 0x0000_0000
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Sram)
    }
}
///Field `TIM1_ITR3_RMP` reader - Timer 1 ITR3 selection
pub type TIM1_ITR3_RMP_R = crate::BitReader<TIM1_ITR3_RMP_A>;
///Timer 1 ITR3 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM1_ITR3_RMP_A {
    ///0: Not remapped
    NotRemapped = 0,
    ///1: TIM1_ITR3 = TIM17_OC
    Remapped = 1,
}
impl From<TIM1_ITR3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1_ITR3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1_ITR3_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM1_ITR3_RMP_A {
        match self.bits {
            false => TIM1_ITR3_RMP_A::NotRemapped,
            true => TIM1_ITR3_RMP_A::Remapped,
        }
    }
    ///Checks if the value of the field is `NotRemapped`
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP_A::NotRemapped
    }
    ///Checks if the value of the field is `Remapped`
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP_A::Remapped
    }
}
///Field `TIM1_ITR3_RMP` writer - Timer 1 ITR3 selection
pub type TIM1_ITR3_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM1_ITR3_RMP_A, O>;
impl<'a, const O: u8> TIM1_ITR3_RMP_W<'a, O> {
    ///Not remapped
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::NotRemapped)
    }
    ///TIM1_ITR3 = TIM17_OC
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::Remapped)
    }
}
///Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit
pub type TIM16_DMA_RMP_R = crate::BitReader<TIM16_DMA_RMP_A>;
///TIM16 DMA request remapping bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM16_DMA_RMP_A {
    ///0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3
    NotRemapped = 0,
    ///1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4
    Remapped = 1,
}
impl From<TIM16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM16_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP_A {
        match self.bits {
            false => TIM16_DMA_RMP_A::NotRemapped,
            true => TIM16_DMA_RMP_A::Remapped,
        }
    }
    ///Checks if the value of the field is `NotRemapped`
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::NotRemapped
    }
    ///Checks if the value of the field is `Remapped`
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::Remapped
    }
}
///Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit
pub type TIM16_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM16_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM16_DMA_RMP_W<'a, O> {
    ///TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::NotRemapped)
    }
    ///TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::Remapped)
    }
}
///Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit
pub type TIM17_DMA_RMP_R = crate::BitReader<TIM17_DMA_RMP_A>;
///TIM17 DMA request remapping bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM17_DMA_RMP_A {
    ///0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1
    NotRemapped = 0,
    ///1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2
    Remapped = 1,
}
impl From<TIM17_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM17_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP_A {
        match self.bits {
            false => TIM17_DMA_RMP_A::NotRemapped,
            true => TIM17_DMA_RMP_A::Remapped,
        }
    }
    ///Checks if the value of the field is `NotRemapped`
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::NotRemapped
    }
    ///Checks if the value of the field is `Remapped`
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::Remapped
    }
}
///Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit
pub type TIM17_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM17_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM17_DMA_RMP_W<'a, O> {
    ///TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::NotRemapped)
    }
    ///TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::Remapped)
    }
}
///Field `TIM6_DAC1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit
pub type TIM6_DAC1_DMA_RMP_R = crate::BitReader<TIM6_DAC1_DMA_RMP_A>;
///TIM6 and DAC1 DMA request remapping bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM6_DAC1_DMA_RMP_A {
    ///0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3
    NotRemapped = 0,
    ///1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3
    Remapped = 1,
}
impl From<TIM6_DAC1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6_DAC1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM6_DAC1_DMA_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIM6_DAC1_DMA_RMP_A {
        match self.bits {
            false => TIM6_DAC1_DMA_RMP_A::NotRemapped,
            true => TIM6_DAC1_DMA_RMP_A::Remapped,
        }
    }
    ///Checks if the value of the field is `NotRemapped`
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM6_DAC1_DMA_RMP_A::NotRemapped
    }
    ///Checks if the value of the field is `Remapped`
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM6_DAC1_DMA_RMP_A::Remapped
    }
}
///Field `TIM6_DAC1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit
pub type TIM6_DAC1_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM6_DAC1_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM6_DAC1_DMA_RMP_W<'a, O> {
    ///TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_DMA_RMP_A::NotRemapped)
    }
    ///TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_DMA_RMP_A::Remapped)
    }
}
///Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP_A>;
///Fast Mode Plus (FM+) driving capability activation bits.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP_A {
    ///0: PB6 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB6_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB6_FMP_A {
        match self.bits {
            false => I2C_PB6_FMP_A::Standard,
            true => I2C_PB6_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP_A::Fmp
    }
}
///Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB6_FMP_A, O>;
impl<'a, const O: u8> I2C_PB6_FMP_W<'a, O> {
    ///PB6 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB6 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Fmp)
    }
}
///Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP_A>;
///Fast Mode Plus (FM+) driving capability activation bits.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP_A {
    ///0: PB7 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB7_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB7_FMP_A {
        match self.bits {
            false => I2C_PB7_FMP_A::Standard,
            true => I2C_PB7_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP_A::Fmp
    }
}
///Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB7_FMP_A, O>;
impl<'a, const O: u8> I2C_PB7_FMP_W<'a, O> {
    ///PB7 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB7 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Fmp)
    }
}
///Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP_A>;
///Fast Mode Plus (FM+) driving capability activation bits.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP_A {
    ///0: PB8 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB8_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB8_FMP_A {
        match self.bits {
            false => I2C_PB8_FMP_A::Standard,
            true => I2C_PB8_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP_A::Fmp
    }
}
///Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB8_FMP_A, O>;
impl<'a, const O: u8> I2C_PB8_FMP_W<'a, O> {
    ///PB8 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB8 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Fmp)
    }
}
///Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP_A>;
///Fast Mode Plus (FM+) driving capability activation bits.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP_A {
    ///0: PB9 pin operate in standard mode
    Standard = 0,
    ///1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    Fmp = 1,
}
impl From<I2C_PB9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C_PB9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB9_FMP_A {
        match self.bits {
            false => I2C_PB9_FMP_A::Standard,
            true => I2C_PB9_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP_A::Fmp
    }
}
///Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits.
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB9_FMP_A, O>;
impl<'a, const O: u8> I2C_PB9_FMP_W<'a, O> {
    ///PB9 pin operate in standard mode
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Standard)
    }
    ///I2C FM+ mode enabled on PB9 and the Speed control is bypassed
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Fmp)
    }
}
///Field `I2C1_FMP` reader - I2C1 Fast Mode Plus
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP_A>;
///I2C1 Fast Mode Plus
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits
    Fmp = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C1_FMP_A {
        match self.bits {
            false => I2C1_FMP_A::Standard,
            true => I2C1_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP_A::Fmp
    }
}
///Field `I2C1_FMP` writer - I2C1 Fast Mode Plus
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C1_FMP_A, O>;
impl<'a, const O: u8> I2C1_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Fmp)
    }
}
///Field `I2C2_FMP` reader - I2C2 Fast Mode Plus
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP_A>;
///I2C2 Fast Mode Plus
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits
    Fmp = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C2_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C2_FMP_A {
        match self.bits {
            false => I2C2_FMP_A::Standard,
            true => I2C2_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP_A::Fmp
    }
}
///Field `I2C2_FMP` writer - I2C2 Fast Mode Plus
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C2_FMP_A, O>;
impl<'a, const O: u8> I2C2_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Fmp)
    }
}
///Field `ENCODER_MODE` reader - Encoder mode
pub type ENCODER_MODE_R = crate::FieldReader<u8, ENCODER_MODE_A>;
///Encoder mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENCODER_MODE_A {
    ///0: No redirection
    NoRedirection = 0,
    ///1: TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively
    MapTim2tim15 = 1,
}
impl From<ENCODER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENCODER_MODE_A) -> Self {
        variant as _
    }
}
impl ENCODER_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ENCODER_MODE_A> {
        match self.bits {
            0 => Some(ENCODER_MODE_A::NoRedirection),
            1 => Some(ENCODER_MODE_A::MapTim2tim15),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoRedirection`
    #[inline(always)]
    pub fn is_no_redirection(&self) -> bool {
        *self == ENCODER_MODE_A::NoRedirection
    }
    ///Checks if the value of the field is `MapTim2tim15`
    #[inline(always)]
    pub fn is_map_tim2tim15(&self) -> bool {
        *self == ENCODER_MODE_A::MapTim2tim15
    }
}
///Field `ENCODER_MODE` writer - Encoder mode
pub type ENCODER_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, ENCODER_MODE_A, 2, O>;
impl<'a, const O: u8> ENCODER_MODE_W<'a, O> {
    ///No redirection
    #[inline(always)]
    pub fn no_redirection(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::NoRedirection)
    }
    ///TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively
    #[inline(always)]
    pub fn map_tim2tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MapTim2tim15)
    }
}
///Field `I2C3_FMP` reader - I2C3 Fast Mode Plus
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP_A>;
///I2C3 Fast Mode Plus
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C3_FMP_A {
    ///0: FM+ mode is controlled by I2C_Pxx_FMP bits only
    Standard = 0,
    ///1: FM+ mode is enabled on all I2C3 pins selected through selection trhough IOPORT control registers AF selection bits
    Fmp = 1,
}
impl From<I2C3_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C3_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C3_FMP_A {
        match self.bits {
            false => I2C3_FMP_A::Standard,
            true => I2C3_FMP_A::Fmp,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP_A::Standard
    }
    ///Checks if the value of the field is `Fmp`
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP_A::Fmp
    }
}
///Field `I2C3_FMP` writer - I2C3 Fast Mode Plus
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C3_FMP_A, O>;
impl<'a, const O: u8> I2C3_FMP_W<'a, O> {
    ///FM+ mode is controlled by I2C_Pxx_FMP bits only
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Standard)
    }
    ///FM+ mode is enabled on all I2C3 pins selected through selection trhough IOPORT control registers AF selection bits
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Fmp)
    }
}
///Field `FPU_IE0` reader - Invalid operation interrupt enable
pub type FPU_IE0_R = crate::BitReader<FPU_IE0_A>;
///Invalid operation interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE0_A {
    ///0: Invalid operation interrupt disable
    Disabled = 0,
    ///1: Invalid operation interrupt enable
    Enabled = 1,
}
impl From<FPU_IE0_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE0_A {
        match self.bits {
            false => FPU_IE0_A::Disabled,
            true => FPU_IE0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0_A::Enabled
    }
}
///Field `FPU_IE0` writer - Invalid operation interrupt enable
pub type FPU_IE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE0_A, O>;
impl<'a, const O: u8> FPU_IE0_W<'a, O> {
    ///Invalid operation interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Disabled)
    }
    ///Invalid operation interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Enabled)
    }
}
///Field `FPU_IE1` reader - Devide-by-zero interrupt enable
pub type FPU_IE1_R = crate::BitReader<FPU_IE1_A>;
///Devide-by-zero interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE1_A {
    ///0: Devide-by-zero interrupt disable
    Disabled = 0,
    ///1: Devide-by-zero interrupt enable
    Enabled = 1,
}
impl From<FPU_IE1_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE1_A {
        match self.bits {
            false => FPU_IE1_A::Disabled,
            true => FPU_IE1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE1_A::Enabled
    }
}
///Field `FPU_IE1` writer - Devide-by-zero interrupt enable
pub type FPU_IE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE1_A, O>;
impl<'a, const O: u8> FPU_IE1_W<'a, O> {
    ///Devide-by-zero interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::Disabled)
    }
    ///Devide-by-zero interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::Enabled)
    }
}
///Field `FPU_IE2` reader - Underflow interrupt enable
pub type FPU_IE2_R = crate::BitReader<FPU_IE2_A>;
///Underflow interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE2_A {
    ///0: Underflow interrupt disable
    Disabled = 0,
    ///1: Underflow interrupt enable
    Enabled = 1,
}
impl From<FPU_IE2_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE2_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE2_A {
        match self.bits {
            false => FPU_IE2_A::Disabled,
            true => FPU_IE2_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE2_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE2_A::Enabled
    }
}
///Field `FPU_IE2` writer - Underflow interrupt enable
pub type FPU_IE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE2_A, O>;
impl<'a, const O: u8> FPU_IE2_W<'a, O> {
    ///Underflow interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::Disabled)
    }
    ///Underflow interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::Enabled)
    }
}
///Field `FPU_IE3` reader - Overflow interrupt enable
pub type FPU_IE3_R = crate::BitReader<FPU_IE3_A>;
///Overflow interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE3_A {
    ///0: Overflow interrupt disable
    Disabled = 0,
    ///1: Overflow interrupt enable
    Enabled = 1,
}
impl From<FPU_IE3_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE3_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE3_A {
        match self.bits {
            false => FPU_IE3_A::Disabled,
            true => FPU_IE3_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE3_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE3_A::Enabled
    }
}
///Field `FPU_IE3` writer - Overflow interrupt enable
pub type FPU_IE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE3_A, O>;
impl<'a, const O: u8> FPU_IE3_W<'a, O> {
    ///Overflow interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::Disabled)
    }
    ///Overflow interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::Enabled)
    }
}
///Field `FPU_IE4` reader - Input denormal interrupt enable
pub type FPU_IE4_R = crate::BitReader<FPU_IE4_A>;
///Input denormal interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE4_A {
    ///0: Input denormal interrupt disable
    Disabled = 0,
    ///1: Input denormal interrupt enable
    Enabled = 1,
}
impl From<FPU_IE4_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE4_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE4_A {
        match self.bits {
            false => FPU_IE4_A::Disabled,
            true => FPU_IE4_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE4_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE4_A::Enabled
    }
}
///Field `FPU_IE4` writer - Input denormal interrupt enable
pub type FPU_IE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE4_A, O>;
impl<'a, const O: u8> FPU_IE4_W<'a, O> {
    ///Input denormal interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::Disabled)
    }
    ///Input denormal interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::Enabled)
    }
}
///Field `FPU_IE5` reader - Inexact interrupt enable
pub type FPU_IE5_R = crate::BitReader<FPU_IE5_A>;
///Inexact interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPU_IE5_A {
    ///0: Inexact interrupt disable
    Disabled = 0,
    ///1: Inexact interrupt enable
    Enabled = 1,
}
impl From<FPU_IE5_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE5_A) -> Self {
        variant as u8 != 0
    }
}
impl FPU_IE5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE5_A {
        match self.bits {
            false => FPU_IE5_A::Disabled,
            true => FPU_IE5_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE5_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE5_A::Enabled
    }
}
///Field `FPU_IE5` writer - Inexact interrupt enable
pub type FPU_IE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE5_A, O>;
impl<'a, const O: u8> FPU_IE5_W<'a, O> {
    ///Inexact interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::Disabled)
    }
    ///Inexact interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::Enabled)
    }
}
impl R {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 6 - Timer 1 ITR3 selection
    #[inline(always)]
    pub fn tim1_itr3_rmp(&self) -> TIM1_ITR3_RMP_R {
        TIM1_ITR3_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - TIM16 DMA request remapping bit
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIM17 DMA request remapping bit
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM6 and DAC1 DMA request remapping bit
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&self) -> TIM6_DAC1_DMA_RMP_R {
        TIM6_DAC1_DMA_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - I2C1 Fast Mode Plus
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C2 Fast Mode Plus
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:23 - Encoder mode
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - I2C3 Fast Mode Plus
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Invalid operation interrupt enable
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Devide-by-zero interrupt enable
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Underflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Overflow interrupt enable
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Input denormal interrupt enable
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Inexact interrupt enable
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    ///Bit 6 - Timer 1 ITR3 selection
    #[inline(always)]
    #[must_use]
    pub fn tim1_itr3_rmp(&mut self) -> TIM1_ITR3_RMP_W<6> {
        TIM1_ITR3_RMP_W::new(self)
    }
    ///Bit 11 - TIM16 DMA request remapping bit
    #[inline(always)]
    #[must_use]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<11> {
        TIM16_DMA_RMP_W::new(self)
    }
    ///Bit 12 - TIM17 DMA request remapping bit
    #[inline(always)]
    #[must_use]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<12> {
        TIM17_DMA_RMP_W::new(self)
    }
    ///Bit 13 - TIM6 and DAC1 DMA request remapping bit
    #[inline(always)]
    #[must_use]
    pub fn tim6_dac1_dma_rmp(&mut self) -> TIM6_DAC1_DMA_RMP_W<13> {
        TIM6_DAC1_DMA_RMP_W::new(self)
    }
    ///Bit 16 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<16> {
        I2C_PB6_FMP_W::new(self)
    }
    ///Bit 17 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<17> {
        I2C_PB7_FMP_W::new(self)
    }
    ///Bit 18 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<18> {
        I2C_PB8_FMP_W::new(self)
    }
    ///Bit 19 - Fast Mode Plus (FM+) driving capability activation bits.
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<19> {
        I2C_PB9_FMP_W::new(self)
    }
    ///Bit 20 - I2C1 Fast Mode Plus
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<20> {
        I2C1_FMP_W::new(self)
    }
    ///Bit 21 - I2C2 Fast Mode Plus
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<21> {
        I2C2_FMP_W::new(self)
    }
    ///Bits 22:23 - Encoder mode
    #[inline(always)]
    #[must_use]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<22> {
        ENCODER_MODE_W::new(self)
    }
    ///Bit 24 - I2C3 Fast Mode Plus
    #[inline(always)]
    #[must_use]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<24> {
        I2C3_FMP_W::new(self)
    }
    ///Bit 26 - Invalid operation interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<26> {
        FPU_IE0_W::new(self)
    }
    ///Bit 27 - Devide-by-zero interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<27> {
        FPU_IE1_W::new(self)
    }
    ///Bit 28 - Underflow interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<28> {
        FPU_IE2_W::new(self)
    }
    ///Bit 29 - Overflow interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<29> {
        FPU_IE3_W::new(self)
    }
    ///Bit 30 - Input denormal interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<30> {
        FPU_IE4_W::new(self)
    }
    ///Bit 31 - Inexact interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<31> {
        FPU_IE5_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
