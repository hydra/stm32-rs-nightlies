///Register `BRR` reader
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BRR` writer
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BR3` reader - Port Reset bit
pub type BR3_R = crate::BitReader<BR3W_A>;
///Port Reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W_A {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BR3W_A> for bool {
    #[inline(always)]
    fn from(variant: BR3W_A) -> Self {
        variant as u8 != 0
    }
}
impl BR3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR3W_A {
        match self.bits {
            false => BR3W_A::NoAction,
            true => BR3W_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoAction`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR3W_A::NoAction
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR3W_A::Reset
    }
}
///Field `BR3` writer - Port Reset bit
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, BR3W_A, O>;
impl<'a, const O: u8> BR3_W<'a, O> {
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR3W_A::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR3W_A::Reset)
    }
}
impl R {
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port bit reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](index.html) module
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [brr::R](R) reader structure
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [brr::W](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
