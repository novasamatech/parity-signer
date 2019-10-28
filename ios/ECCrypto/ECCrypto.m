// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.
//
//  ECCrypto.m
//

#import "ECCrypto.h"
#import <Foundation/Foundation.h>
#import <React/RCTUtils.h>

#define encryptAlgorithm        kSecKeyAlgorithmECIESEncryptionStandardX963SHA256AESGCM

#if TARGET_OS_SIMULATOR
static BOOL isSimulator = YES;
#else
static BOOL isSimulator = NO;
#endif

@implementation ECCrypto

RCT_EXPORT_MODULE();

/**
 * @return base64 pub key string
 */
- (SecKeyRef) generateECPair:(nonnull NSDictionary*) options
                      errMsg:(NSString **)errMsg
{
  CFErrorRef sacErr = NULL;
  SecAccessControlRef sacObject;
  sacObject = SecAccessControlCreateWithFlags(kCFAllocatorDefault,
                                              kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
                                              kSecAccessControlUserPresence | kSecAccessControlPrivateKeyUsage,
                                              &sacErr);
  
  if (sacErr) {
    *errMsg = [(__bridge NSError *)sacErr description];
    return nil;
  }
  
  // Create parameters dictionary for key generation.
  NSString* uuid = [options valueForKey:@"label"];
  
  if(uuid == nil) {
    uuid = [self uuidString];
  }
  NSString* publicKeyTag = [self toPublicIdentifier:uuid];
  
  NSMutableDictionary *privateKeyAttrs = [NSMutableDictionary dictionaryWithDictionary: @{
                                                                                          (__bridge id)kSecAttrIsPermanent: @YES,
                                                                                          (__bridge id)kSecAttrApplicationTag: uuid,
                                                                                          }];
  if (!isSimulator) {
    [privateKeyAttrs setObject:(__bridge_transfer id)sacObject forKey:(__bridge id)kSecAttrAccessControl];
  }
  NSDictionary *publicKeyAttrs = @{
                                   (__bridge id)kSecAttrIsPermanent: isSimulator ? @YES : @NO,
                                   (__bridge id)kSecAttrApplicationLabel: publicKeyTag,
                                   };
  NSMutableDictionary *parameters = [NSMutableDictionary dictionaryWithDictionary: @{
                                                                                     (__bridge id)kSecAttrKeyType: (__bridge id)kSecAttrKeyTypeEC,
                                                                                     (__bridge id)kSecAttrKeySizeInBits: @256,
                                                                                     (__bridge id)kSecPrivateKeyAttrs: privateKeyAttrs,
                                                                                     (__bridge id)kSecPublicKeyAttrs: publicKeyAttrs,
                                                                                     }];
  if (!isSimulator) {
    [parameters setObject:(__bridge id)kSecAttrTokenIDSecureEnclave forKey:(__bridge id)kSecAttrTokenID];
  }
  CFErrorRef error = NULL;
  SecKeyRef privateKey = SecKeyCreateRandomKey((__bridge CFDictionaryRef)parameters,
                                               &error);
  if (!privateKey) {
    *errMsg = [CFBridgingRelease(error) description];  // ARC takes ownership
    return nil;
  }
  
  SecKeyRef publicKey = SecKeyCopyPublicKey(privateKey);
  
  // Save public Key
  OSStatus status = SecItemAdd((__bridge CFDictionaryRef)@{
                                                           (__bridge id)kSecClass: (__bridge id)kSecClassKey,
                                                           (__bridge id)kSecAttrKeyClass: (__bridge id)kSecAttrKeyClassPublic,
                                                           (__bridge id)kSecAttrApplicationTag: publicKeyTag,
                                                           (__bridge id)kSecValueRef: (__bridge id)publicKey
                                                           }, nil);
  
  if (status != errSecSuccess) {
    CFRelease(privateKey);
    CFRelease(publicKey);
    *errMsg = keychainStatusToString(status);
    return nil;
  }
  
  CFRelease(privateKey);
  return publicKey;
}

