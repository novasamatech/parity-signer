#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE(SubstrateSign, NSObject)

// explanation about threading here: https://stackoverflow.com/a/50775641/3060739
- (dispatch_queue_t)methodQueue
{
  return dispatch_get_main_queue();
}

+ (BOOL)requiresMainQueueSetup
{
  return YES;
}

RCT_EXTERN_METHOD(brainWalletAddress:(NSString*)seed resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(brainWalletSecret:(NSString*)seed resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(brainWalletSign:(NSString*)seed message:(NSString*)message resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(rlpItem:(NSString*)rlp position:(NSUInteger)position resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(keccak:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(ethSign:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(blockiesIcon:(NSString*)seed resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(randomPhrase:(NSInteger*)wordsNumber resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(encryptData:(NSString*)data password:(NSString*)password resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(decryptData:(NSString*)data password:(NSString*)password resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(qrCode:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(qrCodeHex:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateAddress:(NSString*)seed prefix:(NSUInteger*)prefix resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateSign:(NSString*)seed message:(NSString*)message resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(blake2b:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(schnorrkelVerify: (NSString*)seed message:(NSString*)message signature:(NSString*)signature resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(decryptDataRef:(NSString*)data password:(NSString*)password resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(destroyDataRef:(int64_t)dataRef resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(brainWalletSignWithRef:(int64_t)seedRef message:(NSString*)message resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateSignWithRef:(int64_t)seedRef suri_suffix:(NSString*)suri_suffix message:(NSString*)message  resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateAddressWithRef:(int64_t)seedRef suri_suffix:(NSString*)suri_suffix prefix:(NSUInteger*)prefix resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(brainWalletAddressWithRef:(int64_t)seedRef resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateSecret:(NSString*)suri resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(substrateSecretWithRef:(int64_t*)seedRef suri_suffix:(NSString*)suri_suffix resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(tryDecodeQrSequence:(NSInteger*)size chunk_size:(NSInteger*)chunk_size data:(NSString*)data resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(generateMetadataHandle:(NSString*)metadata resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(parseTransaction:(NSString*)payload gen_hash:(NSString*)gen_hash metadata:(NSString*)metadata type_desc:(NSString*)type_desc identities:(NSString*)identities resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(signTransaction:(NSString*)action pin:(NSString*)pin password:(NSString*)password resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)
RCT_EXTERN_METHOD(developmentTest:(NSString*)input resolve:(RCTPromiseResolveBlock)resolve reject:(RCTPromiseRejectBlock)reject)

@end
