//
//  KeychainQueryProviderTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

@testable import PolkadotVault
import XCTest

// swiftlint:disable force_cast
final class KeychainQueryProviderTests: XCTestCase {
    private var subject: KeychainSeedsQueryProvider!

    override func setUp() {
        super.setUp()
        subject = KeychainSeedsQueryProvider()
    }

    func test_query_fetch_returnsExpectedValues() {
        // Given
        let queryType: KeychainSeedsQuery = .fetch
        let expectedSecClass = kSecClassGenericPassword
        let expectedMatchLimit = kSecMatchLimitAll
        let expectedReturnAttributes = true
        let expectedReturnData = false

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, expectedMatchLimit)
        XCTAssertEqual(result[kSecReturnAttributes] as! Bool, expectedReturnAttributes)
        XCTAssertEqual(result[kSecReturnData] as! Bool, expectedReturnData)
    }

    func test_query_deleteAll_returnsExpectedValues() {
        // Given
        let queryType: KeychainSeedsQuery = .deleteAll
        let expectedSecClass = kSecClassGenericPassword

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
    }

    func test_query_check_returnsExpectedValues() {
        // Given
        let queryType: KeychainSeedsQuery = .check
        let expectedSecClass = kSecClassGenericPassword
        let expectedMatchLimit = kSecMatchLimitAll
        let expectedReturnData = true

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, expectedMatchLimit)
        XCTAssertEqual(result[kSecReturnData] as! Bool, expectedReturnData)
    }

    func test_query_search_returnsExpectedValues() {
        // Given
        let seedName = "account"
        let queryType: KeychainSeedsQuery = .search(seedName: seedName)
        let expectedSecClass = kSecClassGenericPassword
        let expectedMatchLimit = kSecMatchLimitOne
        let expectedReturnData = true

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, expectedMatchLimit)
        XCTAssertEqual(result[kSecAttrAccount] as! String, seedName)
        XCTAssertEqual(result[kSecReturnData] as! Bool, expectedReturnData)
    }

    func test_query_delete_returnsExpectedValues() {
        // Given
        let seedName = "account"
        let expectedSecClass = kSecClassGenericPassword
        let queryType: KeychainSeedsQuery = .delete(seedName: seedName)

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
        XCTAssertEqual(result[kSecAttrAccount] as! String, seedName)
    }

    func test_query_restoreQuery_returnsExpectedValues() {
        // Given
        let seedName = "account"
        let finalSeedPhrase: Data! = "account".data(using: .utf8)
        let expectedSecClass = kSecClassGenericPassword
        let expectedAccessControl: SecAccessControl! = try? SimulatorAccessControlProvider()
            .accessControl() // it's fine to use it instead of mock, as! it's just dedicated to be used on simulator
        let expectedReturnData = true
        let queryType: KeychainSeedsQuery = .restoreQuery(
            seedName: seedName,
            finalSeedPhrase: finalSeedPhrase,
            accessControl: expectedAccessControl
        )
        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, expectedSecClass)
        XCTAssertTrue(result[kSecAttrAccessControl] as! SecAccessControl === expectedAccessControl)
        XCTAssertEqual(result[kSecAttrAccount] as! String, seedName)
        XCTAssertEqual(result[kSecValueData] as? Data, finalSeedPhrase)
        XCTAssertEqual(result[kSecReturnData] as! Bool, expectedReturnData)
    }
}
