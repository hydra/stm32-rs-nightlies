///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCPC` reader - Capture/compare preloaded control
pub type CCPC_R = crate::BitReader<bool>;
///Field `CCPC` writer - Capture/compare preloaded control
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `CCUS` reader - Capture/compare control update selection
pub type CCUS_R = crate::BitReader<bool>;
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader<CCDS_A>;
///Capture/compare DMA selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS_A {
    ///0: CCx DMA request sent when CCx event occurs
    OnCompare = 0,
    ///1: CCx DMA request sent when update event occurs
    OnUpdate = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::OnCompare,
            true => CCDS_A::OnUpdate,
        }
    }
    ///Checks if the value of the field is `OnCompare`
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS_A::OnCompare
    }
    ///Checks if the value of the field is `OnUpdate`
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS_A::OnUpdate
    }
}
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCDS_A, O>;
impl<'a, const O: u8> CCDS_W<'a, O> {
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::OnCompare)
    }
    ///CCx DMA request sent when update event occurs
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::OnUpdate)
    }
}
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<u8, MMS_A>;
///Master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    ///0: The UG bit from the TIMx_EGR register is used as trigger output
    Reset = 0,
    ///1: The counter enable signal, CNT_EN, is used as trigger output
    Enable = 1,
    ///2: The update event is selected as trigger output
    Update = 2,
    ///3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    ComparePulse = 3,
    ///4: OC1REF signal is used as trigger output
    CompareOc1 = 4,
    ///5: OC2REF signal is used as trigger output
    CompareOc2 = 5,
    ///6: OC3REF signal is used as trigger output
    CompareOc3 = 6,
    ///7: OC4REF signal is used as trigger output
    CompareOc4 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::Reset,
            1 => MMS_A::Enable,
            2 => MMS_A::Update,
            3 => MMS_A::ComparePulse,
            4 => MMS_A::CompareOc1,
            5 => MMS_A::CompareOc2,
            6 => MMS_A::CompareOc3,
            7 => MMS_A::CompareOc4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::Reset
    }
    ///Checks if the value of the field is `Enable`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::Enable
    }
    ///Checks if the value of the field is `Update`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::Update
    }
    ///Checks if the value of the field is `ComparePulse`
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS_A::ComparePulse
    }
    ///Checks if the value of the field is `CompareOc1`
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS_A::CompareOc1
    }
    ///Checks if the value of the field is `CompareOc2`
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS_A::CompareOc2
    }
    ///Checks if the value of the field is `CompareOc3`
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS_A::CompareOc3
    }
    ///Checks if the value of the field is `CompareOc4`
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS_A::CompareOc4
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, MMS_A, 3, O>;
impl<'a, const O: u8> MMS_W<'a, O> {
    ///The UG bit from the TIMx_EGR register is used as trigger output
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::Reset)
    }
    ///The counter enable signal, CNT_EN, is used as trigger output
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::Enable)
    }
    ///The update event is selected as trigger output
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::Update)
    }
    ///The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::ComparePulse)
    }
    ///OC1REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc1)
    }
    ///OC2REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc2)
    }
    ///OC3REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc3)
    }
    ///OC4REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc4)
    }
}
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader<TI1S_A>;
///TI1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S_A {
    ///0: The TIMx_CH1 pin is connected to TI1 input
    Normal = 0,
    ///1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    Xor = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::Normal,
            true => TI1S_A::Xor,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S_A::Normal
    }
    ///Checks if the value of the field is `Xor`
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S_A::Xor
    }
}
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TI1S_A, O>;
impl<'a, const O: u8> TI1S_W<'a, O> {
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI1S_A::Normal)
    }
    ///The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI1S_A::Xor)
    }
}
///Field `OIS1` reader - Output Idle state 1
pub type OIS1_R = crate::BitReader<bool>;
///Field `OIS1` writer - Output Idle state 1
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS1N` reader - Output Idle state 1
pub type OIS1N_R = crate::BitReader<bool>;
///Field `OIS1N` writer - Output Idle state 1
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS2` reader - Output Idle state 2
pub type OIS2_R = crate::BitReader<bool>;
///Field `OIS2` writer - Output Idle state 2
pub type OIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS2N` reader - Output Idle state 2
pub type OIS2N_R = crate::BitReader<bool>;
///Field `OIS2N` writer - Output Idle state 2
pub type OIS2N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS3` reader - Output Idle state 3
pub type OIS3_R = crate::BitReader<bool>;
///Field `OIS3` writer - Output Idle state 3
pub type OIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS3N` reader - Output Idle state 3
pub type OIS3N_R = crate::BitReader<bool>;
///Field `OIS3N` writer - Output Idle state 3
pub type OIS3N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS4` reader - Output Idle state 4
pub type OIS4_R = crate::BitReader<bool>;
///Field `OIS4` writer - Output Idle state 4
pub type OIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS5` reader - Output Idle state 5
pub type OIS5_R = crate::BitReader<bool>;
///Field `OIS5` writer - Output Idle state 5
pub type OIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `OIS6` reader - Output Idle state 6
pub type OIS6_R = crate::BitReader<bool>;
///Field `OIS6` writer - Output Idle state 6
pub type OIS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `MMS2` reader - Master mode selection 2
pub type MMS2_R = crate::FieldReader<u8, u8>;
///Field `MMS2` writer - Master mode selection 2
pub type MMS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Output Idle state 5
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output Idle state 6
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    ///Bit 8 - Output Idle state 1
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    ///Bit 9 - Output Idle state 1
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    ///Bit 10 - Output Idle state 2
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<10> {
        OIS2_W::new(self)
    }
    ///Bit 11 - Output Idle state 2
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<11> {
        OIS2N_W::new(self)
    }
    ///Bit 12 - Output Idle state 3
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<12> {
        OIS3_W::new(self)
    }
    ///Bit 13 - Output Idle state 3
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<13> {
        OIS3N_W::new(self)
    }
    ///Bit 14 - Output Idle state 4
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<14> {
        OIS4_W::new(self)
    }
    ///Bit 16 - Output Idle state 5
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<16> {
        OIS5_W::new(self)
    }
    ///Bit 18 - Output Idle state 6
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<18> {
        OIS6_W::new(self)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<20> {
        MMS2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
