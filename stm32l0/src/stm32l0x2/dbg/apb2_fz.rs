///Register `APB2_FZ` reader
pub struct R(crate::R<APB2_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2_FZ_SPEC>) -> Self {
        R(reader)
    }
}
///Register `APB2_FZ` writer
pub struct W(crate::W<APB2_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2_FZ_SPEC>;
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
impl From<crate::W<APB2_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2_FZ_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_TIMER21_STOP` reader - Debug Timer 21 stopped when Core is halted
pub type DBG_TIMER21_STOP_R = crate::BitReader<DBG_TIMER21_STOP_A>;
///Debug Timer 21 stopped when Core is halted
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIMER21_STOP_A {
    ///0: The counter clock of TIMx is fed even if the core is halted
    Continue = 0,
    ///1: The counter clock of TIMx is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_TIMER21_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIMER21_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_TIMER21_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_TIMER21_STOP_A {
        match self.bits {
            false => DBG_TIMER21_STOP_A::Continue,
            true => DBG_TIMER21_STOP_A::Stop,
        }
    }
    ///Checks if the value of the field is `Continue`
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIMER21_STOP_A::Continue
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIMER21_STOP_A::Stop
    }
}
///Field `DBG_TIMER21_STOP` writer - Debug Timer 21 stopped when Core is halted
pub type DBG_TIMER21_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APB2_FZ_SPEC, DBG_TIMER21_STOP_A, O>;
impl<'a, const O: u8> DBG_TIMER21_STOP_W<'a, O> {
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::Continue)
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(DBG_TIMER21_STOP_A::Stop)
    }
}
///Field `DBG_TIMER22_STO` reader - Debug Timer 22 stopped when Core is halted
pub type DBG_TIMER22_STO_R = crate::BitReader<bool>;
///Field `DBG_TIMER22_STO` writer - Debug Timer 22 stopped when Core is halted
pub type DBG_TIMER22_STO_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2_FZ_SPEC, bool, O>;
impl R {
    ///Bit 2 - Debug Timer 21 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer21_stop(&self) -> DBG_TIMER21_STOP_R {
        DBG_TIMER21_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Debug Timer 22 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer22_sto(&self) -> DBG_TIMER22_STO_R {
        DBG_TIMER22_STO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Debug Timer 21 stopped when Core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer21_stop(&mut self) -> DBG_TIMER21_STOP_W<2> {
        DBG_TIMER21_STOP_W::new(self)
    }
    ///Bit 6 - Debug Timer 22 stopped when Core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer22_sto(&mut self) -> DBG_TIMER22_STO_W<6> {
        DBG_TIMER22_STO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB High Freeze Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [apb2_fz](index.html) module
pub struct APB2_FZ_SPEC;
impl crate::RegisterSpec for APB2_FZ_SPEC {
    type Ux = u32;
}
///`read()` method returns [apb2_fz::R](R) reader structure
impl crate::Readable for APB2_FZ_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [apb2_fz::W](W) writer structure
impl crate::Writable for APB2_FZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets APB2_FZ to value 0
impl crate::Resettable for APB2_FZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
