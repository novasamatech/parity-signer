//
//  NativeSignerTests.swift
//  NativeSignerTests
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import XCTest
@testable import NativeSigner

class NativeSignerTests: XCTestCase {
    
    override func setUpWithError() throws {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDownWithError() throws {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testExample() throws {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct results.
    }

    func testTransactionCardsDecode() throws {
        let res = get_all_tx_cards(nil)
        let actionResultJSONString = String(cString: res!)
        print(actionResultJSONString)
        let actionResultJSON = actionResultJSONString.data(using: .utf8)
        let transactionCards = try JSONDecoder().decode(TransactionCardSet.self, from: actionResultJSON!)
        print(transactionCards.assemble())
    }
    
    func testHistoryDecode() throws {
        let res = get_all_log_cards(nil)
        let actionResultJSONString = String(cString: res!)
        print(actionResultJSONString)
        let actionResultJSON = actionResultJSONString.data(using: .utf8)
        let eventCards = try JSONDecoder().decode(History.self, from: actionResultJSON!)
        print(eventCards)
    }
    
    func testPerformanceExample() throws {
        // This is an example of a performance test case.
        self.measure {
            // Put the code you want to measure the time of here.
        }
    }

}
