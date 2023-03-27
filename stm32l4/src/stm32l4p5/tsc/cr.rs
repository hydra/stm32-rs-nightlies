///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSCE` reader - Touch sensing controller enable
pub type TSCE_R = crate::BitReader<TSCE_A>;
///Touch sensing controller enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCE_A {
    ///0: Touch sensing controller disabled
    Disabled = 0,
    ///1: Touch sensing controller enabled
    Enabled = 1,
}
impl From<TSCE_A> for bool {
    #[inline(always)]
    fn from(variant: TSCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSCE_A {
        match self.bits {
            false => TSCE_A::Disabled,
            true => TSCE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCE_A::Enabled
    }
}
///Field `TSCE` writer - Touch sensing controller enable
pub type TSCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TSCE_A, O>;
impl<'a, const O: u8> TSCE_W<'a, O> {
    ///Touch sensing controller disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSCE_A::Disabled)
    }
    ///Touch sensing controller enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSCE_A::Enabled)
    }
}
///Field `START` reader - Start a new acquisition
pub type START_R = crate::BitReader<START_A>;
///Start a new acquisition
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    ///0: Acquisition not started
    NoStarted = 0,
    ///1: Start a new acquisition
    Started = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NoStarted,
            true => START_A::Started,
        }
    }
    ///Checks if the value of the field is `NoStarted`
    #[inline(always)]
    pub fn is_no_started(&self) -> bool {
        *self == START_A::NoStarted
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == START_A::Started
    }
}
///Field `START` writer - Start a new acquisition
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    ///Acquisition not started
    #[inline(always)]
    pub fn no_started(self) -> &'a mut W {
        self.variant(START_A::NoStarted)
    }
    ///Start a new acquisition
    #[inline(always)]
    pub fn started(self) -> &'a mut W {
        self.variant(START_A::Started)
    }
}
///Field `AM` reader - Acquisition mode
pub type AM_R = crate::BitReader<AM_A>;
///Acquisition mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM_A {
    ///0: Normal acquisition mode (acquisition starts as soon as START bit is set)
    Normal = 0,
    ///1: Synchronized acquisition mode (acquisition starts if START bit is set and when the selected signal is detected on the SYNC input pin)
    Synchronized = 1,
}
impl From<AM_A> for bool {
    #[inline(always)]
    fn from(variant: AM_A) -> Self {
        variant as u8 != 0
    }
}
impl AM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AM_A {
        match self.bits {
            false => AM_A::Normal,
            true => AM_A::Synchronized,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == AM_A::Normal
    }
    ///Checks if the value of the field is `Synchronized`
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        *self == AM_A::Synchronized
    }
}
///Field `AM` writer - Acquisition mode
pub type AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, AM_A, O>;
impl<'a, const O: u8> AM_W<'a, O> {
    ///Normal acquisition mode (acquisition starts as soon as START bit is set)
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(AM_A::Normal)
    }
    ///Synchronized acquisition mode (acquisition starts if START bit is set and when the selected signal is detected on the SYNC input pin)
    #[inline(always)]
    pub fn synchronized(self) -> &'a mut W {
        self.variant(AM_A::Synchronized)
    }
}
///Field `SYNCPOL` reader - Synchronization pin polarity
pub type SYNCPOL_R = crate::BitReader<SYNCPOL_A>;
///Synchronization pin polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCPOL_A {
    ///0: Falling edge only
    FallingEdge = 0,
    ///1: Rising edge and high level
    RisingEdge = 1,
}
impl From<SYNCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCPOL_A {
        match self.bits {
            false => SYNCPOL_A::FallingEdge,
            true => SYNCPOL_A::RisingEdge,
        }
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SYNCPOL_A::FallingEdge
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SYNCPOL_A::RisingEdge
    }
}
///Field `SYNCPOL` writer - Synchronization pin polarity
pub type SYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SYNCPOL_A, O>;
impl<'a, const O: u8> SYNCPOL_W<'a, O> {
    ///Falling edge only
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SYNCPOL_A::FallingEdge)
    }
    ///Rising edge and high level
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SYNCPOL_A::RisingEdge)
    }
}
///Field `IODEF` reader - I/O Default mode
pub type IODEF_R = crate::BitReader<IODEF_A>;
///I/O Default mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IODEF_A {
    ///0: I/Os are forced to output push-pull low
    PushPull = 0,
    ///1: I/Os are in input floating
    Floating = 1,
}
impl From<IODEF_A> for bool {
    #[inline(always)]
    fn from(variant: IODEF_A) -> Self {
        variant as u8 != 0
    }
}
impl IODEF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IODEF_A {
        match self.bits {
            false => IODEF_A::PushPull,
            true => IODEF_A::Floating,
        }
    }
    ///Checks if the value of the field is `PushPull`
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == IODEF_A::PushPull
    }
    ///Checks if the value of the field is `Floating`
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == IODEF_A::Floating
    }
}
///Field `IODEF` writer - I/O Default mode
pub type IODEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, IODEF_A, O>;
impl<'a, const O: u8> IODEF_W<'a, O> {
    ///I/Os are forced to output push-pull low
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(IODEF_A::PushPull)
    }
    ///I/Os are in input floating
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(IODEF_A::Floating)
    }
}
///Field `MCV` reader - Max count value
pub type MCV_R = crate::FieldReader<u8, u8>;
///Field `MCV` writer - Max count value
pub type MCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `PGPSC` reader - pulse generator prescaler
pub type PGPSC_R = crate::FieldReader<u8, u8>;
///Field `PGPSC` writer - pulse generator prescaler
pub type PGPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `SSPSC` reader - Spread spectrum prescaler
pub type SSPSC_R = crate::BitReader<bool>;
///Field `SSPSC` writer - Spread spectrum prescaler
pub type SSPSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SSE` reader - Spread spectrum enable
pub type SSE_R = crate::BitReader<SSE_A>;
///Spread spectrum enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSE_A {
    ///0: Spread spectrum disabled
    Disabled = 0,
    ///1: Spread spectrum enabled
    Enabled = 1,
}
impl From<SSE_A> for bool {
    #[inline(always)]
    fn from(variant: SSE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSE_A {
        match self.bits {
            false => SSE_A::Disabled,
            true => SSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSE_A::Enabled
    }
}
///Field `SSE` writer - Spread spectrum enable
pub type SSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SSE_A, O>;
impl<'a, const O: u8> SSE_W<'a, O> {
    ///Spread spectrum disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSE_A::Disabled)
    }
    ///Spread spectrum enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSE_A::Enabled)
    }
}
///Field `SSD` reader - Spread spectrum deviation
pub type SSD_R = crate::FieldReader<u8, u8>;
///Field `SSD` writer - Spread spectrum deviation
pub type SSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
///Field `CTPL` reader - Charge transfer pulse low
pub type CTPL_R = crate::FieldReader<u8, u8>;
///Field `CTPL` writer - Charge transfer pulse low
pub type CTPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `CTPH` reader - Charge transfer pulse high
pub type CTPH_R = crate::FieldReader<u8, u8>;
///Field `CTPH` writer - Charge transfer pulse high
pub type CTPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Touch sensing controller enable
    #[inline(always)]
    #[must_use]
    pub fn tsce(&mut self) -> TSCE_W<0> {
        TSCE_W::new(self)
    }
    ///Bit 1 - Start a new acquisition
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    ///Bit 2 - Acquisition mode
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<2> {
        AM_W::new(self)
    }
    ///Bit 3 - Synchronization pin polarity
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SYNCPOL_W<3> {
        SYNCPOL_W::new(self)
    }
    ///Bit 4 - I/O Default mode
    #[inline(always)]
    #[must_use]
    pub fn iodef(&mut self) -> IODEF_W<4> {
        IODEF_W::new(self)
    }
    ///Bits 5:7 - Max count value
    #[inline(always)]
    #[must_use]
    pub fn mcv(&mut self) -> MCV_W<5> {
        MCV_W::new(self)
    }
    ///Bits 12:14 - pulse generator prescaler
    #[inline(always)]
    #[must_use]
    pub fn pgpsc(&mut self) -> PGPSC_W<12> {
        PGPSC_W::new(self)
    }
    ///Bit 15 - Spread spectrum prescaler
    #[inline(always)]
    #[must_use]
    pub fn sspsc(&mut self) -> SSPSC_W<15> {
        SSPSC_W::new(self)
    }
    ///Bit 16 - Spread spectrum enable
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SSE_W<16> {
        SSE_W::new(self)
    }
    ///Bits 17:23 - Spread spectrum deviation
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SSD_W<17> {
        SSD_W::new(self)
    }
    ///Bits 24:27 - Charge transfer pulse low
    #[inline(always)]
    #[must_use]
    pub fn ctpl(&mut self) -> CTPL_W<24> {
        CTPL_W::new(self)
    }
    ///Bits 28:31 - Charge transfer pulse high
    #[inline(always)]
    #[must_use]
    pub fn ctph(&mut self) -> CTPH_W<28> {
        CTPH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
