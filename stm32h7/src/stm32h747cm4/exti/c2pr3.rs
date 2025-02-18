///Register `C2PR3` reader
pub struct R(crate::R<C2PR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2PR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2PR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2PR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2PR3` writer
pub struct W(crate::W<C2PR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2PR3_SPEC>;
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
impl From<crate::W<C2PR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2PR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PR82` reader - CPU2 configurable event inputs x+64 Pending bit
pub type PR82_R = crate::BitReader<PR82R_A>;
///CPU2 configurable event inputs x+64 Pending bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82R_A {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PR82R_A> for bool {
    #[inline(always)]
    fn from(variant: PR82R_A) -> Self {
        variant as u8 != 0
    }
}
impl PR82_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PR82R_A {
        match self.bits {
            false => PR82R_A::NotPending,
            true => PR82R_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR82R_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR82R_A::Pending
    }
}
///CPU2 configurable event inputs x+64 Pending bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PR82W_AW {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PR82W_AW> for bool {
    #[inline(always)]
    fn from(variant: PR82W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PR82` writer - CPU2 configurable event inputs x+64 Pending bit
pub type PR82_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, C2PR3_SPEC, PR82W_AW, O>;
impl<'a, const O: u8> PR82_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR82W_AW::Clear)
    }
}
///Field `PR84` reader - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_R as PR84_R;
///Field `PR85` reader - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_R as PR85_R;
///Field `PR86` reader - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_R as PR86_R;
///Field `PR84` writer - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_W as PR84_W;
///Field `PR85` writer - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_W as PR85_W;
///Field `PR86` writer - CPU2 configurable event inputs x+64 Pending bit
pub use PR82_W as PR86_W;
impl R {
    ///Bit 18 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr82(&self) -> PR82_R {
        PR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr84(&self) -> PR84_R {
        PR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr85(&self) -> PR85_R {
        PR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    pub fn pr86(&self) -> PR86_R {
        PR86_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 18 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pr82(&mut self) -> PR82_W<18> {
        PR82_W::new(self)
    }
    ///Bit 20 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pr84(&mut self) -> PR84_W<20> {
        PR84_W::new(self)
    }
    ///Bit 21 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pr85(&mut self) -> PR85_W<21> {
        PR85_W::new(self)
    }
    ///Bit 22 - CPU2 configurable event inputs x+64 Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pr86(&mut self) -> PR86_W<22> {
        PR86_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 EXTI pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2pr3](index.html) module
pub struct C2PR3_SPEC;
impl crate::RegisterSpec for C2PR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2pr3::R](R) reader structure
impl crate::Readable for C2PR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2pr3::W](W) writer structure
impl crate::Writable for C2PR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0074_0000;
}
///`reset()` method sets C2PR3 to value 0
impl crate::Resettable for C2PR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
