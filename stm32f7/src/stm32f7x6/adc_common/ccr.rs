///Register `CCR` reader
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
///Register `CCR` writer
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
///Field `MULTI` reader - Multi ADC mode selection
pub type MULTI_R = crate::FieldReader<u8, MULTI_A>;
///Multi ADC mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULTI_A {
    ///0: All the ADCs independent: independent mode
    Independent = 0,
    ///1: Dual ADC1 and ADC2, combined regular and injected simultaneous mode
    DualRj = 1,
    ///2: Dual ADC1 and ADC2, combined regular and alternate trigger mode
    DualRa = 2,
    ///5: Dual ADC1 and ADC2, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual ADC1 and ADC2, regular simultaneous mode only
    DualR = 6,
    ///7: Dual ADC1 and ADC2, interleaved mode only
    DualI = 7,
    ///9: Dual ADC1 and ADC2, alternate trigger mode only
    DualA = 9,
    ///17: Triple ADC, regular and injected simultaneous mode
    TripleRj = 17,
    ///18: Triple ADC, regular and alternate trigger mode
    TripleRa = 18,
    ///21: Triple ADC, injected simultaneous mode only
    TripleJ = 21,
    ///22: Triple ADC, regular simultaneous mode only
    TripleR = 22,
    ///23: Triple ADC, interleaved mode only
    TripleI = 23,
    ///24: Triple ADC, alternate trigger mode only
    TripleA = 24,
}
impl From<MULTI_A> for u8 {
    #[inline(always)]
    fn from(variant: MULTI_A) -> Self {
        variant as _
    }
}
impl MULTI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MULTI_A> {
        match self.bits {
            0 => Some(MULTI_A::Independent),
            1 => Some(MULTI_A::DualRj),
            2 => Some(MULTI_A::DualRa),
            5 => Some(MULTI_A::DualJ),
            6 => Some(MULTI_A::DualR),
            7 => Some(MULTI_A::DualI),
            9 => Some(MULTI_A::DualA),
            17 => Some(MULTI_A::TripleRj),
            18 => Some(MULTI_A::TripleRa),
            21 => Some(MULTI_A::TripleJ),
            22 => Some(MULTI_A::TripleR),
            23 => Some(MULTI_A::TripleI),
            24 => Some(MULTI_A::TripleA),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Independent`
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == MULTI_A::Independent
    }
    ///Checks if the value of the field is `DualRj`
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == MULTI_A::DualRj
    }
    ///Checks if the value of the field is `DualRa`
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == MULTI_A::DualRa
    }
    ///Checks if the value of the field is `DualJ`
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == MULTI_A::DualJ
    }
    ///Checks if the value of the field is `DualR`
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == MULTI_A::DualR
    }
    ///Checks if the value of the field is `DualI`
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == MULTI_A::DualI
    }
    ///Checks if the value of the field is `DualA`
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == MULTI_A::DualA
    }
    ///Checks if the value of the field is `TripleRj`
    #[inline(always)]
    pub fn is_triple_rj(&self) -> bool {
        *self == MULTI_A::TripleRj
    }
    ///Checks if the value of the field is `TripleRa`
    #[inline(always)]
    pub fn is_triple_ra(&self) -> bool {
        *self == MULTI_A::TripleRa
    }
    ///Checks if the value of the field is `TripleJ`
    #[inline(always)]
    pub fn is_triple_j(&self) -> bool {
        *self == MULTI_A::TripleJ
    }
    ///Checks if the value of the field is `TripleR`
    #[inline(always)]
    pub fn is_triple_r(&self) -> bool {
        *self == MULTI_A::TripleR
    }
    ///Checks if the value of the field is `TripleI`
    #[inline(always)]
    pub fn is_triple_i(&self) -> bool {
        *self == MULTI_A::TripleI
    }
    ///Checks if the value of the field is `TripleA`
    #[inline(always)]
    pub fn is_triple_a(&self) -> bool {
        *self == MULTI_A::TripleA
    }
}
///Field `MULTI` writer - Multi ADC mode selection
pub type MULTI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, MULTI_A, 5, O>;
impl<'a, const O: u8> MULTI_W<'a, O> {
    ///All the ADCs independent: independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(MULTI_A::Independent)
    }
    ///Dual ADC1 and ADC2, combined regular and injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(MULTI_A::DualRj)
    }
    ///Dual ADC1 and ADC2, combined regular and alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(MULTI_A::DualRa)
    }
    ///Dual ADC1 and ADC2, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(MULTI_A::DualJ)
    }
    ///Dual ADC1 and ADC2, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(MULTI_A::DualR)
    }
    ///Dual ADC1 and ADC2, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(MULTI_A::DualI)
    }
    ///Dual ADC1 and ADC2, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(MULTI_A::DualA)
    }
    ///Triple ADC, regular and injected simultaneous mode
    #[inline(always)]
    pub fn triple_rj(self) -> &'a mut W {
        self.variant(MULTI_A::TripleRj)
    }
    ///Triple ADC, regular and alternate trigger mode
    #[inline(always)]
    pub fn triple_ra(self) -> &'a mut W {
        self.variant(MULTI_A::TripleRa)
    }
    ///Triple ADC, injected simultaneous mode only
    #[inline(always)]
    pub fn triple_j(self) -> &'a mut W {
        self.variant(MULTI_A::TripleJ)
    }
    ///Triple ADC, regular simultaneous mode only
    #[inline(always)]
    pub fn triple_r(self) -> &'a mut W {
        self.variant(MULTI_A::TripleR)
    }
    ///Triple ADC, interleaved mode only
    #[inline(always)]
    pub fn triple_i(self) -> &'a mut W {
        self.variant(MULTI_A::TripleI)
    }
    ///Triple ADC, alternate trigger mode only
    #[inline(always)]
    pub fn triple_a(self) -> &'a mut W {
        self.variant(MULTI_A::TripleA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader<u8, u8>;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 4, O>;
///Field `DDS` reader - DMA disable selection for multi-ADC mode
pub type DDS_R = crate::BitReader<DDS_A>;
///DMA disable selection for multi-ADC mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS_A {
    ///0: No new DMA request is issued after the last transfer
    Single = 0,
    ///1: DMA requests are issued as long as data are converted and DMA=01, 10 or 11
    Continuous = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::Single,
            true => DDS_A::Continuous,
        }
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS_A::Single
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS_A::Continuous
    }
}
///Field `DDS` writer - DMA disable selection for multi-ADC mode
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DDS_A, O>;
impl<'a, const O: u8> DDS_W<'a, O> {
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::Single)
    }
    ///DMA requests are issued as long as data are converted and DMA=01, 10 or 11
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::Continuous)
    }
}
///Field `DMA` reader - Direct memory access mode for multi ADC mode
pub type DMA_R = crate::FieldReader<u8, DMA_A>;
///Direct memory access mode for multi ADC mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA_A {
    ///0: DMA mode disabled
    Disabled = 0,
    ///1: DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
    Mode1 = 1,
    ///2: DMA mode 2 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)
    Mode2 = 2,
    ///3: DMA mode 3 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)
    Mode3 = 3,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as _
    }
}
impl DMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            0 => DMA_A::Disabled,
            1 => DMA_A::Mode1,
            2 => DMA_A::Mode2,
            3 => DMA_A::Mode3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::Disabled
    }
    ///Checks if the value of the field is `Mode1`
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == DMA_A::Mode1
    }
    ///Checks if the value of the field is `Mode2`
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == DMA_A::Mode2
    }
    ///Checks if the value of the field is `Mode3`
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == DMA_A::Mode3
    }
}
///Field `DMA` writer - Direct memory access mode for multi ADC mode
pub type DMA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, DMA_A, 2, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    ///DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::Disabled)
    }
    ///DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(DMA_A::Mode1)
    }
    ///DMA mode 2 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(DMA_A::Mode2)
    }
    ///DMA mode 3 enabled (2 / 3 half-words by pairs - 2&amp;1 then 1&amp;3 then 3&amp;2)
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(DMA_A::Mode3)
    }
}
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader<u8, ADCPRE_A>;
///ADC prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE_A {
    ///0: PCLK2 divided by 2
    Div2 = 0,
    ///1: PCLK2 divided by 4
    Div4 = 1,
    ///2: PCLK2 divided by 6
    Div6 = 2,
    ///3: PCLK2 divided by 8
    Div8 = 3,
}
impl From<ADCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE_A) -> Self {
        variant as _
    }
}
impl ADCPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCPRE_A {
        match self.bits {
            0 => ADCPRE_A::Div2,
            1 => ADCPRE_A::Div4,
            2 => ADCPRE_A::Div6,
            3 => ADCPRE_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE_A::Div8
    }
}
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, ADCPRE_A, 2, O>;
impl<'a, const O: u8> ADCPRE_W<'a, O> {
    ///PCLK2 divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div2)
    }
    ///PCLK2 divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div4)
    }
    ///PCLK2 divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div6)
    }
    ///PCLK2 divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div8)
    }
}
///Field `VBATE` reader - VBAT enable
pub type VBATE_R = crate::BitReader<VBATE_A>;
///VBAT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATE_A {
    ///0: V_BAT channel disabled
    Disabled = 0,
    ///1: V_BAT channel enabled
    Enabled = 1,
}
impl From<VBATE_A> for bool {
    #[inline(always)]
    fn from(variant: VBATE_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VBATE_A {
        match self.bits {
            false => VBATE_A::Disabled,
            true => VBATE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATE_A::Enabled
    }
}
///Field `VBATE` writer - VBAT enable
pub type VBATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATE_A, O>;
impl<'a, const O: u8> VBATE_W<'a, O> {
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATE_A::Disabled)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATE_A::Enabled)
    }
}
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader<TSVREFE_A>;
///Temperature sensor and VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE_A {
    ///0: Temperature sensor and V_REFINT channel disabled
    Disabled = 0,
    ///1: Temperature sensor and V_REFINT channel enabled
    Enabled = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSVREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::Disabled,
            true => TSVREFE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE_A::Enabled
    }
}
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, TSVREFE_A, O>;
impl<'a, const O: u8> TSVREFE_W<'a, O> {
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Disabled)
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    pub fn multi(&self) -> MULTI_R {
        MULTI_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Multi ADC mode selection
    #[inline(always)]
    #[must_use]
    pub fn multi(&mut self) -> MULTI_W<0> {
        MULTI_W::new(self)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    ///Bit 13 - DMA disable selection for multi-ADC mode
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<13> {
        DDS_W::new(self)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<14> {
        DMA_W::new(self)
    }
    ///Bits 16:17 - ADC prescaler
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> ADCPRE_W<16> {
        ADCPRE_W::new(self)
    }
    ///Bit 22 - VBAT enable
    #[inline(always)]
    #[must_use]
    pub fn vbate(&mut self) -> VBATE_W<22> {
        VBATE_W::new(self)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<23> {
        TSVREFE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC common control register
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
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
