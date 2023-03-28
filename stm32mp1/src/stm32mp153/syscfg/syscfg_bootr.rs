///Register `SYSCFG_BOOTR` reader
pub struct R(crate::R<SYSCFG_BOOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_BOOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_BOOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_BOOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SYSCFG_BOOTR` writer
pub struct W(crate::W<SYSCFG_BOOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_BOOTR_SPEC>;
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
impl From<crate::W<SYSCFG_BOOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_BOOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BOOT0` reader - BOOT0
pub type BOOT0_R = crate::BitReader<bool>;
///Field `BOOT1` reader - BOOT1
pub type BOOT1_R = crate::BitReader<bool>;
///Field `BOOT2` reader - BOOT2
pub type BOOT2_R = crate::BitReader<bool>;
///Field `BOOT0_PD` reader - BOOT0_PD
pub type BOOT0_PD_R = crate::BitReader<bool>;
///Field `BOOT0_PD` writer - BOOT0_PD
pub type BOOT0_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_BOOTR_SPEC, bool, O>;
///Field `BOOT1_PD` reader - BOOT1_PD
pub type BOOT1_PD_R = crate::BitReader<bool>;
///Field `BOOT1_PD` writer - BOOT1_PD
pub type BOOT1_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_BOOTR_SPEC, bool, O>;
///Field `BOOT2_PD` reader - BOOT2_PD
pub type BOOT2_PD_R = crate::BitReader<bool>;
///Field `BOOT2_PD` writer - BOOT2_PD
pub type BOOT2_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCFG_BOOTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - BOOT0
    #[inline(always)]
    pub fn boot0(&self) -> BOOT0_R {
        BOOT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BOOT1
    #[inline(always)]
    pub fn boot1(&self) -> BOOT1_R {
        BOOT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BOOT2
    #[inline(always)]
    pub fn boot2(&self) -> BOOT2_R {
        BOOT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - BOOT0_PD
    #[inline(always)]
    pub fn boot0_pd(&self) -> BOOT0_PD_R {
        BOOT0_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - BOOT1_PD
    #[inline(always)]
    pub fn boot1_pd(&self) -> BOOT1_PD_R {
        BOOT1_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BOOT2_PD
    #[inline(always)]
    pub fn boot2_pd(&self) -> BOOT2_PD_R {
        BOOT2_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - BOOT0_PD
    #[inline(always)]
    #[must_use]
    pub fn boot0_pd(&mut self) -> BOOT0_PD_W<4> {
        BOOT0_PD_W::new(self)
    }
    ///Bit 5 - BOOT1_PD
    #[inline(always)]
    #[must_use]
    pub fn boot1_pd(&mut self) -> BOOT1_PD_W<5> {
        BOOT1_PD_W::new(self)
    }
    ///Bit 6 - BOOT2_PD
    #[inline(always)]
    #[must_use]
    pub fn boot2_pd(&mut self) -> BOOT2_PD_W<6> {
        BOOT2_PD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [syscfg_bootr](index.html) module
pub struct SYSCFG_BOOTR_SPEC;
impl crate::RegisterSpec for SYSCFG_BOOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [syscfg_bootr::R](R) reader structure
impl crate::Readable for SYSCFG_BOOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [syscfg_bootr::W](W) writer structure
impl crate::Writable for SYSCFG_BOOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SYSCFG_BOOTR to value 0
impl crate::Resettable for SYSCFG_BOOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
