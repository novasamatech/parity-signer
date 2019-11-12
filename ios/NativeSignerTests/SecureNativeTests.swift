//
//  SecureNativeTests.swift
//  NativeSignerTests
//
//  Created by Joseph Mark on 11/11/19.
//  Copyright © 2019 Facebook. All rights reserved.
//

import Foundation
import XCTest

@testable import NativeSigner

let TEST_APP = "TEST_APP";
let NOT_TEST_APP = "NOT_TEST_APP";
let TEST_KEY = "TEST_KEY";
let NOT_TEST_KEY = "NOT_TEST_KEY";
let TEST_PIN = "42";

class SecureNativeTests: XCTestCase {
  let ethkey = EthkeyBridge();
  
  func testFailsWithoutBiometricAuth() {
    let expectation = XCTestExpectation(description: "Pin corresponds to key");
    self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) -> Void in
      XCTAssert(result as! UInt8 == 0);
      self.ethkey.securePut(TEST_APP, key: TEST_KEY, seed: TEST_PIN, withBiometry: 1, resolve: { (result: Any) in
        XCTAssertNotNil(result);
        self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
          XCTAssert(result as! UInt8 == 0);
          expectation.fulfill();
          } , reject: { (code, msg, error) in XCTAssert(false) });
        } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
    }, reject: { (code, msg, error) in XCTAssert(false) });
    wait(for: [expectation], timeout: 2.0);
  }
  
  func testPinCorrespondsToKey() {
    let expectation = XCTestExpectation(description: "Pin corresponds to key");
    self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) -> Void in
      XCTAssert(result as! UInt8 == 0);
      self.ethkey.securePut(TEST_APP, key: TEST_KEY, seed: TEST_PIN, withBiometry: 0, resolve: { (result: Any) in
        XCTAssertNotNil(result);
        self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
          XCTAssert(result as! UInt8 != 0);
          self.ethkey.secureDelete(TEST_APP, key: TEST_KEY, resolve: { (result: Any) in
            self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
              XCTAssert(result as! UInt8 == 0);
              expectation.fulfill();
            } , reject: { (code, msg, error) in XCTAssert(false) });
            } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
        } , reject: { (code, msg, error) in XCTAssert(false) });
        } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
    }, reject: { (code, msg, error) in XCTAssert(false) });
    wait(for: [expectation], timeout: 2.0);
  }
  
  func testPinDoesNotCorrespondToADifferentKey() {
    let expectation = XCTestExpectation(description: "Pin does not correspond to a different key");
    self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) -> Void in
      XCTAssert(result as! UInt8 == 0);
      self.ethkey.securePut(TEST_APP, key: NOT_TEST_KEY, seed: TEST_PIN, withBiometry: 0, resolve: { (result: Any) in
        XCTAssertNotNil(result);
        self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
          XCTAssert(result as! UInt8 == 0);
          self.ethkey.secureDelete(TEST_APP, key: NOT_TEST_KEY, resolve: { (result: Any) in
            XCTAssertNotNil(result);
            self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
              XCTAssert(result as! UInt8 == 0);
              expectation.fulfill();
            } , reject: { (code, msg, error) in XCTAssert(false) });
            } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
        } , reject: { (code, msg, error) in XCTAssert(false) });
        } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
    }, reject: { (code, msg, error) in XCTAssert(false) });
    wait(for: [expectation], timeout: 2.0);
  }
  
  func testFailsToDeleteNonExistentKey() {
    let expectation = XCTestExpectation(description: "Correct failure deleting non-existent key");
    self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) -> Void in
      XCTAssert(result as! UInt8 == 0);
      self.ethkey.securePut(TEST_APP, key: TEST_KEY, seed: TEST_PIN, withBiometry: 0, resolve: { (result: Any) in
        XCTAssertNotNil(result);
        self.ethkey.secureContains(TEST_APP, key: TEST_KEY, resolve: { (result: Any?) in
          XCTAssert(result as! UInt8 != 0);
          self.ethkey.secureDelete(TEST_APP, key: NOT_TEST_KEY, resolve: { (result: Any) in
            XCTAssert(false);
            } as (Any?) -> Void, reject: { (code, msg, error) in
              self.ethkey.secureDelete(TEST_APP, key: TEST_KEY, resolve: { (result: Any) in
                expectation.fulfill();
                } as (Any?) -> Void, reject: { (code, msg, error) in
                  XCTAssert(false);
              });
          });
        } , reject: { (code, msg, error) in XCTAssert(false) });
        } as (Any?) -> Void, reject: { (code, msg, error) in XCTAssert(false) });
    }, reject: { (code, msg, error) in XCTAssert(false) });
    wait(for: [expectation], timeout: 2.0);
  }
}
