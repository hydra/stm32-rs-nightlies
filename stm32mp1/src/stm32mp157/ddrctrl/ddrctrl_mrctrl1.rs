///Register `DDRCTRL_MRCTRL1` reader
pub struct R(crate::R<DDRCTRL_MRCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_MRCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_MRCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_MRCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_MRCTRL1` writer
pub struct W(crate::W<DDRCTRL_MRCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_MRCTRL1_SPEC>;
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
impl From<crate::W<DDRCTRL_MRCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_MRCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR_DATA` reader - MR_DATA
pub type MR_DATA_R = crate::FieldReader<u16, u16>;
///Field `MR_DATA` writer - MR_DATA
pub type MR_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_MRCTRL1_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - MR_DATA
    #[inline(always)]
    pub fn mr_data(&self) -> MR_DATA_R {
        MR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - MR_DATA
    #[inline(always)]
    #[must_use]
    pub fn mr_data(&mut self) -> MR_DATA_W<0> {
        MR_DATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL mode register read/write control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_mrctrl1](index.html) module
pub struct DDRCTRL_MRCTRL1_SPEC;
impl crate::RegisterSpec for DDRCTRL_MRCTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_mrctrl1::R](R) reader structure
impl crate::Readable for DDRCTRL_MRCTRL1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_mrctrl1::W](W) writer structure
impl crate::Writable for DDRCTRL_MRCTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_MRCTRL1 to value 0
impl crate::Resettable for DDRCTRL_MRCTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
