///Register `DDRCTRL_PCTRL_1` reader
pub struct R(crate::R<DDRCTRL_PCTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_PCTRL_1` writer
pub struct W(crate::W<DDRCTRL_PCTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCTRL_1_SPEC>;
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
impl From<crate::W<DDRCTRL_PCTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PORT_EN` reader - PORT_EN
pub type PORT_EN_R = crate::BitReader<bool>;
///Field `PORT_EN` writer - PORT_EN
pub type PORT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PCTRL_1_SPEC, bool, O>;
impl R {
    ///Bit 0 - PORT_EN
    #[inline(always)]
    pub fn port_en(&self) -> PORT_EN_R {
        PORT_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PORT_EN
    #[inline(always)]
    #[must_use]
    pub fn port_en(&mut self) -> PORT_EN_W<0> {
        PORT_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL port 1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_pctrl_1](index.html) module
pub struct DDRCTRL_PCTRL_1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCTRL_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_pctrl_1::R](R) reader structure
impl crate::Readable for DDRCTRL_PCTRL_1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_pctrl_1::W](W) writer structure
impl crate::Writable for DDRCTRL_PCTRL_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_PCTRL_1 to value 0
impl crate::Resettable for DDRCTRL_PCTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
