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
///Field `LSION` reader - LSI oscillator enable
pub type LSION_R = crate::BitReader<LSION_A>;
///LSI oscillator enable
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
///Field `LSION` writer - LSI oscillator enable
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
///Field `LSIRDY` reader - LSI oscillator ready
pub type LSIRDY_R = crate::BitReader<LSIRDYR_A>;
///LSI oscillator ready
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
pub type OBLRSTF_R = crate::BitReader<OBLRSTFR_A>;
///Option byte loader reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBLRSTFR_A {
    ///0: No reset has occured
    NoReset = 0,
    ///1: A reset has occured
    Reset = 1,
}
impl From<OBLRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: OBLRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl OBLRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OBLRSTFR_A {
        match self.bits {
            false => OBLRSTFR_A::NoReset,
            true => OBLRSTFR_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == OBLRSTFR_A::NoReset
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OBLRSTFR_A::Reset
    }
}
///Field `PINRSTF` reader - Pad reset flag
pub use OBLRSTF_R as PINRSTF_R;
///Field `BORRSTF` reader - BOR flag
pub use OBLRSTF_R as BORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use OBLRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub use OBLRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use OBLRSTF_R as WWDGRSTF_R;
///Field `LPWRSTF` reader - Low-power reset flag
pub use OBLRSTF_R as LPWRSTF_R;
impl R {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pad reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
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
    pub fn lpwrstf(&self) -> LPWRSTF_R {
        LPWRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CSR
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
