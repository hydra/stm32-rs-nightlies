///Register `TAMP_ATCR2` reader
pub struct R(crate::R<TAMP_ATCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ATCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ATCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ATCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TAMP_ATCR2` writer
pub struct W(crate::W<TAMP_ATCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ATCR2_SPEC>;
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
impl From<crate::W<TAMP_ATCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ATCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ATOSEL1` reader - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL1` writer - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL2` reader - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL2` writer - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL3` reader - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL3` writer - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL4` reader - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL4_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL4` writer - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\[1:0\]
///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
pub type ATOSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL5` reader - Active tamper shared output 5 selection The selected output must be available in the package pinout.
pub type ATOSEL5_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL5` writer - Active tamper shared output 5 selection The selected output must be available in the package pinout.
pub type ATOSEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL6` reader - Active tamper shared output 6 selection The selected output must be available in the package pinout.
pub type ATOSEL6_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL6` writer - Active tamper shared output 6 selection The selected output must be available in the package pinout.
pub type ATOSEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL7` reader - Active tamper shared output 7 selection The selected output must be available in the package pinout.
pub type ATOSEL7_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL7` writer - Active tamper shared output 7 selection The selected output must be available in the package pinout.
pub type ATOSEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
///Field `ATOSEL8` reader - Active tamper shared output 8 selection The selected output must be available in the package pinout.
pub type ATOSEL8_R = crate::FieldReader<u8, u8>;
///Field `ATOSEL8` writer - Active tamper shared output 8 selection The selected output must be available in the package pinout.
pub type ATOSEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_ATCR2_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 14) & 7) as u8)
    }
    ///Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel5(&self) -> ATOSEL5_R {
        ATOSEL5_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel6(&self) -> ATOSEL6_R {
        ATOSEL6_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel7(&self) -> ATOSEL7_R {
        ATOSEL7_R::new(((self.bits >> 26) & 7) as u8)
    }
    ///Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout.
    #[inline(always)]
    pub fn atosel8(&self) -> ATOSEL8_R {
        ATOSEL8_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 8:10 - Active tamper shared output 1 selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    #[must_use]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    ///Bits 11:13 - Active tamper shared output 2 selection The selected output must be available in the package pinout. Bits 12:11 are the mirror of ATOSEL2\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    #[must_use]
    pub fn atosel2(&mut self) -> ATOSEL2_W<11> {
        ATOSEL2_W::new(self)
    }
    ///Bits 14:16 - Active tamper shared output 3 selection The selected output must be available in the package pinout. Bits 15:14 are the mirror of ATOSEL3\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    #[must_use]
    pub fn atosel3(&mut self) -> ATOSEL3_W<14> {
        ATOSEL3_W::new(self)
    }
    ///Bits 17:19 - Active tamper shared output 4 selection The selected output must be available in the package pinout. Bits 18:17 are the mirror of ATOSEL2\[1:0\]
    ///in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1.
    #[inline(always)]
    #[must_use]
    pub fn atosel4(&mut self) -> ATOSEL4_W<17> {
        ATOSEL4_W::new(self)
    }
    ///Bits 20:22 - Active tamper shared output 5 selection The selected output must be available in the package pinout.
    #[inline(always)]
    #[must_use]
    pub fn atosel5(&mut self) -> ATOSEL5_W<20> {
        ATOSEL5_W::new(self)
    }
    ///Bits 23:25 - Active tamper shared output 6 selection The selected output must be available in the package pinout.
    #[inline(always)]
    #[must_use]
    pub fn atosel6(&mut self) -> ATOSEL6_W<23> {
        ATOSEL6_W::new(self)
    }
    ///Bits 26:28 - Active tamper shared output 7 selection The selected output must be available in the package pinout.
    #[inline(always)]
    #[must_use]
    pub fn atosel7(&mut self) -> ATOSEL7_W<26> {
        ATOSEL7_W::new(self)
    }
    ///Bits 29:31 - Active tamper shared output 8 selection The selected output must be available in the package pinout.
    #[inline(always)]
    #[must_use]
    pub fn atosel8(&mut self) -> ATOSEL8_W<29> {
        ATOSEL8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP active tamper control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tamp_atcr2](index.html) module
pub struct TAMP_ATCR2_SPEC;
impl crate::RegisterSpec for TAMP_ATCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tamp_atcr2::R](R) reader structure
impl crate::Readable for TAMP_ATCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tamp_atcr2::W](W) writer structure
impl crate::Writable for TAMP_ATCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TAMP_ATCR2 to value 0
impl crate::Resettable for TAMP_ATCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
