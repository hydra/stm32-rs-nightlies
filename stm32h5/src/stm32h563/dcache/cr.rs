///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - enable
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CACHEINV` writer - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect.
pub type CACHEINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CACHECMD` reader - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
pub type CACHECMD_R = crate::FieldReader<u8, u8>;
///Field `CACHECMD` writer - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
pub type CACHECMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `STARTCMD` writer - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect.
pub type STARTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RHITMEN` reader - read-hit monitor enable
pub type RHITMEN_R = crate::BitReader<bool>;
///Field `RHITMEN` writer - read-hit monitor enable
pub type RHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RMISSMEN` reader - read-miss monitor enable
pub type RMISSMEN_R = crate::BitReader<bool>;
///Field `RMISSMEN` writer - read-miss monitor enable
pub type RMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RHITMRST` reader - read-hit monitor reset
pub type RHITMRST_R = crate::BitReader<bool>;
///Field `RHITMRST` writer - read-hit monitor reset
pub type RHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RMISSMRST` reader - read-miss monitor reset
pub type RMISSMRST_R = crate::BitReader<bool>;
///Field `RMISSMRST` writer - read-miss monitor reset
pub type RMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WHITMEN` reader - write-hit monitor enable
pub type WHITMEN_R = crate::BitReader<bool>;
///Field `WHITMEN` writer - write-hit monitor enable
pub type WHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WMISSMEN` reader - write-miss monitor enable
pub type WMISSMEN_R = crate::BitReader<bool>;
///Field `WMISSMEN` writer - write-miss monitor enable
pub type WMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WHITMRST` reader - write-hit monitor reset
pub type WHITMRST_R = crate::BitReader<bool>;
///Field `WHITMRST` writer - write-hit monitor reset
pub type WHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WMISSMRST` reader - write-miss monitor reset
pub type WMISSMRST_R = crate::BitReader<bool>;
///Field `WMISSMRST` writer - write-miss monitor reset
pub type WMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `HBURST` reader - output burst type for cache master port read accesses Write access is always done in INCR burst type.
pub type HBURST_R = crate::BitReader<bool>;
///Field `HBURST` writer - output burst type for cache master port read accesses Write access is always done in INCR burst type.
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - read-hit monitor reset
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - read-miss monitor reset
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write-hit monitor reset
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - write-miss monitor reset
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type.
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<1> {
        CACHEINV_W::new(self)
    }
    ///Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
    #[inline(always)]
    #[must_use]
    pub fn cachecmd(&mut self) -> CACHECMD_W<8> {
        CACHECMD_W::new(self)
    }
    ///Bit 11 - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect.
    #[inline(always)]
    #[must_use]
    pub fn startcmd(&mut self) -> STARTCMD_W<11> {
        STARTCMD_W::new(self)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    #[must_use]
    pub fn rhitmen(&mut self) -> RHITMEN_W<16> {
        RHITMEN_W::new(self)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    #[must_use]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<17> {
        RMISSMEN_W::new(self)
    }
    ///Bit 18 - read-hit monitor reset
    #[inline(always)]
    #[must_use]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<18> {
        RHITMRST_W::new(self)
    }
    ///Bit 19 - read-miss monitor reset
    #[inline(always)]
    #[must_use]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<19> {
        RMISSMRST_W::new(self)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    #[must_use]
    pub fn whitmen(&mut self) -> WHITMEN_W<20> {
        WHITMEN_W::new(self)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    #[must_use]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<21> {
        WMISSMEN_W::new(self)
    }
    ///Bit 22 - write-hit monitor reset
    #[inline(always)]
    #[must_use]
    pub fn whitmrst(&mut self) -> WHITMRST_W<22> {
        WHITMRST_W::new(self)
    }
    ///Bit 23 - write-miss monitor reset
    #[inline(always)]
    #[must_use]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<23> {
        WMISSMRST_W::new(self)
    }
    ///Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type.
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCACHE control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
