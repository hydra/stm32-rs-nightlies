///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<bool>;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `PVDFT` reader - Power voltage detector falling threshold selection
pub type PVDFT_R = crate::FieldReader<u8, u8>;
///Field `PVDFT` writer - Power voltage detector falling threshold selection
pub type PVDFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
///Field `PVDRT` reader - Power voltage detector rising threshold selection
pub type PVDRT_R = crate::FieldReader<u8, u8>;
///Field `PVDRT` writer - Power voltage detector rising threshold selection
pub type PVDRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
///Field `PVMENDAC` reader - PVMENDAC
pub type PVMENDAC_R = crate::BitReader<bool>;
///Field `PVMENDAC` writer - PVMENDAC
pub type PVMENDAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `PVMENUSB` reader - PVMENUSB
pub type PVMENUSB_R = crate::BitReader<bool>;
///Field `PVMENUSB` writer - PVMENUSB
pub type PVMENUSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `IOSV` reader - IOSV
pub type IOSV_R = crate::BitReader<bool>;
///Field `IOSV` writer - IOSV
pub type IOSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `USV` reader - USV
pub type USV_R = crate::BitReader<bool>;
///Field `USV` writer - USV
pub type USV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    pub fn pvdft(&self) -> PVDFT_R {
        PVDFT_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    pub fn pvdrt(&self) -> PVDRT_R {
        PVDRT_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    pub fn pvmendac(&self) -> PVMENDAC_R {
        PVMENDAC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    pub fn pvmenusb(&self) -> PVMENUSB_R {
        PVMENUSB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USV
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power voltage detector enable
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    ///Bits 1:3 - Power voltage detector falling threshold selection
    #[inline(always)]
    #[must_use]
    pub fn pvdft(&mut self) -> PVDFT_W<1> {
        PVDFT_W::new(self)
    }
    ///Bits 4:6 - Power voltage detector rising threshold selection
    #[inline(always)]
    #[must_use]
    pub fn pvdrt(&mut self) -> PVDRT_W<4> {
        PVDRT_W::new(self)
    }
    ///Bit 7 - PVMENDAC
    #[inline(always)]
    #[must_use]
    pub fn pvmendac(&mut self) -> PVMENDAC_W<7> {
        PVMENDAC_W::new(self)
    }
    ///Bit 8 - PVMENUSB
    #[inline(always)]
    #[must_use]
    pub fn pvmenusb(&mut self) -> PVMENUSB_W<8> {
        PVMENUSB_W::new(self)
    }
    ///Bit 9 - IOSV
    #[inline(always)]
    #[must_use]
    pub fn iosv(&mut self) -> IOSV_W<9> {
        IOSV_W::new(self)
    }
    ///Bit 10 - USV
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<10> {
        USV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
