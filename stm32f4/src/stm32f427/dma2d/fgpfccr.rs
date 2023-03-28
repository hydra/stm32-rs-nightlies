///Register `FGPFCCR` reader
pub struct R(crate::R<FGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FGPFCCR` writer
pub struct W(crate::W<FGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGPFCCR_SPEC>;
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
impl From<crate::W<FGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CM` reader - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
pub type CM_R = crate::FieldReader<u8, u8>;
///Field `CM` writer - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 4, O>;
///Field `CCM` reader - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
pub type CCM_R = crate::BitReader<bool>;
///Field `CCM` writer - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
///Field `START` reader - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).
pub type START_R = crate::BitReader<bool>;
///Field `START` writer - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
///Field `CS` reader - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\]
///+ 1.
pub type CS_R = crate::FieldReader<u8, u8>;
///Field `CS` writer - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\]
///+ 1.
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
///Field `AM` reader - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless
pub type AM_R = crate::FieldReader<u8, u8>;
///Field `AM` writer - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 2, O>;
///Field `CSS` reader - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless
pub type CSS_R = crate::FieldReader<u8, u8>;
///Field `CSS` writer - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless
pub type CSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 2, O>;
///Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
pub type AI_R = crate::BitReader<bool>;
///Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
///Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
pub type RBS_R = crate::BitReader<bool>;
///Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
pub type RBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGPFCCR_SPEC, bool, O>;
///Field `ALPHA` reader - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\[1:0\]
///bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.
pub type ALPHA_R = crate::FieldReader<u8, u8>;
///Field `ALPHA` writer - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\[1:0\]
///bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FGPFCCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:3 - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\]
    ///+ 1.
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\[1:0\]
    ///bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:3 - Color mode These bits defines the color format of the foreground image. They can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    ///Bit 4 - CLUT color mode This bit defines the color format of the CLUT. It can only be written when the transfer is disabled. Once the CLUT transfer has started, this bit is read-only.
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<4> {
        CCM_W::new(self)
    }
    ///Bit 5 - Start This bit can be set to start the automatic loading of the CLUT. It is automatically reset: ** at the end of the transfer ** when the transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** when a transfer error occurs ** when the transfer has not started due to a configuration error or another transfer operation already ongoing (data transfer or automatic background CLUT transfer).
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<5> {
        START_W::new(self)
    }
    ///Bits 8:15 - CLUT size These bits define the size of the CLUT used for the foreground image. Once the CLUT transfer has started, this field is read-only. The number of CLUT entries is equal to CS\[7:0\]
    ///+ 1.
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    ///Bits 16:17 - Alpha mode These bits select the alpha channel value to be used for the foreground image. They can only be written data the transfer are disabled. Once the transfer has started, they become read-only. other configurations are meaningless
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<16> {
        AM_W::new(self)
    }
    ///Bits 18:19 - Chroma Sub-Sampling These bits define the chroma sub-sampling mode for YCbCr color mode. Once the transfer has started, these bits are read-only. others: meaningless
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CSS_W<18> {
        CSS_W::new(self)
    }
    ///Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<20> {
        AI_W::new(self)
    }
    ///Bit 21 - Red Blue Swap This bit allows to swap the R &amp;amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<21> {
        RBS_W::new(self)
    }
    ///Bits 24:31 - Alpha value These bits define a fixed alpha channel value which can replace the original alpha value or be multiplied by the original alpha value according to the alpha mode selected through the AM\[1:0\]
    ///bits. These bits can only be written when data transfers are disabled. Once a transfer has started, they become read-only.
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D foreground PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fgpfccr](index.html) module
pub struct FGPFCCR_SPEC;
impl crate::RegisterSpec for FGPFCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fgpfccr::R](R) reader structure
impl crate::Readable for FGPFCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fgpfccr::W](W) writer structure
impl crate::Writable for FGPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FGPFCCR to value 0
impl crate::Resettable for FGPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
