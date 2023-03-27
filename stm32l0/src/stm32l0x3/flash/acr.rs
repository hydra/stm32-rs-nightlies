///Register `ACR` reader
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ACR` writer
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::BitReader<LATENCY_A>;
///Latency
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATENCY_A {
    ///0: Zero wait state is used to read a word in the NVM
    Ws0 = 0,
    ///1: One wait state is used to read a word in the NVM
    Ws1 = 1,
}
impl From<LATENCY_A> for bool {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as u8 != 0
    }
}
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            false => LATENCY_A::Ws0,
            true => LATENCY_A::Ws1,
        }
    }
    ///Checks if the value of the field is `Ws0`
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::Ws0
    }
    ///Checks if the value of the field is `Ws1`
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::Ws1
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, LATENCY_A, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    ///Zero wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    ///One wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
///Prefetch enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN_A {
    ///0: Prefetch is disabled
    Disabled = 0,
    ///1: Prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::Disabled,
            true => PRFTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::Enabled
    }
}
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRFTEN_A, O>;
impl<'a, const O: u8> PRFTEN_W<'a, O> {
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Disabled)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Enabled)
    }
}
///Field `SLEEP_PD` reader - Flash mode during Sleep
pub type SLEEP_PD_R = crate::BitReader<SLEEP_PD_A>;
///Flash mode during Sleep
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_PD_A {
    ///0: When the device is in Sleep mode, the NVM is in Idle mode
    NvmidleMode = 0,
    ///1: When the device is in Sleep mode, the NVM is in power-down mode
    NvmpwrDownMode = 1,
}
impl From<SLEEP_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_PD_A {
        match self.bits {
            false => SLEEP_PD_A::NvmidleMode,
            true => SLEEP_PD_A::NvmpwrDownMode,
        }
    }
    ///Checks if the value of the field is `NvmidleMode`
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == SLEEP_PD_A::NvmidleMode
    }
    ///Checks if the value of the field is `NvmpwrDownMode`
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == SLEEP_PD_A::NvmpwrDownMode
    }
}
///Field `SLEEP_PD` writer - Flash mode during Sleep
pub type SLEEP_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, SLEEP_PD_A, O>;
impl<'a, const O: u8> SLEEP_PD_W<'a, O> {
    ///When the device is in Sleep mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::NvmidleMode)
    }
    ///When the device is in Sleep mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut W {
        self.variant(SLEEP_PD_A::NvmpwrDownMode)
    }
}
///Field `RUN_PD` reader - Flash mode during Run
pub type RUN_PD_R = crate::BitReader<RUN_PD_A>;
///Flash mode during Run
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_PD_A {
    ///0: When the device is in Run mode, the NVM is in Idle mode
    NvmidleMode = 0,
    ///1: When the device is in Run mode, the NVM is in power-down mode
    NvmpwrDownMode = 1,
}
impl From<RUN_PD_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD_A) -> Self {
        variant as u8 != 0
    }
}
impl RUN_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RUN_PD_A {
        match self.bits {
            false => RUN_PD_A::NvmidleMode,
            true => RUN_PD_A::NvmpwrDownMode,
        }
    }
    ///Checks if the value of the field is `NvmidleMode`
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == RUN_PD_A::NvmidleMode
    }
    ///Checks if the value of the field is `NvmpwrDownMode`
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == RUN_PD_A::NvmpwrDownMode
    }
}
///Field `RUN_PD` writer - Flash mode during Run
pub type RUN_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, RUN_PD_A, O>;
impl<'a, const O: u8> RUN_PD_W<'a, O> {
    ///When the device is in Run mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut W {
        self.variant(RUN_PD_A::NvmidleMode)
    }
    ///When the device is in Run mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut W {
        self.variant(RUN_PD_A::NvmpwrDownMode)
    }
}
///Field `DISAB_BUF` reader - Disable Buffer
pub type DISAB_BUF_R = crate::BitReader<DISAB_BUF_A>;
///Disable Buffer
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISAB_BUF_A {
    ///0: The buffers are enabled
    Enabled = 0,
    ///1: The buffers are disabled
    Disabled = 1,
}
impl From<DISAB_BUF_A> for bool {
    #[inline(always)]
    fn from(variant: DISAB_BUF_A) -> Self {
        variant as u8 != 0
    }
}
impl DISAB_BUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISAB_BUF_A {
        match self.bits {
            false => DISAB_BUF_A::Enabled,
            true => DISAB_BUF_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISAB_BUF_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISAB_BUF_A::Disabled
    }
}
///Field `DISAB_BUF` writer - Disable Buffer
pub type DISAB_BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, DISAB_BUF_A, O>;
impl<'a, const O: u8> DISAB_BUF_W<'a, O> {
    ///The buffers are enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISAB_BUF_A::Enabled)
    }
    ///The buffers are disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISAB_BUF_A::Disabled)
    }
}
///Field `PRE_READ` reader - Pre-read data address
pub type PRE_READ_R = crate::BitReader<PRE_READ_A>;
///Pre-read data address
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRE_READ_A {
    ///0: The pre-read is disabled
    Disabled = 0,
    ///1: The pre-read is enabled
    Enabled = 1,
}
impl From<PRE_READ_A> for bool {
    #[inline(always)]
    fn from(variant: PRE_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl PRE_READ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRE_READ_A {
        match self.bits {
            false => PRE_READ_A::Disabled,
            true => PRE_READ_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRE_READ_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRE_READ_A::Enabled
    }
}
///Field `PRE_READ` writer - Pre-read data address
pub type PRE_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRE_READ_A, O>;
impl<'a, const O: u8> PRE_READ_W<'a, O> {
    ///The pre-read is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRE_READ_A::Disabled)
    }
    ///The pre-read is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRE_READ_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Disable Buffer
    #[inline(always)]
    pub fn disab_buf(&self) -> DISAB_BUF_R {
        DISAB_BUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pre-read data address
    #[inline(always)]
    pub fn pre_read(&self) -> PRE_READ_R {
        PRE_READ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Latency
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<1> {
        PRFTEN_W::new(self)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<3> {
        SLEEP_PD_W::new(self)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<4> {
        RUN_PD_W::new(self)
    }
    ///Bit 5 - Disable Buffer
    #[inline(always)]
    #[must_use]
    pub fn disab_buf(&mut self) -> DISAB_BUF_W<5> {
        DISAB_BUF_W::new(self)
    }
    ///Bit 6 - Pre-read data address
    #[inline(always)]
    #[must_use]
    pub fn pre_read(&mut self) -> PRE_READ_W<6> {
        PRE_READ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr](index.html) module
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [acr::R](R) reader structure
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [acr::W](W) writer structure
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
