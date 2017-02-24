//
//  EthkeyBridge.m
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE(EthkeyBridge, NSObject)

RCT_EXTERN_METHOD(brainWalletAddress:(NSString*)seed callback:(RCTResponseSenderBlock)callback)
RCT_EXTERN_METHOD(brainWalletSecret:(NSString*)seed callback:(RCTResponseSenderBlock)callback)
RCT_EXTERN_METHOD(brainWalletSign:(NSString*)seed message:(NSString*)message callback:(RCTResponseSenderBlock)callback)
RCT_EXTERN_METHOD(rlpItem:(NSString*)rlp position:(NSUInteger)position callback:(RCTResponseSenderBlock)callback)

@end
