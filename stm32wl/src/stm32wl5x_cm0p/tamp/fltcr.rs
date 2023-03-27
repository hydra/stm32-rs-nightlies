///Register `FLTCR` reader
pub struct R(crate::R<FLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTCR` writer
pub struct W(crate::W<FLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCR_SPEC>;
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
impl From<crate::W<FLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TAMPFREQ` reader - TAMPFREQ
pub type TAMPFREQ_R = crate::FieldReader<u8, TAMPFREQ_A>;
///TAMPFREQ
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    Hz1 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    Hz2 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    Hz4 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    Hz8 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    Hz16 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    Hz32 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    Hz64 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    Hz128 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
impl TAMPFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::Hz1,
            1 => TAMPFREQ_A::Hz2,
            2 => TAMPFREQ_A::Hz4,
            3 => TAMPFREQ_A::Hz8,
            4 => TAMPFREQ_A::Hz16,
            5 => TAMPFREQ_A::Hz32,
            6 => TAMPFREQ_A::Hz64,
            7 => TAMPFREQ_A::Hz128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Hz1`
    #[inline(always)]
    pub fn is_hz_1(&self) -> bool {
        *self == TAMPFREQ_A::Hz1
    }
    ///Checks if the value of the field is `Hz2`
    #[inline(always)]
    pub fn is_hz_2(&self) -> bool {
        *self == TAMPFREQ_A::Hz2
    }
    ///Checks if the value of the field is `Hz4`
    #[inline(always)]
    pub fn is_hz_4(&self) -> bool {
        *self == TAMPFREQ_A::Hz4
    }
    ///Checks if the value of the field is `Hz8`
    #[inline(always)]
    pub fn is_hz_8(&self) -> bool {
        *self == TAMPFREQ_A::Hz8
    }
    ///Checks if the value of the field is `Hz16`
    #[inline(always)]
    pub fn is_hz_16(&self) -> bool {
        *self == TAMPFREQ_A::Hz16
    }
    ///Checks if the value of the field is `Hz32`
    #[inline(always)]
    pub fn is_hz_32(&self) -> bool {
        *self == TAMPFREQ_A::Hz32
    }
    ///Checks if the value of the field is `Hz64`
    #[inline(always)]
    pub fn is_hz_64(&self) -> bool {
        *self == TAMPFREQ_A::Hz64
    }
    ///Checks if the value of the field is `Hz128`
    #[inline(always)]
    pub fn is_hz_128(&self) -> bool {
        *self == TAMPFREQ_A::Hz128
    }
}
///Field `TAMPFREQ` writer - TAMPFREQ
pub type TAMPFREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTCR_SPEC, u8, TAMPFREQ_A, 3, O>;
impl<'a, const O: u8> TAMPFREQ_W<'a, O> {
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_1(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz1)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_2(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz2)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_4(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz4)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_8(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz8)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_16(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz16)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_32(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz32)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_64(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz64)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn hz_128(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Hz128)
    }
}
///Field `TAMPFLT` reader - TAMPFLT
pub type TAMPFLT_R = crate::FieldReader<u8, TAMPFLT_A>;
///TAMPFLT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    ///0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    NoFilter = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level"
    Filter2 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level"
    Filter4 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level"
    Filter8 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
impl TAMPFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::NoFilter,
            1 => TAMPFLT_A::Filter2,
            2 => TAMPFLT_A::Filter4,
            3 => TAMPFLT_A::Filter8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == TAMPFLT_A::NoFilter
    }
    ///Checks if the value of the field is `Filter2`
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == TAMPFLT_A::Filter2
    }
    ///Checks if the value of the field is `Filter4`
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == TAMPFLT_A::Filter4
    }
    ///Checks if the value of the field is `Filter8`
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == TAMPFLT_A::Filter8
    }
}
///Field `TAMPFLT` writer - TAMPFLT
pub type TAMPFLT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTCR_SPEC, u8, TAMPFLT_A, 2, O>;
impl<'a, const O: u8> TAMPFLT_W<'a, O> {
    ///Tamper event is activated on edge of TAMP_INx input transitions to the active level (no internal pull-up on TAMP_INx input)"
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(TAMPFLT_A::NoFilter)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Filter2)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Filter4)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level"
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Filter8)
    }
}
///Field `TAMPPRCH` reader - TAMPPRCH
pub type TAMPPRCH_R = crate::FieldReader<u8, TAMPPRCH_A>;
///TAMPPRCH
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    ///0: 1 RTCCLK cycle
    Cycles1 = 0,
    ///1: 2 RTCCLK cycles
    Cycles2 = 1,
    ///2: 4 RTCCLK cycles
    Cycles4 = 2,
    ///3: 8 RTCCLK cycles
    Cycles8 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
impl TAMPPRCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::Cycles1,
            1 => TAMPPRCH_A::Cycles2,
            2 => TAMPPRCH_A::Cycles4,
            3 => TAMPPRCH_A::Cycles8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Cycles1`
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH_A::Cycles1
    }
    ///Checks if the value of the field is `Cycles2`
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH_A::Cycles2
    }
    ///Checks if the value of the field is `Cycles4`
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH_A::Cycles4
    }
    ///Checks if the value of the field is `Cycles8`
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH_A::Cycles8
    }
}
///Field `TAMPPRCH` writer - TAMPPRCH
pub type TAMPPRCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLTCR_SPEC, u8, TAMPPRCH_A, 2, O>;
impl<'a, const O: u8> TAMPPRCH_W<'a, O> {
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles1)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles2)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles4)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles8)
    }
}
///Field `TAMPPUDIS` reader - TAMPPUDIS
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS_A>;
///TAMPPUDIS
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS_A {
    ///0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
    Enabled = 0,
    ///1: Disable precharge of TAMP_INx pins
    Disabled = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TAMPPUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::Enabled,
            true => TAMPPUDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS_A::Disabled
    }
}
///Field `TAMPPUDIS` writer - TAMPPUDIS
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTCR_SPEC, TAMPPUDIS_A, O>;
impl<'a, const O: u8> TAMPPUDIS_W<'a, O> {
    ///Precharge TAMP_INx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Enabled)
    }
    ///Disable precharge of TAMP_INx pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Disabled)
    }
}
impl R {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - TAMPFREQ
    #[inline(always)]
    #[must_use]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<0> {
        TAMPFREQ_W::new(self)
    }
    ///Bits 3:4 - TAMPFLT
    #[inline(always)]
    #[must_use]
    pub fn tampflt(&mut self) -> TAMPFLT_W<3> {
        TAMPFLT_W::new(self)
    }
    ///Bits 5:6 - TAMPPRCH
    #[inline(always)]
    #[must_use]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<5> {
        TAMPPRCH_W::new(self)
    }
    ///Bit 7 - TAMPPUDIS
    #[inline(always)]
    #[must_use]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<7> {
        TAMPPUDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fltcr](index.html) module
pub struct FLTCR_SPEC;
impl crate::RegisterSpec for FLTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fltcr::R](R) reader structure
impl crate::Readable for FLTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fltcr::W](W) writer structure
impl crate::Writable for FLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLTCR to value 0
impl crate::Resettable for FLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
