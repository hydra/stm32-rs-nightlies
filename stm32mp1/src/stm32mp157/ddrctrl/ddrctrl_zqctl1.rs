///Register `DDRCTRL_ZQCTL1` reader
pub struct R(crate::R<DDRCTRL_ZQCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_ZQCTL1` writer
pub struct W(crate::W<DDRCTRL_ZQCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL1_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `T_ZQ_SHORT_INTERVAL_X1024` reader - T_ZQ_SHORT_INTERVAL_X1024
pub type T_ZQ_SHORT_INTERVAL_X1024_R = crate::FieldReader<u32, u32>;
///Field `T_ZQ_SHORT_INTERVAL_X1024` writer - T_ZQ_SHORT_INTERVAL_X1024
pub type T_ZQ_SHORT_INTERVAL_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ZQCTL1_SPEC, u32, u32, 20, O>;
///Field `T_ZQ_RESET_NOP` reader - T_ZQ_RESET_NOP
pub type T_ZQ_RESET_NOP_R = crate::FieldReader<u16, u16>;
///Field `T_ZQ_RESET_NOP` writer - T_ZQ_RESET_NOP
pub type T_ZQ_RESET_NOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ZQCTL1_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&self) -> T_ZQ_SHORT_INTERVAL_X1024_R {
        T_ZQ_SHORT_INTERVAL_X1024_R::new(self.bits & 0x000f_ffff)
    }
    ///Bits 20:29 - T_ZQ_RESET_NOP
    #[inline(always)]
    pub fn t_zq_reset_nop(&self) -> T_ZQ_RESET_NOP_R {
        T_ZQ_RESET_NOP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024
    #[inline(always)]
    #[must_use]
    pub fn t_zq_short_interval_x1024(&mut self) -> T_ZQ_SHORT_INTERVAL_X1024_W<0> {
        T_ZQ_SHORT_INTERVAL_X1024_W::new(self)
    }
    ///Bits 20:29 - T_ZQ_RESET_NOP
    #[inline(always)]
    #[must_use]
    pub fn t_zq_reset_nop(&mut self) -> T_ZQ_RESET_NOP_W<20> {
        T_ZQ_RESET_NOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL ZQ control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_zqctl1](index.html) module
pub struct DDRCTRL_ZQCTL1_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_zqctl1::R](R) reader structure
impl crate::Readable for DDRCTRL_ZQCTL1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_zqctl1::W](W) writer structure
impl crate::Writable for DDRCTRL_ZQCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_ZQCTL1 to value 0x0200_0100
impl crate::Resettable for DDRCTRL_ZQCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0100;
}
