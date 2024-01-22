//
//  SignSpecDetailsViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 19/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class SignSpecDetailsViewModelTests: XCTestCase {
    private var viewModel: SignSpecDetails.ViewModel!
    private var cancelBag: CancelBag!
    private var metadataSpecsVersion: String!

    override func setUp() {
        super.setUp()
        metadataSpecsVersion = "version"
        viewModel = SignSpecDetails.ViewModel(content: .generate(), type: .metadata(metadataSpecsVersion: ""))
        cancelBag = CancelBag()
    }

    override func tearDown() {
        viewModel = nil
        cancelBag.cancel()
        cancelBag = nil
        super.tearDown()
    }

    func testOnBackTap_SendsDismissRequest() {
        // Given
        let dismissRequestExpectation = expectation(description: "Dismiss request sent")

        viewModel.dismissViewRequest
            .sink { _ in
                // Then
                dismissRequestExpectation.fulfill()
            }
            .store(in: cancelBag)

        // When
        viewModel.onBackTap()

        wait(for: [dismissRequestExpectation], timeout: 1.0)
    }

    func testTitle_ForMetadataType() {
        // Given
        viewModel = SignSpecDetails.ViewModel(content: .generate(), type: .metadata(metadataSpecsVersion: ""))

        // Then
        XCTAssertEqual(viewModel.title, Localizable.SignSpecsDetails.Label.Title.metadata.string)
    }

    func testTitle_ForNetworkType() {
        // Given
        viewModel = SignSpecDetails.ViewModel(content: .generate(), type: .network)

        // Then
        XCTAssertEqual(viewModel.title, Localizable.SignSpecsDetails.Label.Title.specs.string)
    }

    func testQRCodeSectionTitle_ForMetadataType() {
        // Given
        viewModel = SignSpecDetails.ViewModel(content: .generate(), type: .metadata(metadataSpecsVersion: ""))

        // Then
        XCTAssertEqual(viewModel.qrCodeSectionTitle, Localizable.SignSpecsDetails.Label.ScanQRCode.metadata.string)
    }

    func testQRCodeSectionTitle_ForNetworkType() {
        // Given
        viewModel = SignSpecDetails.ViewModel(content: .generate(), type: .network)

        // Then
        XCTAssertEqual(viewModel.qrCodeSectionTitle, Localizable.SignSpecsDetails.Label.ScanQRCode.specs.string)
    }
}
