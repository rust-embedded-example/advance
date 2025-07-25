#[doc = "Register `IDCODE` reader"]
pub type R = crate::R<IdcodeSpec>;
#[doc = "Field `DEV_ID` reader - Device Identifier"]
pub type DevIdR = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - Revision Identifier"]
pub type RevIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Device Identifier"]
    #[inline(always)]
    pub fn dev_id(&self) -> DevIdR {
        DevIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - Revision Identifier"]
    #[inline(always)]
    pub fn rev_id(&self) -> RevIdR {
        RevIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "MCU Device ID Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcodeSpec;
impl crate::RegisterSpec for IdcodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idcode::R`](R) reader structure"]
impl crate::Readable for IdcodeSpec {}
#[doc = "`reset()` method sets IDCODE to value 0"]
impl crate::Resettable for IdcodeSpec {}
