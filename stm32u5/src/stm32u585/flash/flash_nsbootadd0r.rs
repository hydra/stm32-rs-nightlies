///Register `FLASH_NSBOOTADD0R` reader
pub struct R(crate::R<FLASH_NSBOOTADD0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_NSBOOTADD0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_NSBOOTADD0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_NSBOOTADD0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_NSBOOTADD0R` writer
pub struct W(crate::W<FLASH_NSBOOTADD0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_NSBOOTADD0R_SPEC>;
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
impl From<crate::W<FLASH_NSBOOTADD0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_NSBOOTADD0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NSBOOTADD0` reader - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
///= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
///= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
///= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
pub type NSBOOTADD0_R = crate::FieldReader<u32, u32>;
///Field `NSBOOTADD0` writer - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
///= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
///= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
///= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
pub type NSBOOTADD0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_NSBOOTADD0R_SPEC, u32, u32, 25, O>;
impl R {
    ///Bits 7:31 - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
    ///= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
    ///= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
    ///= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
    #[inline(always)]
    pub fn nsbootadd0(&self) -> NSBOOTADD0_R {
        NSBOOTADD0_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    ///Bits 7:31 - Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \[31:7\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\[24:0\]
    ///= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\[24:0\]
    ///= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\[24:0\]
    ///= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)
    #[inline(always)]
    #[must_use]
    pub fn nsbootadd0(&mut self) -> NSBOOTADD0_W<7> {
        NSBOOTADD0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH non-secure boot address 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_nsbootadd0r](index.html) module
pub struct FLASH_NSBOOTADD0R_SPEC;
impl crate::RegisterSpec for FLASH_NSBOOTADD0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_nsbootadd0r::R](R) reader structure
impl crate::Readable for FLASH_NSBOOTADD0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_nsbootadd0r::W](W) writer structure
impl crate::Writable for FLASH_NSBOOTADD0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_NSBOOTADD0R to value 0x0f
impl crate::Resettable for FLASH_NSBOOTADD0R_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
