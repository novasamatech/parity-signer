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

import Foundation
import XCTest

@testable import NativeSigner

let TEST_APP = "TEST_APP";
let TEST_KEY = "TEST_KEY";
let NOT_TEST_KEY = "NOT_TEST_KEY";
let TEST_PIN = "42";

class SecureNativeTests: XCTestCase {
  let ethkey = EthkeyBridge();
  
  func testFailsWithoutBiometricAuth() {
    let expectation = XCTestExpectation(description: "Pin corresponds to TEST_APP, key");
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
    let expectation = XCTestExpectation(description: "Pin corresponds to TEST_APP, key");
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
    let expectation = XCTestExpectation(description: "Pin does not correspond to a different TEST_APP, key");
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
    let expectation = XCTestExpectation(description: "Correct failure deleting non-existent TEST_APP, key");
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
