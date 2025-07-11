#[doc = "Register `CNDTR7` reader"]
pub type R = crate::R<Cndtr7Spec>;
#[doc = "Register `CNDTR7` writer"]
pub type W = crate::W<Cndtr7Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, Cndtr7Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA channel 7 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr7Spec;
impl crate::RegisterSpec for Cndtr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr7::R`](R) reader structure"]
impl crate::Readable for Cndtr7Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr7::W`](W) writer structure"]
impl crate::Writable for Cndtr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDTR7 to value 0"]
impl crate::Resettable for Cndtr7Spec {}
