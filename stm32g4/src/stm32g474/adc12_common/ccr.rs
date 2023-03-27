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
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader<u8, DUAL_A>;
///Dual ADC mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL_A {
    ///0: Independent mode
    Independent = 0,
    ///1: Dual, combined regular simultaneous + injected simultaneous mode
    DualRj = 1,
    ///2: Dual, combined regular simultaneous + alternate trigger mode
    DualRa = 2,
    ///3: Dual, combined interleaved mode + injected simultaneous mode
    DualIj = 3,
    ///5: Dual, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual, regular simultaneous mode only
    DualR = 6,
    ///7: Dual, interleaved mode only
    DualI = 7,
    ///9: Dual, alternate trigger mode only
    DualA = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
impl DUAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DUAL_A> {
        match self.bits {
            0 => Some(DUAL_A::Independent),
            1 => Some(DUAL_A::DualRj),
            2 => Some(DUAL_A::DualRa),
            3 => Some(DUAL_A::DualIj),
            5 => Some(DUAL_A::DualJ),
            6 => Some(DUAL_A::DualR),
            7 => Some(DUAL_A::DualI),
            9 => Some(DUAL_A::DualA),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Independent`
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL_A::Independent
    }
    ///Checks if the value of the field is `DualRj`
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL_A::DualRj
    }
    ///Checks if the value of the field is `DualRa`
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL_A::DualRa
    }
    ///Checks if the value of the field is `DualIj`
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL_A::DualIj
    }
    ///Checks if the value of the field is `DualJ`
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL_A::DualJ
    }
    ///Checks if the value of the field is `DualR`
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL_A::DualR
    }
    ///Checks if the value of the field is `DualI`
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL_A::DualI
    }
    ///Checks if the value of the field is `DualA`
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL_A::DualA
    }
}
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, DUAL_A, 5, O>;
impl<'a, const O: u8> DUAL_W<'a, O> {
    ///Independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::Independent)
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DualRj)
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DualRa)
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DualIj)
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DualJ)
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DualR)
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DualI)
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DualA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader<u8, u8>;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 4, O>;
///Field `DMACFG` reader - DMA configuration (for multi-ADC mode)
pub type DMACFG_R = crate::BitReader<bool>;
///Field `DMACFG` writer - DMA configuration (for multi-ADC mode)
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
///Field `MDMA` reader - Direct memory access mode for multi ADC mode
pub type MDMA_R = crate::FieldReader<u8, u8>;
///Field `MDMA` writer - Direct memory access mode for multi ADC mode
pub type MDMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 2, O>;
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<u8, CKMODE_A>;
///ADC clock mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    ///0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    Asynchronous = 0,
    ///1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    SyncDiv1 = 1,
    ///2: Use AHB clock rcc_hclk3 divided by 2
    SyncDiv2 = 2,
    ///3: Use AHB clock rcc_hclk3 divided by 4
    SyncDiv4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::Asynchronous,
            1 => CKMODE_A::SyncDiv1,
            2 => CKMODE_A::SyncDiv2,
            3 => CKMODE_A::SyncDiv4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Asynchronous`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE_A::Asynchronous
    }
    ///Checks if the value of the field is `SyncDiv1`
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE_A::SyncDiv1
    }
    ///Checks if the value of the field is `SyncDiv2`
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE_A::SyncDiv2
    }
    ///Checks if the value of the field is `SyncDiv4`
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE_A::SyncDiv4
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, CKMODE_A, 2, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::Asynchronous)
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv1)
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv2)
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv4)
    }
}
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader<u8, u8>;
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 4, O>;
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
///VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    ///0: V_REFINT channel disabled
    Disabled = 0,
    ///1: V_REFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::Disabled,
            true => VREFEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::Enabled
    }
}
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VREFEN_A, O>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Disabled)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Enabled)
    }
}
///Field `VSENSESEL` reader - VTS selection
pub type VSENSESEL_R = crate::BitReader<VSENSESEL_A>;
///VTS selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSESEL_A {
    ///0: Temperature sensor channel disabled
    Disabled = 0,
    ///1: Temperature sensor channel enabled
    Enabled = 1,
}
impl From<VSENSESEL_A> for bool {
    #[inline(always)]
    fn from(variant: VSENSESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VSENSESEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSENSESEL_A {
        match self.bits {
            false => VSENSESEL_A::Disabled,
            true => VSENSESEL_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSESEL_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSESEL_A::Enabled
    }
}
///Field `VSENSESEL` writer - VTS selection
pub type VSENSESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VSENSESEL_A, O>;
impl<'a, const O: u8> VSENSESEL_W<'a, O> {
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VSENSESEL_A::Disabled)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VSENSESEL_A::Enabled)
    }
}
///Field `VBATSEL` reader - VBAT selection
pub type VBATSEL_R = crate::BitReader<VBATSEL_A>;
///VBAT selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATSEL_A {
    ///0: V_BAT channel disabled
    Disabled = 0,
    ///1: V_BAT channel enabled
    Enabled = 1,
}
impl From<VBATSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VBATSEL_A {
        match self.bits {
            false => VBATSEL_A::Disabled,
            true => VBATSEL_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATSEL_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATSEL_A::Enabled
    }
}
///Field `VBATSEL` writer - VBAT selection
pub type VBATSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATSEL_A, O>;
impl<'a, const O: u8> VBATSEL_W<'a, O> {
    ///V_BAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATSEL_A::Disabled)
    }
    ///V_BAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATSEL_A::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - VTS selection
    #[inline(always)]
    pub fn vsensesel(&self) -> VSENSESEL_R {
        VSENSESEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    pub fn vbatsel(&self) -> VBATSEL_R {
        VBATSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<13> {
        DMACFG_W::new(self)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<14> {
        MDMA_W::new(self)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<16> {
        CKMODE_W::new(self)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    ///Bit 23 - VTS selection
    #[inline(always)]
    #[must_use]
    pub fn vsensesel(&mut self) -> VSENSESEL_W<23> {
        VSENSESEL_W::new(self)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    #[must_use]
    pub fn vbatsel(&mut self) -> VBATSEL_W<24> {
        VBATSEL_W::new(self)
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