RCT_EXPORT_METHOD(decrypt:(nonnull NSDictionary *)options
                  findEventsWithResolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
  dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
    NSString* errMsg;
    NSData* clearText = [self decrypt:options errMsg:&errMsg];
    if (!clearText) {
      reject(@"ECCrypto", errMsg, eccryptoMakeError(errMsg));
      return;
    }
    NSString* base64ClearText = [[NSString alloc] initWithData:clearText encoding:NSUTF8StringEncoding];
    resolve(base64ClearText);
  });
}

-(NSData *)decrypt:(nonnull NSDictionary *)options
            errMsg:(NSString **) errMsg
{
  NSString* keyPairTag = [options valueForKey:@"label"];
  SecKeyRef privateKeyRef = [self getPrivateKeyRef:keyPairTag errMsg:errMsg];
  if(!privateKeyRef)
    return nil;
  BOOL canDecrypt = SecKeyIsAlgorithmSupported(privateKeyRef,
                                               kSecKeyOperationTypeDecrypt,
                                               encryptAlgorithm);
  if(!canDecrypt) {
    *errMsg = @"can't decrypt";
    return nil;
  }
  NSData* clearText = nil;
  NSString* base64Data = [options valueForKey:@"data"];
  NSData *data = [[NSData alloc] initWithBase64EncodedString:base64Data options:0];
  CFErrorRef error = NULL;
  clearText = (NSData*)CFBridgingRelease(       // ARC takes ownership
                                         SecKeyCreateDecryptedData(privateKeyRef,
                                                                   encryptAlgorithm,
                                                                   (__bridge CFDataRef)data,
                                                                   &error));
  CFRelease(privateKeyRef);
  if (!clearText) {
    *errMsg = [CFBridgingRelease(error) description]; // ARC takes ownership
    return nil;
  }
  return clearText;
}

RCT_EXPORT_METHOD(encrypt:(nonnull NSDictionary *)options
                  findEventsWithResolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject) {
  dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
    NSString* errMsg;
    NSData* cipherText = [self encrypt:options errMsg:&errMsg];
    if (!cipherText) {
      reject(@"ECCrypto", errMsg, eccryptoMakeError(errMsg));
      return;
    }
    
    NSString* base64cipherText = [cipherText base64EncodedStringWithOptions:0];
    resolve(base64cipherText);
  });
}

- (NSData *)encrypt:(nonnull NSDictionary *)options
             errMsg:(NSString **) errMsg
{
  SecKeyRef publicKeyRef = [self getOrGenerateNewPublicKeyRef:options errMsg: errMsg];
  
  if(!publicKeyRef)
    return nil;
  
  BOOL canEncrypt = SecKeyIsAlgorithmSupported(publicKeyRef,
                                               kSecKeyOperationTypeEncrypt,
                                               encryptAlgorithm);
  if(!canEncrypt) {
    *errMsg = @"can't encrypt";
    return nil;
  }
  NSData* cipherText = nil;
  CFErrorRef error = NULL;
  NSString* base64Data = [options valueForKey:@"data"];
  NSData *data = [base64Data dataUsingEncoding:NSUTF8StringEncoding];
  //Releasing the data?
  cipherText = (NSData*)CFBridgingRelease(      // ARC takes ownership
                                          SecKeyCreateEncryptedData(publicKeyRef,
                                                                    encryptAlgorithm,
                                                                    (__bridge CFDataRef)data,
                                                                    &error));
  if (!cipherText) {
    *errMsg = [CFBridgingRelease(error) description];  // ARC takes ownership
    return nil;
  }
  
  CFRelease(publicKeyRef);
  return cipherText;
}

