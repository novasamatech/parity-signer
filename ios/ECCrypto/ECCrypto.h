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
//  ECCrypto.h
//

#import <Foundation/Foundation.h>
#import <React/RCTBridgeModule.h>

@interface ECCrypto : NSObject <RCTBridgeModule>

- (NSString * _Nonnull) toPublicIdentifier:(NSString * _Nonnull)keyPairTag;
- (SecKeyRef _Nullable) getPublicKeyRef:(NSString * _Nullable)publicKeyTag errMsg:(NSString *_Nullable*_Nullable)errMsg;
- (SecKeyRef _Nullable) getPrivateKeyRef:(NSString * _Nullable)privateKeyTag errMsg:(NSString *_Nullable*_Nullable)errMsg;
- (SecKeyRef _Nullable) getOrGenerateNewPublicKeyRef:(NSDictionary * _Nonnull) options
                                              errMsg:(NSString *_Nullable*_Nullable)errMsg;
- (NSString * _Nonnull) uuidString;
- (NSData * _Nullable)encrypt:(NSDictionary* _Nonnull)options errMsg:(NSString *_Nullable*_Nullable) errMsg;
- (NSData * _Nullable)decrypt:(NSDictionary* _Nonnull)options errMsg:(NSString *_Nullable*_Nullable) errMsg;
@end
