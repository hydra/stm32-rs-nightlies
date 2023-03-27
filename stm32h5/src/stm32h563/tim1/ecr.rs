///Register `ECR` reader
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ECR` writer
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IE` reader - Index enable This bit indicates if the Index event resets the counter.
pub type IE_R = crate::BitReader<bool>;
///Field `IE` writer - Index enable This bit indicates if the Index event resets the counter.
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
///Field `IDIR` reader - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\[1:0\]
///bitfield must be written when IE bit is reset (index disabled).
pub type IDIR_R = crate::FieldReader<u8, u8>;
///Field `IDIR` writer - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\[1:0\]
///bitfield must be written when IE bit is reset (index disabled).
pub type IDIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
///Field `IBLK` reader - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input
pub type IBLK_R = crate::FieldReader<u8, u8>;
///Field `IBLK` writer - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input
pub type IBLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
///Field `FIDX` reader - First index This bit indicates if the first index only is taken into account
pub type FIDX_R = crate::BitReader<bool>;
///Field `FIDX` writer - First index This bit indicates if the first index only is taken into account
pub type FIDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
///Field `IPOS` reader - Index positioning In quadrature encoder mode (SMS\[3:0\]
///= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\[3:0\]
///= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\[1\]
///bit is not significant
pub type IPOS_R = crate::FieldReader<u8, u8>;
///Field `IPOS` writer - Index positioning In quadrature encoder mode (SMS\[3:0\]
///= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\[3:0\]
///= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\[1\]
///bit is not significant
pub type IPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 2, O>;
///Field `PW` reader - Pulse width This bitfield defines the pulse duration, as following: t&lt;sub>PW&lt;/sub> = PW\[7:0\]
///x t&lt;sub>PWG&lt;/sub>
pub type PW_R = crate::FieldReader<u8, u8>;
///Field `PW` writer - Pulse width This bitfield defines the pulse duration, as following: t&lt;sub>PW&lt;/sub> = PW\[7:0\]
///x t&lt;sub>PWG&lt;/sub>
pub type PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
///Field `PWPRSC` reader - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: t&lt;sub>PWG&lt;/sub> = (2&lt;sup>(PWPRSC\[2:0\])&lt;/sup>) x t&lt;sub>tim_ker_ck&lt;/sub>
pub type PWPRSC_R = crate::FieldReader<u8, u8>;
///Field `PWPRSC` writer - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: t&lt;sub>PWG&lt;/sub> = (2&lt;sup>(PWPRSC\[2:0\])&lt;/sup>) x t&lt;sub>tim_ker_ck&lt;/sub>
pub type PWPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - Index enable This bit indicates if the Index event resets the counter.
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\[1:0\]
    ///bitfield must be written when IE bit is reset (index disabled).
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - First index This bit indicates if the first index only is taken into account
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Index positioning In quadrature encoder mode (SMS\[3:0\]
    ///= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\[3:0\]
    ///= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\[1\]
    ///bit is not significant
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: t&lt;sub>PW&lt;/sub> = PW\[7:0\]
    ///x t&lt;sub>PWG&lt;/sub>
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: t&lt;sub>PWG&lt;/sub> = (2&lt;sup>(PWPRSC\[2:0\])&lt;/sup>) x t&lt;sub>tim_ker_ck&lt;/sub>
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Index enable This bit indicates if the Index event resets the counter.
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    ///Bits 1:2 - Index direction This bit indicates in which direction the Index event resets the counter. Note: The IDR\[1:0\]
    ///bitfield must be written when IE bit is reset (index disabled).
    #[inline(always)]
    #[must_use]
    pub fn idir(&mut self) -> IDIR_W<1> {
        IDIR_W::new(self)
    }
    ///Bits 3:4 - Index blanking This bit indicates if the Index event is conditioned by the tim_ti3 input
    #[inline(always)]
    #[must_use]
    pub fn iblk(&mut self) -> IBLK_W<3> {
        IBLK_W::new(self)
    }
    ///Bit 5 - First index This bit indicates if the first index only is taken into account
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FIDX_W<5> {
        FIDX_W::new(self)
    }
    ///Bits 6:7 - Index positioning In quadrature encoder mode (SMS\[3:0\]
    ///= 0001, 0010, 0011, 1110, 1111), this bit indicates in which AB input configuration the Index event resets the counter. In directional clock mode or clock plus direction mode (SMS\[3:0\]
    ///= 1010, 1011, 1100, 1101), these bits indicates on which level the Index event resets the counter. In bidirectional clock mode, this applies for both clock inputs. x0: Index resets the counter when clock is 0 x1: Index resets the counter when clock is 1 Note: IPOS\[1\]
    ///bit is not significant
    #[inline(always)]
    #[must_use]
    pub fn ipos(&mut self) -> IPOS_W<6> {
        IPOS_W::new(self)
    }
    ///Bits 16:23 - Pulse width This bitfield defines the pulse duration, as following: t&lt;sub>PW&lt;/sub> = PW\[7:0\]
    ///x t&lt;sub>PWG&lt;/sub>
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<16> {
        PW_W::new(self)
    }
    ///Bits 24:26 - Pulse width prescaler This bitfield sets the clock prescaler for the pulse generator, as following: t&lt;sub>PWG&lt;/sub> = (2&lt;sup>(PWPRSC\[2:0\])&lt;/sup>) x t&lt;sub>tim_ker_ck&lt;/sub>
    #[inline(always)]
    #[must_use]
    pub fn pwprsc(&mut self) -> PWPRSC_W<24> {
        PWPRSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 timer encoder control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ecr](index.html) module
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ecr::R](R) reader structure
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ecr::W](W) writer structure
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ECR to value 0
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
