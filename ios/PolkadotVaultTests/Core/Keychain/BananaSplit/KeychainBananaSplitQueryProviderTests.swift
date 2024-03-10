//
//  KeychainBananaSplitQueryProviderTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 04/03/2024.
//

import Foundation
@testable import PolkadotVault
import Security
import XCTest

final class KeychainBananaSplitQueryProviderTests: XCTestCase {
    private var subject: KeychainBananaSplitQueryProvider!
    private var jsonEncoder: JSONEncoder!

    override func setUp() {
        super.setUp()
        jsonEncoder = JSONEncoder()
        subject = KeychainBananaSplitQueryProvider(jsonEncoder: jsonEncoder)
    }

    override func tearDown() {
        jsonEncoder = nil
        subject = nil
        super.tearDown()
    }

    func test_query_fetchBananaSplit_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let queryType = KeychainBananaSplitQuery.fetch(seedName: seedName)

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, kSecMatchLimitOne)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.bananaSplitSuffix
        )
        XCTAssertEqual(result[kSecReturnData] as! Bool, true)
    }

    func test_query_checkBananaSplit_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let queryType = KeychainBananaSplitQuery.check(seedName: seedName)

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, kSecMatchLimitOne)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.bananaSplitSuffix
        )
        XCTAssertEqual(result[kSecReturnData] as! Bool, false)
    }

    func test_query_deleteBananaSplit_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let queryType = KeychainBananaSplitQuery.delete(seedName: seedName)

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.bananaSplitSuffix
        )
    }

    func test_query_saveBananaSplit_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let bananaSplit = BananaSplitBackup(qrCodes: [[10]])
        let queryType = KeychainBananaSplitQuery.save(seedName: seedName, bananaSplit: bananaSplit)
        let expectedData = try? jsonEncoder.encode(bananaSplit)

        // When
        let result = subject.query(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.bananaSplitSuffix
        )
        XCTAssertEqual(result[kSecValueData] as? Data, expectedData)
        XCTAssertEqual(result[kSecReturnData] as! Bool, false)
    }

    func test_query_fetchPassphrase_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let queryType = KeychainBananaSplitPassphraseQuery.fetch(seedName: seedName)

        // When
        let result = subject.passhpraseQuery(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(result[kSecMatchLimit] as! CFString, kSecMatchLimitOne)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.passphraseSuffix
        )
        XCTAssertEqual(result[kSecReturnData] as! Bool, true)
    }

    func test_query_deletePassphrase_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let queryType = KeychainBananaSplitPassphraseQuery.delete(seedName: seedName)

        // When
        let result = subject.passhpraseQuery(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.passphraseSuffix
        )
    }

    func test_query_savePassphrase_returnsExpectedValues() {
        // Given
        let seedName = "testSeed"
        let passphrase = BananaSplitPassphrase(passphrase: "dummyPassphrase")
        let expectedAccessControl: SecAccessControl! = try? SimulatorAccessControlProvider()
            .accessControl() // it's fine to use it instead of mock, as! it's just dedicated to be used on simulator
        let queryType = KeychainBananaSplitPassphraseQuery.save(
            seedName: seedName,
            passphrase: passphrase,
            accessControl: expectedAccessControl
        )
        let expectedData = try? jsonEncoder.encode(passphrase)

        // When
        let result = subject.passhpraseQuery(for: queryType) as! [CFString: Any]

        // Then
        XCTAssertEqual(result[kSecClass] as! CFString, kSecClassGenericPassword)
        XCTAssertEqual(
            result[kSecAttrAccount] as! String,
            seedName + KeychainBananaSplitQueryProvider.Constants.passphraseSuffix
        )
        XCTAssertEqual(result[kSecValueData] as? Data, expectedData)
        XCTAssertTrue(result[kSecAttrAccessControl] as! SecAccessControl === expectedAccessControl)
        XCTAssertEqual(result[kSecReturnData] as! Bool, false)
    }
}
