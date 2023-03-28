///Register `SECCFGR2` reader
pub struct R(crate::R<SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECCFGR2` writer
pub struct W(crate::W<SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR2_SPEC>;
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
impl From<crate::W<SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEC32` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC32_R = crate::BitReader<bool>;
///Field `SEC32` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC33` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC33_R = crate::BitReader<bool>;
///Field `SEC33` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC33_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC34` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC34_R = crate::BitReader<bool>;
///Field `SEC34` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC34_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC35` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC35_R = crate::BitReader<bool>;
///Field `SEC35` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC36` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC36_R = crate::BitReader<bool>;
///Field `SEC36` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC36_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC37` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC37_R = crate::BitReader<bool>;
///Field `SEC37` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC37_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC38` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC38_R = crate::BitReader<bool>;
///Field `SEC38` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC38_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC39` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC39_R = crate::BitReader<bool>;
///Field `SEC39` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC39_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC40` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC40_R = crate::BitReader<bool>;
///Field `SEC40` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC40_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC41` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC41_R = crate::BitReader<bool>;
///Field `SEC41` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC41_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC42` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC42_R = crate::BitReader<bool>;
///Field `SEC42` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC42_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC43` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC43_R = crate::BitReader<bool>;
///Field `SEC43` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC43_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC44` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC44_R = crate::BitReader<bool>;
///Field `SEC44` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC44_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC45` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC45_R = crate::BitReader<bool>;
///Field `SEC45` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC45_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC46` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC46_R = crate::BitReader<bool>;
///Field `SEC46` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC46_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC47` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC47_R = crate::BitReader<bool>;
///Field `SEC47` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC47_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC48` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC48_R = crate::BitReader<bool>;
///Field `SEC48` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC48_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC49` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC49_R = crate::BitReader<bool>;
///Field `SEC49` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC49_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC50` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC50_R = crate::BitReader<bool>;
///Field `SEC50` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC50_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC51` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC51_R = crate::BitReader<bool>;
///Field `SEC51` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC51_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC52` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC52_R = crate::BitReader<bool>;
///Field `SEC52` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC52_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC53` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC53_R = crate::BitReader<bool>;
///Field `SEC53` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC53_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC54` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC54_R = crate::BitReader<bool>;
///Field `SEC54` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC54_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC55` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC55_R = crate::BitReader<bool>;
///Field `SEC55` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC55_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC56` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC56_R = crate::BitReader<bool>;
///Field `SEC56` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC56_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
///Field `SEC57` reader - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC57_R = crate::BitReader<bool>;
///Field `SEC57` writer - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
pub type SEC57_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec32(&self) -> SEC32_R {
        SEC32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec33(&self) -> SEC33_R {
        SEC33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec34(&self) -> SEC34_R {
        SEC34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec35(&self) -> SEC35_R {
        SEC35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec36(&self) -> SEC36_R {
        SEC36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec37(&self) -> SEC37_R {
        SEC37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec38(&self) -> SEC38_R {
        SEC38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec39(&self) -> SEC39_R {
        SEC39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec40(&self) -> SEC40_R {
        SEC40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec41(&self) -> SEC41_R {
        SEC41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec42(&self) -> SEC42_R {
        SEC42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec43(&self) -> SEC43_R {
        SEC43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec44(&self) -> SEC44_R {
        SEC44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec45(&self) -> SEC45_R {
        SEC45_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec46(&self) -> SEC46_R {
        SEC46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec47(&self) -> SEC47_R {
        SEC47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec48(&self) -> SEC48_R {
        SEC48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec49(&self) -> SEC49_R {
        SEC49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec50(&self) -> SEC50_R {
        SEC50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec51(&self) -> SEC51_R {
        SEC51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec52(&self) -> SEC52_R {
        SEC52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec53(&self) -> SEC53_R {
        SEC53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec54(&self) -> SEC54_R {
        SEC54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec55(&self) -> SEC55_R {
        SEC55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec56(&self) -> SEC56_R {
        SEC56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    pub fn sec57(&self) -> SEC57_R {
        SEC57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec32(&mut self) -> SEC32_W<0> {
        SEC32_W::new(self)
    }
    ///Bit 1 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec33(&mut self) -> SEC33_W<1> {
        SEC33_W::new(self)
    }
    ///Bit 2 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec34(&mut self) -> SEC34_W<2> {
        SEC34_W::new(self)
    }
    ///Bit 3 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec35(&mut self) -> SEC35_W<3> {
        SEC35_W::new(self)
    }
    ///Bit 4 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec36(&mut self) -> SEC36_W<4> {
        SEC36_W::new(self)
    }
    ///Bit 5 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec37(&mut self) -> SEC37_W<5> {
        SEC37_W::new(self)
    }
    ///Bit 6 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec38(&mut self) -> SEC38_W<6> {
        SEC38_W::new(self)
    }
    ///Bit 7 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec39(&mut self) -> SEC39_W<7> {
        SEC39_W::new(self)
    }
    ///Bit 8 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec40(&mut self) -> SEC40_W<8> {
        SEC40_W::new(self)
    }
    ///Bit 9 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec41(&mut self) -> SEC41_W<9> {
        SEC41_W::new(self)
    }
    ///Bit 10 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec42(&mut self) -> SEC42_W<10> {
        SEC42_W::new(self)
    }
    ///Bit 11 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec43(&mut self) -> SEC43_W<11> {
        SEC43_W::new(self)
    }
    ///Bit 12 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec44(&mut self) -> SEC44_W<12> {
        SEC44_W::new(self)
    }
    ///Bit 13 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec45(&mut self) -> SEC45_W<13> {
        SEC45_W::new(self)
    }
    ///Bit 14 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec46(&mut self) -> SEC46_W<14> {
        SEC46_W::new(self)
    }
    ///Bit 15 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec47(&mut self) -> SEC47_W<15> {
        SEC47_W::new(self)
    }
    ///Bit 16 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec48(&mut self) -> SEC48_W<16> {
        SEC48_W::new(self)
    }
    ///Bit 17 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec49(&mut self) -> SEC49_W<17> {
        SEC49_W::new(self)
    }
    ///Bit 18 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec50(&mut self) -> SEC50_W<18> {
        SEC50_W::new(self)
    }
    ///Bit 19 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec51(&mut self) -> SEC51_W<19> {
        SEC51_W::new(self)
    }
    ///Bit 20 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec52(&mut self) -> SEC52_W<20> {
        SEC52_W::new(self)
    }
    ///Bit 21 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec53(&mut self) -> SEC53_W<21> {
        SEC53_W::new(self)
    }
    ///Bit 22 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec54(&mut self) -> SEC54_W<22> {
        SEC54_W::new(self)
    }
    ///Bit 23 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec55(&mut self) -> SEC55_W<23> {
        SEC55_W::new(self)
    }
    ///Bit 24 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec56(&mut self) -> SEC56_W<24> {
        SEC56_W::new(self)
    }
    ///Bit 25 - Security enable on event input x When EXTI_PRIVCFGR.PRIVx is disabled, SECx can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIVx is enabled, SECx can only be written with privileged access. Unprivileged write to this SECx is discarded.
    #[inline(always)]
    #[must_use]
    pub fn sec57(&mut self) -> SEC57_W<25> {
        SEC57_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI security configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [seccfgr2](index.html) module
pub struct SECCFGR2_SPEC;
impl crate::RegisterSpec for SECCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [seccfgr2::R](R) reader structure
impl crate::Readable for SECCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [seccfgr2::W](W) writer structure
impl crate::Writable for SECCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECCFGR2 to value 0
impl crate::Resettable for SECCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
