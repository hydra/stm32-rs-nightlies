///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWIF` reader - Early Wakeup Interrupt
pub type EWIF_R = crate::BitReader<EWIFR_A>;
///Early Wakeup Interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR_A {
    ///0: The EWI Interrupt Service Routine has been serviced
    Finished = 0,
    ///1: The EWI Interrupt Service Routine has been triggered
    Pending = 1,
}
impl From<EWIFR_A> for bool {
    #[inline(always)]
    fn from(variant: EWIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl EWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWIFR_A {
        match self.bits {
            false => EWIFR_A::Finished,
            true => EWIFR_A::Pending,
        }
    }
    ///Checks if the value of the field is `Finished`
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIFR_A::Finished
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR_A::Pending
    }
}
///Early Wakeup Interrupt
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFW_AW {
    ///0: The EWI Interrupt Service Routine has been serviced
    Finished = 0,
}
impl From<EWIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: EWIFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIF` writer - Early Wakeup Interrupt
pub type EWIF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, SR_SPEC, EWIFW_AW, O>;
impl<'a, const O: u8> EWIF_W<'a, O> {
    ///The EWI Interrupt Service Routine has been serviced
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(EWIFW_AW::Finished)
    }
}
impl R {
    ///Bit 0 - Early Wakeup Interrupt
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Early Wakeup Interrupt
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<0> {
        EWIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register (WWDG_SR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
