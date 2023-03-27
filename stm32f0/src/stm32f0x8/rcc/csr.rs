///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSION` reader - Internal low speed oscillator enable
pub type LSION_R = crate::BitReader<LSION_A>;
///Internal low speed oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION_A {
    ///0: LSI oscillator Off
    Off = 0,
    ///1: LSI oscillator On
    On = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::Off,
            true => LSION_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION_A::On
    }
}
///Field `LSION` writer - Internal low speed oscillator enable
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSION_A, O>;
impl<'a, const O: u8> LSION_W<'a, O> {
    ///LSI oscillator Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::Off)
    }
    ///LSI oscillator On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::On)
    }
}
///Field `LSIRDY` reader - Internal low speed oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDYR_A>;
///Internal low speed oscillator ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR_A {
    ///0: LSI oscillator not ready
    NotReady = 0,
    ///1: LSI oscillator ready
    Ready = 1,
}
impl From<LSIRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYR_A {
        match self.bits {
            false => LSIRDYR_A::NotReady,
            true => LSIRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR_A::Ready
    }
}
///Field `V18PWRRSTF` reader - 1.8 V domain reset flag
pub type V18PWRRSTF_R = crate::BitReader<V18PWRRSTFR_A>;
///1.8 V domain reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V18PWRRSTFR_A {
    ///0: No reset has occured
    NoReset = 0,
    ///1: A reset has occured
    Reset = 1,
}
impl From<V18PWRRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: V18PWRRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl V18PWRRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> V18PWRRSTFR_A {
        match self.bits {
            false => V18PWRRSTFR_A::NoReset,
            true => V18PWRRSTFR_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == V18PWRRSTFR_A::NoReset
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == V18PWRRSTFR_A::Reset
    }
}
///Field `V18PWRRSTF` writer - 1.8 V domain reset flag
pub type V18PWRRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, V18PWRRSTFR_A, O>;
impl<'a, const O: u8> V18PWRRSTF_W<'a, O> {
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(V18PWRRSTFR_A::NoReset)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(V18PWRRSTFR_A::Reset)
    }
}
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<RMVFW_A>;
///Remove reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW_A {
    ///1: Clears the reset flag
    Clear = 1,
}
impl From<RMVFW_A> for bool {
    #[inline(always)]
    fn from(variant: RMVFW_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RMVFW_A> {
        match self.bits {
            true => Some(RMVFW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW_A::Clear
    }
}
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RMVFW_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    ///Clears the reset flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVFW_A::Clear)
    }
}
///Field `OBLRSTF` reader - Option byte loader reset flag
pub use V18PWRRSTF_R as OBLRSTF_R;
///Field `PINRSTF` reader - PIN reset flag
pub use V18PWRRSTF_R as PINRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag
pub use V18PWRRSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use V18PWRRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent watchdog reset flag
pub use V18PWRRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use V18PWRRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use V18PWRRSTF_R as LPWRRSTF_R;
///Field `OBLRSTF` writer - Option byte loader reset flag
pub use V18PWRRSTF_W as OBLRSTF_W;
///Field `PINRSTF` writer - PIN reset flag
pub use V18PWRRSTF_W as PINRSTF_W;
///Field `PORRSTF` writer - POR/PDR reset flag
pub use V18PWRRSTF_W as PORRSTF_W;
///Field `SFTRSTF` writer - Software reset flag
pub use V18PWRRSTF_W as SFTRSTF_W;
///Field `IWDGRSTF` writer - Independent watchdog reset flag
pub use V18PWRRSTF_W as IWDGRSTF_W;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub use V18PWRRSTF_W as WWDGRSTF_W;
///Field `LPWRRSTF` writer - Low-power reset flag
pub use V18PWRRSTF_W as LPWRRSTF_W;
impl R {
    ///Bit 0 - Internal low speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal low speed oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - 1.8 V domain reset flag
    #[inline(always)]
    pub fn v18pwrrstf(&self) -> V18PWRRSTF_R {
        V18PWRRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Internal low speed oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    ///Bit 23 - 1.8 V domain reset flag
    #[inline(always)]
    #[must_use]
    pub fn v18pwrrstf(&mut self) -> V18PWRRSTF_W<23> {
        V18PWRRSTF_W::new(self)
    }
    ///Bit 24 - Remove reset flag
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<24> {
        RMVF_W::new(self)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    #[must_use]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<25> {
        OBLRSTF_W::new(self)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<26> {
        PINRSTF_W::new(self)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<27> {
        PORRSTF_W::new(self)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<29> {
        IWDGRSTF_W::new(self)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<31> {
        LPWRRSTF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control/status register (RCC_CSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
