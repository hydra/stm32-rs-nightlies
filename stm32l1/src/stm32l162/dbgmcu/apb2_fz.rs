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
///Field `DBG_TIM9_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM9_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM9_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM9_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2_FZ_SPEC, bool, O>;
///Field `DBG_TIM10_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM10_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM10_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM10_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2_FZ_SPEC, bool, O>;
///Field `DBG_TIM11_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM11_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM11_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM11_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2_FZ_SPEC, bool, O>;
impl R {
    ///Bit 2 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - TIM counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<2> {
        DBG_TIM9_STOP_W::new(self)
    }
    ///Bit 3 - TIM counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<3> {
        DBG_TIM10_STOP_W::new(self)
    }
    ///Bit 4 - TIM counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<4> {
        DBG_TIM11_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU APB1 freeze register 2
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
