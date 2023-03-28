///Register `OPTCR_` reader
pub struct R(crate::R<OPTCR__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR__SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTCR_` writer
pub struct W(crate::W<OPTCR__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR__SPEC>;
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
impl From<crate::W<OPTCR__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR__SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_R = crate::BitReader<bool>;
///Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
///Field `OPTSTART` reader - Option byte start change option configuration bit
pub type OPTSTART_R = crate::BitReader<bool>;
///Field `OPTSTART` writer - Option byte start change option configuration bit
pub type OPTSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
///Field `MER` reader - Flash mass erase enable bit
pub type MER_R = crate::BitReader<bool>;
///Field `MER` writer - Flash mass erase enable bit
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
///Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_R = crate::BitReader<bool>;
///Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit
pub type OPTCHANGEERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
///Field `SWAP_BANK` reader - Bank swapping configuration bit
pub type SWAP_BANK_R = crate::BitReader<bool>;
///Field `SWAP_BANK` writer - Bank swapping configuration bit
pub type SWAP_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
impl R {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option byte start change option configuration bit
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Flash mass erase enable bit
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 30 - Option byte change error interrupt enable bit
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Bank swapping configuration bit
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - FLASH_OPTCR lock option configuration bit
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<0> {
        OPTLOCK_W::new(self)
    }
    ///Bit 1 - Option byte start change option configuration bit
    #[inline(always)]
    #[must_use]
    pub fn optstart(&mut self) -> OPTSTART_W<1> {
        OPTSTART_W::new(self)
    }
    ///Bit 4 - Flash mass erase enable bit
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<4> {
        MER_W::new(self)
    }
    ///Bit 30 - Option byte change error interrupt enable bit
    #[inline(always)]
    #[must_use]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<30> {
        OPTCHANGEERRIE_W::new(self)
    }
    ///Bit 31 - Bank swapping configuration bit
    #[inline(always)]
    #[must_use]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<31> {
        SWAP_BANK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH option control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optcr_](index.html) module
pub struct OPTCR__SPEC;
impl crate::RegisterSpec for OPTCR__SPEC {
    type Ux = u32;
}
///`read()` method returns [optcr_::R](R) reader structure
impl crate::Readable for OPTCR__SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optcr_::W](W) writer structure
impl crate::Writable for OPTCR__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTCR_ to value 0
impl crate::Resettable for OPTCR__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
