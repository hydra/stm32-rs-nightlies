///Register `DDRCTRL_DBG1` reader
pub struct R(crate::R<DDRCTRL_DBG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DBG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DBG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DBG1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_DBG1` writer
pub struct W(crate::W<DDRCTRL_DBG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DBG1_SPEC>;
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
impl From<crate::W<DDRCTRL_DBG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DBG1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIS_DQ` reader - DIS_DQ
pub type DIS_DQ_R = crate::BitReader<bool>;
///Field `DIS_DQ` writer - DIS_DQ
pub type DIS_DQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DBG1_SPEC, bool, O>;
///Field `DIS_HIF` reader - DIS_HIF
pub type DIS_HIF_R = crate::BitReader<bool>;
///Field `DIS_HIF` writer - DIS_HIF
pub type DIS_HIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_DBG1_SPEC, bool, O>;
impl R {
    ///Bit 0 - DIS_DQ
    #[inline(always)]
    pub fn dis_dq(&self) -> DIS_DQ_R {
        DIS_DQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DIS_HIF
    #[inline(always)]
    pub fn dis_hif(&self) -> DIS_HIF_R {
        DIS_HIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DIS_DQ
    #[inline(always)]
    #[must_use]
    pub fn dis_dq(&mut self) -> DIS_DQ_W<0> {
        DIS_DQ_W::new(self)
    }
    ///Bit 1 - DIS_HIF
    #[inline(always)]
    #[must_use]
    pub fn dis_hif(&mut self) -> DIS_HIF_W<1> {
        DIS_HIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL debug register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_dbg1](index.html) module
pub struct DDRCTRL_DBG1_SPEC;
impl crate::RegisterSpec for DDRCTRL_DBG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_dbg1::R](R) reader structure
impl crate::Readable for DDRCTRL_DBG1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_dbg1::W](W) writer structure
impl crate::Writable for DDRCTRL_DBG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_DBG1 to value 0
impl crate::Resettable for DDRCTRL_DBG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
