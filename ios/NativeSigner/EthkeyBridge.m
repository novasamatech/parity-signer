//
//  EthkeyBridge.m
//  NativeSigner
//
//  Created by Marek Kotewicz on 19/02/2017.
//  Copyright © 2017 Facebook. All rights reserved.
//

#import <React/RCTBridgeModule.h>

@interface RCT_EXTERN_MODULE(EthkeyBridge, NSObject)

RCT_EXTERN_METHOD(show:(NSString *)name)
RCT_EXTERN_METHOD(brainWalletAddress:(NSString*)seed callback:(RCTResponseSenderBlock)callback)

@end
