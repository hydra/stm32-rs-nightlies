///Register `DDRCTRL_PCFGR_1` reader
pub struct R(crate::R<DDRCTRL_PCFGR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PCFGR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PCFGR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PCFGR_1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_PCFGR_1` writer
pub struct W(crate::W<DDRCTRL_PCFGR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PCFGR_1_SPEC>;
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
impl From<crate::W<DDRCTRL_PCFGR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PCFGR_1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RD_PORT_PRIORITY` reader - RD_PORT_PRIORITY
pub type RD_PORT_PRIORITY_R = crate::FieldReader<u16, u16>;
///Field `RD_PORT_PRIORITY` writer - RD_PORT_PRIORITY
pub type RD_PORT_PRIORITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_PCFGR_1_SPEC, u16, u16, 10, O>;
///Field `RD_PORT_AGING_EN` reader - RD_PORT_AGING_EN
pub type RD_PORT_AGING_EN_R = crate::BitReader<bool>;
///Field `RD_PORT_AGING_EN` writer - RD_PORT_AGING_EN
pub type RD_PORT_AGING_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PCFGR_1_SPEC, bool, O>;
///Field `RD_PORT_URGENT_EN` reader - RD_PORT_URGENT_EN
pub type RD_PORT_URGENT_EN_R = crate::BitReader<bool>;
///Field `RD_PORT_URGENT_EN` writer - RD_PORT_URGENT_EN
pub type RD_PORT_URGENT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PCFGR_1_SPEC, bool, O>;
///Field `RD_PORT_PAGEMATCH_EN` reader - RD_PORT_PAGEMATCH_EN
pub type RD_PORT_PAGEMATCH_EN_R = crate::BitReader<bool>;
///Field `RD_PORT_PAGEMATCH_EN` writer - RD_PORT_PAGEMATCH_EN
pub type RD_PORT_PAGEMATCH_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PCFGR_1_SPEC, bool, O>;
///Field `RDWR_ORDERED_EN` reader - RDWR_ORDERED_EN
pub type RDWR_ORDERED_EN_R = crate::BitReader<bool>;
///Field `RDWR_ORDERED_EN` writer - RDWR_ORDERED_EN
pub type RDWR_ORDERED_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PCFGR_1_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - RD_PORT_PRIORITY
    #[inline(always)]
    pub fn rd_port_priority(&self) -> RD_PORT_PRIORITY_R {
        RD_PORT_PRIORITY_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 12 - RD_PORT_AGING_EN
    #[inline(always)]
    pub fn rd_port_aging_en(&self) -> RD_PORT_AGING_EN_R {
        RD_PORT_AGING_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RD_PORT_URGENT_EN
    #[inline(always)]
    pub fn rd_port_urgent_en(&self) -> RD_PORT_URGENT_EN_R {
        RD_PORT_URGENT_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RD_PORT_PAGEMATCH_EN
    #[inline(always)]
    pub fn rd_port_pagematch_en(&self) -> RD_PORT_PAGEMATCH_EN_R {
        RD_PORT_PAGEMATCH_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - RDWR_ORDERED_EN
    #[inline(always)]
    pub fn rdwr_ordered_en(&self) -> RDWR_ORDERED_EN_R {
        RDWR_ORDERED_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - RD_PORT_PRIORITY
    #[inline(always)]
    #[must_use]
    pub fn rd_port_priority(&mut self) -> RD_PORT_PRIORITY_W<0> {
        RD_PORT_PRIORITY_W::new(self)
    }
    ///Bit 12 - RD_PORT_AGING_EN
    #[inline(always)]
    #[must_use]
    pub fn rd_port_aging_en(&mut self) -> RD_PORT_AGING_EN_W<12> {
        RD_PORT_AGING_EN_W::new(self)
    }
    ///Bit 13 - RD_PORT_URGENT_EN
    #[inline(always)]
    #[must_use]
    pub fn rd_port_urgent_en(&mut self) -> RD_PORT_URGENT_EN_W<13> {
        RD_PORT_URGENT_EN_W::new(self)
    }
    ///Bit 14 - RD_PORT_PAGEMATCH_EN
    #[inline(always)]
    #[must_use]
    pub fn rd_port_pagematch_en(&mut self) -> RD_PORT_PAGEMATCH_EN_W<14> {
        RD_PORT_PAGEMATCH_EN_W::new(self)
    }
    ///Bit 16 - RDWR_ORDERED_EN
    #[inline(always)]
    #[must_use]
    pub fn rdwr_ordered_en(&mut self) -> RDWR_ORDERED_EN_W<16> {
        RDWR_ORDERED_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL port 1 configuration read register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_pcfgr_1](index.html) module
pub struct DDRCTRL_PCFGR_1_SPEC;
impl crate::RegisterSpec for DDRCTRL_PCFGR_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_pcfgr_1::R](R) reader structure
impl crate::Readable for DDRCTRL_PCFGR_1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_pcfgr_1::W](W) writer structure
impl crate::Writable for DDRCTRL_PCFGR_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_PCFGR_1 to value 0x4000
impl crate::Resettable for DDRCTRL_PCFGR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000;
}
