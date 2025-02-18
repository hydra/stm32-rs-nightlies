///Register `C1ICR` reader
pub struct R(crate::R<C1ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1ICR` writer
pub struct W(crate::W<C1ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1ICR_SPEC>;
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
impl From<crate::W<C1ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISC0` reader - Interrupt(N) semaphore n clear bit
pub type ISC0_R = crate::BitReader<ISC0R_A>;
///Interrupt(N) semaphore n clear bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0R_A {
    ///0: Always reads 0
    NoEffect = 0,
}
impl From<ISC0R_A> for bool {
    #[inline(always)]
    fn from(variant: ISC0R_A) -> Self {
        variant as u8 != 0
    }
}
impl ISC0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ISC0R_A> {
        match self.bits {
            false => Some(ISC0R_A::NoEffect),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ISC0R_A::NoEffect
    }
}
///Interrupt(N) semaphore n clear bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISC0W_AW {
    ///0: Interrupt semaphore x status ISFx and masked status MISFx not affected
    NoEffect = 0,
    ///1: Interrupt semaphore x status ISFx and masked status MISFx cleared
    Clear = 1,
}
impl From<ISC0W_AW> for bool {
    #[inline(always)]
    fn from(variant: ISC0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ISC0` writer - Interrupt(N) semaphore n clear bit
pub type ISC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1ICR_SPEC, ISC0W_AW, O>;
impl<'a, const O: u8> ISC0_W<'a, O> {
    ///Interrupt semaphore x status ISFx and masked status MISFx not affected
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ISC0W_AW::NoEffect)
    }
    ///Interrupt semaphore x status ISFx and masked status MISFx cleared
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ISC0W_AW::Clear)
    }
}
///Field `ISC1` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC1_R;
///Field `ISC2` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC2_R;
///Field `ISC3` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC3_R;
///Field `ISC4` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC4_R;
///Field `ISC5` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC5_R;
///Field `ISC6` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC6_R;
///Field `ISC7` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC7_R;
///Field `ISC8` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC8_R;
///Field `ISC9` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC9_R;
///Field `ISC10` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC10_R;
///Field `ISC11` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC11_R;
///Field `ISC12` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC12_R;
///Field `ISC13` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC13_R;
///Field `ISC14` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC14_R;
///Field `ISC15` reader - Interrupt(N) semaphore n clear bit
pub use ISC0_R as ISC15_R;
///Field `ISC1` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC1_W;
///Field `ISC2` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC2_W;
///Field `ISC3` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC3_W;
///Field `ISC4` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC4_W;
///Field `ISC5` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC5_W;
///Field `ISC6` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC6_W;
///Field `ISC7` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC7_W;
///Field `ISC8` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC8_W;
///Field `ISC9` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC9_W;
///Field `ISC10` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC10_W;
///Field `ISC11` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC11_W;
///Field `ISC12` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC12_W;
///Field `ISC13` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC13_W;
///Field `ISC14` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC14_W;
///Field `ISC15` writer - Interrupt(N) semaphore n clear bit
pub use ISC0_W as ISC15_W;
impl R {
    ///Bit 0 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc0(&self) -> ISC0_R {
        ISC0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc1(&self) -> ISC1_R {
        ISC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc2(&self) -> ISC2_R {
        ISC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc3(&self) -> ISC3_R {
        ISC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc4(&self) -> ISC4_R {
        ISC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc5(&self) -> ISC5_R {
        ISC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc6(&self) -> ISC6_R {
        ISC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc7(&self) -> ISC7_R {
        ISC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc8(&self) -> ISC8_R {
        ISC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc9(&self) -> ISC9_R {
        ISC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc10(&self) -> ISC10_R {
        ISC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc11(&self) -> ISC11_R {
        ISC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc12(&self) -> ISC12_R {
        ISC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc13(&self) -> ISC13_R {
        ISC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc14(&self) -> ISC14_R {
        ISC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    pub fn isc15(&self) -> ISC15_R {
        ISC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc0(&mut self) -> ISC0_W<0> {
        ISC0_W::new(self)
    }
    ///Bit 1 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc1(&mut self) -> ISC1_W<1> {
        ISC1_W::new(self)
    }
    ///Bit 2 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc2(&mut self) -> ISC2_W<2> {
        ISC2_W::new(self)
    }
    ///Bit 3 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc3(&mut self) -> ISC3_W<3> {
        ISC3_W::new(self)
    }
    ///Bit 4 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc4(&mut self) -> ISC4_W<4> {
        ISC4_W::new(self)
    }
    ///Bit 5 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc5(&mut self) -> ISC5_W<5> {
        ISC5_W::new(self)
    }
    ///Bit 6 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc6(&mut self) -> ISC6_W<6> {
        ISC6_W::new(self)
    }
    ///Bit 7 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc7(&mut self) -> ISC7_W<7> {
        ISC7_W::new(self)
    }
    ///Bit 8 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc8(&mut self) -> ISC8_W<8> {
        ISC8_W::new(self)
    }
    ///Bit 9 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc9(&mut self) -> ISC9_W<9> {
        ISC9_W::new(self)
    }
    ///Bit 10 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc10(&mut self) -> ISC10_W<10> {
        ISC10_W::new(self)
    }
    ///Bit 11 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc11(&mut self) -> ISC11_W<11> {
        ISC11_W::new(self)
    }
    ///Bit 12 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc12(&mut self) -> ISC12_W<12> {
        ISC12_W::new(self)
    }
    ///Bit 13 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc13(&mut self) -> ISC13_W<13> {
        ISC13_W::new(self)
    }
    ///Bit 14 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc14(&mut self) -> ISC14_W<14> {
        ISC14_W::new(self)
    }
    ///Bit 15 - Interrupt(N) semaphore n clear bit
    #[inline(always)]
    #[must_use]
    pub fn isc15(&mut self) -> ISC15_W<15> {
        ISC15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1icr](index.html) module
pub struct C1ICR_SPEC;
impl crate::RegisterSpec for C1ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1icr::R](R) reader structure
impl crate::Readable for C1ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1icr::W](W) writer structure
impl crate::Writable for C1ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1ICR to value 0
impl crate::Resettable for C1ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
