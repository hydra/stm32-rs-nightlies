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
pub type LATENCY_R = crate::FieldReader<u8, LATENCY_A>;
///Latency
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY_A {
    ///0: Zero wait state, if 0 &lt; SYSCLK≤ 24 MHz
    Ws0 = 0,
    ///1: One wait state, if 24 MHz &lt; SYSCLK ≤ 48 MHz
    Ws1 = 1,
    ///2: Two wait states, if 48 MHz &lt; SYSCLK ≤ 72 MHz
    Ws2 = 2,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LATENCY_A> {
        match self.bits {
            0 => Some(LATENCY_A::Ws0),
            1 => Some(LATENCY_A::Ws1),
            2 => Some(LATENCY_A::Ws2),
            _ => None,
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
    ///Checks if the value of the field is `Ws2`
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::Ws2
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, LATENCY_A, 3, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    ///Zero wait state, if 0 &lt; SYSCLK≤ 24 MHz
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    ///One wait state, if 24 MHz &lt; SYSCLK ≤ 48 MHz
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
    ///Two wait states, if 48 MHz &lt; SYSCLK ≤ 72 MHz
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws2)
    }
}
///Field `HLFCYA` reader - Flash half cycle access enable
pub type HLFCYA_R = crate::BitReader<bool>;
///Field `HLFCYA` writer - Flash half cycle access enable
pub type HLFCYA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `PRFTBE` reader - Prefetch buffer enable
pub type PRFTBE_R = crate::BitReader<bool>;
///Field `PRFTBE` writer - Prefetch buffer enable
pub type PRFTBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `PRFTBS` reader - Prefetch buffer status
pub type PRFTBS_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Flash half cycle access enable
    #[inline(always)]
    pub fn hlfcya(&self) -> HLFCYA_R {
        HLFCYA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Prefetch buffer enable
    #[inline(always)]
    pub fn prftbe(&self) -> PRFTBE_R {
        PRFTBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Prefetch buffer status
    #[inline(always)]
    pub fn prftbs(&self) -> PRFTBS_R {
        PRFTBS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Latency
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    ///Bit 3 - Flash half cycle access enable
    #[inline(always)]
    #[must_use]
    pub fn hlfcya(&mut self) -> HLFCYA_W<3> {
        HLFCYA_W::new(self)
    }
    ///Bit 4 - Prefetch buffer enable
    #[inline(always)]
    #[must_use]
    pub fn prftbe(&mut self) -> PRFTBE_W<4> {
        PRFTBE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash access control register
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
///`reset()` method sets ACR to value 0x30
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