- (SecKeyRef) getOrGenerateNewPublicKeyRef:(nonnull NSDictionary*) options
                                    errMsg:(NSString **)errMsg
{
  NSString* keyPairTag = [options valueForKey:@"label"];
  
  SecKeyRef publicKeyRef = [self getPublicKeyRef:[self toPublicIdentifier:keyPairTag] errMsg:errMsg];
  if (!publicKeyRef) {
    publicKeyRef = [self generateECPair:options errMsg:errMsg];
    
    if (!publicKeyRef)
      return nil;
  }
  return publicKeyRef;
}

-(SecKeyRef)getPrivateKeyRef:(NSString *)privateKeyTag errMsg:(NSString **)errMsg
{
  NSDictionary *getPrivateKeyQuery = @{
                                       (__bridge id)kSecClass: (__bridge id)kSecClassKey,
                                       (__bridge id)kSecAttrKeyClass: (__bridge id)kSecAttrKeyClassPrivate,
                                       (__bridge id)kSecAttrApplicationTag: privateKeyTag,
                                       (__bridge id)kSecReturnRef:  @YES,
                                       };
  
  SecKeyRef privateKeyRef = NULL;
  OSStatus statusGetPrivateKey = SecItemCopyMatching((__bridge CFDictionaryRef)getPrivateKeyQuery,
                                                     (CFTypeRef *)&privateKeyRef);
  
  if (statusGetPrivateKey!=errSecSuccess) {
    *errMsg = keychainStatusToString(statusGetPrivateKey);
    //Is the SecKeyRef now still NULL? Need CFRelease the SecKeyRef?
    return nil;
  }
  if (!privateKeyRef) {
    *errMsg = @"can't find public key";
    return nil;
  }
  
  return privateKeyRef;
}

- (SecKeyRef)getPublicKeyRef:(NSString *)publicKeyTag errMsg:(NSString **)errMsg
{
  NSDictionary *getPublicKeyQuery = @{
                                      (__bridge id)kSecClass: (__bridge id)kSecClassKey,
                                      (__bridge id)kSecAttrKeyClass: (__bridge id)kSecAttrKeyClassPublic,
                                      (__bridge id)kSecAttrApplicationTag: publicKeyTag,
                                      (__bridge id)kSecReturnRef:  @YES,
                                      };
  SecKeyRef publicKeyRef = NULL;
  
  OSStatus statusGetPublicKey = SecItemCopyMatching((__bridge CFDictionaryRef)getPublicKeyQuery,
                                                    (CFTypeRef *)&publicKeyRef);
  if (statusGetPublicKey != errSecSuccess) {
    *errMsg = keychainStatusToString(statusGetPublicKey);
    //Is the SecKeyRef now still NULL? Need CFRelease the SecKeyRef?
    return nil;
  }
  if (!publicKeyRef) {
    *errMsg = @"can't find public key";
    return nil;
  }
  
  return publicKeyRef;
}

NSString *keychainStatusToString(OSStatus status) {
  NSString *message = [NSString stringWithFormat:@"%ld", (long)status];
  
  switch (status) {
    case errSecSuccess:
      message = @"success";
      break;
      
    case errSecDuplicateItem:
      message = @"error item already exists";
      break;
      
    case errSecItemNotFound :
      message = @"error item not found";
      break;
      
    case errSecAuthFailed:
      message = @"error item authentication failed";
      break;
      
    default:
      message = [NSString stringWithFormat:@"error with OSStatus %d", status];
      break;
  }
  
  return message;
}

- (NSString *) toPublicIdentifier:(NSString *)keyPairTag
{
  return [keyPairTag stringByAppendingString:@"-pub"];
}

- (NSString *)uuidString {
  CFUUIDRef uuid = CFUUIDCreate(kCFAllocatorDefault);
  NSString *uuidString = (__bridge_transfer NSString *)CFUUIDCreateString(kCFAllocatorDefault, uuid);
  CFRelease(uuid);
  
  return uuidString;
}

NSError* eccryptoMakeError(NSString* errMsg)
{
  return [NSError errorWithDomain:@"ECCrypto" code:1 userInfo:@{NSLocalizedDescriptionKey:errMsg}];
}

@end
