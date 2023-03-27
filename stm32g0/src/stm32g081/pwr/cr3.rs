///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1
pub type EWUP1_R = crate::BitReader<bool>;
///Field `EWUP1` writer - Enable Wakeup pin WKUP1
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `EWUP2` reader - Enable Wakeup pin WKUP2
pub type EWUP2_R = crate::BitReader<bool>;
///Field `EWUP2` writer - Enable Wakeup pin WKUP2
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `EWUP4` reader - Enable Wakeup pin WKUP4
pub type EWUP4_R = crate::BitReader<bool>;
///Field `EWUP4` writer - Enable Wakeup pin WKUP4
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `EWUP5` reader - Enable WKUP5 wakeup pin
pub type EWUP5_R = crate::BitReader<bool>;
///Field `EWUP5` writer - Enable WKUP5 wakeup pin
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `EWUP6` reader - Enable WKUP6 wakeup pin
pub type EWUP6_R = crate::BitReader<bool>;
///Field `EWUP6` writer - Enable WKUP6 wakeup pin
pub type EWUP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `RRS` reader - SRAM retention in Standby mode
pub type RRS_R = crate::BitReader<bool>;
///Field `RRS` writer - SRAM retention in Standby mode
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `ULPEN` reader - Enable the periodical sampling mode for PDR detection
pub type ULPEN_R = crate::BitReader<bool>;
///Field `ULPEN` writer - Enable the periodical sampling mode for PDR detection
pub type ULPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `APC` reader - Apply pull-up and pull-down configuration
pub type APC_R = crate::BitReader<bool>;
///Field `APC` writer - Apply pull-up and pull-down configuration
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `EIWUL` reader - Enable internal wakeup line
pub type EIWUL_R = crate::BitReader<bool>;
///Field `EIWUL` writer - Enable internal wakeup line
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable WKUP5 wakeup pin
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable WKUP6 wakeup pin
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - SRAM retention in Standby mode
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable the periodical sampling mode for PDR detection
    #[inline(always)]
    pub fn ulpen(&self) -> ULPEN_R {
        ULPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<3> {
        EWUP4_W::new(self)
    }
    ///Bit 4 - Enable WKUP5 wakeup pin
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<4> {
        EWUP5_W::new(self)
    }
    ///Bit 5 - Enable WKUP6 wakeup pin
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> EWUP6_W<5> {
        EWUP6_W::new(self)
    }
    ///Bit 8 - SRAM retention in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<8> {
        RRS_W::new(self)
    }
    ///Bit 9 - Enable the periodical sampling mode for PDR detection
    #[inline(always)]
    #[must_use]
    pub fn ulpen(&mut self) -> ULPEN_W<9> {
        ULPEN_W::new(self)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<10> {
        APC_W::new(self)
    }
    ///Bit 15 - Enable internal wakeup line
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<15> {
        EIWUL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR3 to value 0x8000
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
