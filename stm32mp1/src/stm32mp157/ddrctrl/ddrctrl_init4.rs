///Register `DDRCTRL_INIT4` reader
pub struct R(crate::R<DDRCTRL_INIT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_INIT4` writer
pub struct W(crate::W<DDRCTRL_INIT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT4_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EMR3` reader - EMR3
pub type EMR3_R = crate::FieldReader<u16, u16>;
///Field `EMR3` writer - EMR3
pub type EMR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_INIT4_SPEC, u16, u16, 16, O>;
///Field `EMR2` reader - EMR2
pub type EMR2_R = crate::FieldReader<u16, u16>;
///Field `EMR2` writer - EMR2
pub type EMR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRCTRL_INIT4_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - EMR3
    #[inline(always)]
    pub fn emr3(&self) -> EMR3_R {
        EMR3_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EMR2
    #[inline(always)]
    pub fn emr2(&self) -> EMR2_R {
        EMR2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - EMR3
    #[inline(always)]
    #[must_use]
    pub fn emr3(&mut self) -> EMR3_W<0> {
        EMR3_W::new(self)
    }
    ///Bits 16:31 - EMR2
    #[inline(always)]
    #[must_use]
    pub fn emr2(&mut self) -> EMR2_W<16> {
        EMR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL SDRAM initialization register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_init4](index.html) module
pub struct DDRCTRL_INIT4_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_init4::R](R) reader structure
impl crate::Readable for DDRCTRL_INIT4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_init4::W](W) writer structure
impl crate::Writable for DDRCTRL_INIT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_INIT4 to value 0
impl crate::Resettable for DDRCTRL_INIT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
