///Register `C2CR3` reader
pub struct R(crate::R<C2CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2CR3` writer
pub struct W(crate::W<C2CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR3_SPEC>;
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
impl From<crate::W<C2CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_R = crate::BitReader<bool>;
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_R = crate::BitReader<bool>;
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_R = crate::BitReader<bool>;
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EWUP4` reader - Enable Wakeup pin WKUP4 for CPU2
pub type EWUP4_R = crate::BitReader<bool>;
///Field `EWUP4` writer - Enable Wakeup pin WKUP4 for CPU2
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EWUP5` reader - Enable Wakeup pin WKUP5 for CPU2
pub type EWUP5_R = crate::BitReader<bool>;
///Field `EWUP5` writer - Enable Wakeup pin WKUP5 for CPU2
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EBLEWUP` reader - Enable BLE host wakeup interrupt for CPU2
pub type EBLEWUP_R = crate::BitReader<bool>;
///Field `EBLEWUP` writer - Enable BLE host wakeup interrupt for CPU2
pub type EBLEWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `E802WUP` reader - Enable 802.15.4 host wakeup interrupt for CPU2
pub type E802WUP_R = crate::BitReader<bool>;
///Field `E802WUP` writer - Enable 802.15.4 host wakeup interrupt for CPU2
pub type E802WUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `APC` reader - Apply pull-up and pull-down configuration for CPU2
pub type APC_R = crate::BitReader<bool>;
///Field `APC` writer - Apply pull-up and pull-down configuration for CPU2
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
///Field `EIWUL` reader - Enable internal wakeup line for CPU2
pub type EIWUL_R = crate::BitReader<bool>;
///Field `EIWUL` writer - Enable internal wakeup line for CPU2
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, bool, O>;
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 for CPU2
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 for CPU2
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - Enable BLE host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn eblewup(&self) -> EBLEWUP_R {
        EBLEWUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2
    #[inline(always)]
    pub fn e802wup(&self) -> E802WUP_R {
        E802WUP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<2> {
        EWUP3_W::new(self)
    }
    ///Bit 3 - Enable Wakeup pin WKUP4 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<3> {
        EWUP4_W::new(self)
    }
    ///Bit 4 - Enable Wakeup pin WKUP5 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<4> {
        EWUP5_W::new(self)
    }
    ///Bit 9 - Enable BLE host wakeup interrupt for CPU2
    #[inline(always)]
    #[must_use]
    pub fn eblewup(&mut self) -> EBLEWUP_W<9> {
        EBLEWUP_W::new(self)
    }
    ///Bit 10 - Enable 802.15.4 host wakeup interrupt for CPU2
    #[inline(always)]
    #[must_use]
    pub fn e802wup(&mut self) -> E802WUP_W<10> {
        E802WUP_W::new(self)
    }
    ///Bit 12 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<12> {
        APC_W::new(self)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
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
///CPU2 Power control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2cr3](index.html) module
pub struct C2CR3_SPEC;
impl crate::RegisterSpec for C2CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2cr3::R](R) reader structure
impl crate::Readable for C2CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2cr3::W](W) writer structure
impl crate::Writable for C2CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2CR3 to value 0x8000
impl crate::Resettable for C2CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
