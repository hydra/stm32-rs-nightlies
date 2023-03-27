///Register `D3PMR3` reader
pub struct R(crate::R<D3PMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PMR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D3PMR3` writer
pub struct W(crate::W<D3PMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PMR3_SPEC>;
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
impl From<crate::W<D3PMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PMR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR88` reader - D3 Pending Mask on Event input x+64
pub type MR88_R = crate::BitReader<MR88_A>;
///D3 Pending Mask on Event input x+64
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR88_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<MR88_A> for bool {
    #[inline(always)]
    fn from(variant: MR88_A) -> Self {
        variant as u8 != 0
    }
}
impl MR88_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MR88_A {
        match self.bits {
            false => MR88_A::Masked,
            true => MR88_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR88_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR88_A::Unmasked
    }
}
///Field `MR88` writer - D3 Pending Mask on Event input x+64
pub type MR88_W<'a, const O: u8> = crate::BitWriter<'a, u32, D3PMR3_SPEC, MR88_A, O>;
impl<'a, const O: u8> MR88_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR88_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR88_A::Unmasked)
    }
}
impl R {
    ///Bit 24 - D3 Pending Mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - D3 Pending Mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<24> {
        MR88_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI D3 pending mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d3pmr3](index.html) module
pub struct D3PMR3_SPEC;
impl crate::RegisterSpec for D3PMR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [d3pmr3::R](R) reader structure
impl crate::Readable for D3PMR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d3pmr3::W](W) writer structure
impl crate::Writable for D3PMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D3PMR3 to value 0
impl crate::Resettable for D3PMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
