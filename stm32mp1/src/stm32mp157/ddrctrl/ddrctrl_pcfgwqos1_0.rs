///Register `DDRCTRL_PCFGWQOS1_0` reader
pub struct R(crate::R<DDRCTRL_PCFGWQOS1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGWQOS1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGWQOS1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGWQOS1_0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_PCFGWQOS1_0` writer
pub struct W(crate::W<DDRCTRL_PCFGWQOS1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGWQOS1_0_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGWQOS1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGWQOS1_0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WQOS_MAP_TIMEOUT1` reader - WQOS_MAP_TIMEOUT1
pub type WQOS_MAP_TIMEOUT1_R = crate::FieldReader<u16, u16>;
///Field `WQOS_MAP_TIMEOUT1` writer - WQOS_MAP_TIMEOUT1
pub type WQOS_MAP_TIMEOUT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS1_0_SPEC, u16, u16, 11, O>;
///Field `WQOS_MAP_TIMEOUT2` reader - WQOS_MAP_TIMEOUT2
pub type WQOS_MAP_TIMEOUT2_R = crate::FieldReader<u16, u16>;
///Field `WQOS_MAP_TIMEOUT2` writer - WQOS_MAP_TIMEOUT2
pub type WQOS_MAP_TIMEOUT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGWQOS1_0_SPEC, u16, u16, 11, O>;
impl R {
    ///Bits 0:10 - WQOS_MAP_TIMEOUT1
    #[inline(always)]
    pub fn wqos_map_timeout1(&self) -> WQOS_MAP_TIMEOUT1_R {
        WQOS_MAP_TIMEOUT1_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:26 - WQOS_MAP_TIMEOUT2
    #[inline(always)]
    pub fn wqos_map_timeout2(&self) -> WQOS_MAP_TIMEOUT2_R {
        WQOS_MAP_TIMEOUT2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    ///Bits 0:10 - WQOS_MAP_TIMEOUT1
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout1(&mut self) -> WQOS_MAP_TIMEOUT1_W<0> {
        WQOS_MAP_TIMEOUT1_W::new(self)
    }
    ///Bits 16:26 - WQOS_MAP_TIMEOUT2
    #[inline(always)]
    #[must_use]
    pub fn wqos_map_timeout2(&mut self) -> WQOS_MAP_TIMEOUT2_W<16> {
        WQOS_MAP_TIMEOUT2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL port 0 write Q0S configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_pcfgwqos1_0](index.html) module
pub struct DDRCTRL_PCFGWQOS1_0_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGWQOS1_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_pcfgwqos1_0::R](R) reader structure
impl crate::Readable for DDRCTRL_PCFGWQOS1_0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_pcfgwqos1_0::W](W) writer structure
impl crate::Writable for DDRCTRL_PCFGWQOS1_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_PCFGWQOS1_0 to value 0
impl crate::Resettable for DDRCTRL_PCFGWQOS1_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
