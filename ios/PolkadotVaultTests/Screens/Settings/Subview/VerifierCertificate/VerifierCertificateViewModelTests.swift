//
//  VerifierCertificateViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 12/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class VerifierCertificateViewModelTests: XCTestCase {
    private var viewModel: VerifierCertificateView.ViewModel!
    private var onboardingMediatorMock: OnboardingMediatingMock!
    private var generalVerifierServiceMock: GeneralVerifierServicingMock!
    private var cancelBag: CancelBag!

    override func setUp() {
        super.setUp()
        cancelBag = CancelBag()
        onboardingMediatorMock = OnboardingMediatingMock()
        generalVerifierServiceMock = GeneralVerifierServicingMock()
    }

    override func tearDown() {
        cancelBag = nil
        viewModel = nil
        onboardingMediatorMock = nil
        generalVerifierServiceMock = nil
        super.tearDown()
    }

    func testInit_whenLoadDataSuccess_callsService_loadsContent() {
        // Given
        let verifierDetails = MVerifierDetails.generate()

        // When
        viewModel = VerifierCertificateView.ViewModel(
            onboardingMediator: onboardingMediatorMock,
            service: generalVerifierServiceMock
        )
        generalVerifierServiceMock.getGeneralVerifierReceivedCompletion.first?(.success(verifierDetails))

        // Then
        XCTAssertEqual(generalVerifierServiceMock.getGeneralVerifierCallsCount, 1)
        XCTAssertEqual(viewModel.content, verifierDetails)
    }

    func testInit_whenLoadDataFails_callsService_presentsError_contentNil() {
        // Given
        let error = ServiceError(message: "Error occurred")

        // When
        viewModel = VerifierCertificateView.ViewModel(
            onboardingMediator: onboardingMediatorMock,
            service: generalVerifierServiceMock
        )
        generalVerifierServiceMock.getGeneralVerifierReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(generalVerifierServiceMock.getGeneralVerifierCallsCount, 1)
        XCTAssertTrue(viewModel.isPresentingError)
        XCTAssertEqual(viewModel.presentableError, .alertError(message: error.localizedDescription))
        XCTAssertNil(viewModel.content)
    }

    func testOnRemoveTap() {
        // Given
        viewModel = VerifierCertificateView.ViewModel(
            onboardingMediator: onboardingMediatorMock,
            service: generalVerifierServiceMock
        )

        // When
        viewModel.onRemoveTap()

        // Then
        XCTAssertTrue(viewModel.isPresentingRemoveConfirmation)
    }

    func testOnRemoveConfirmationTap() {
        // Given
        viewModel = VerifierCertificateView.ViewModel(
            onboardingMediator: onboardingMediatorMock,
            service: generalVerifierServiceMock
        )

        // When
        viewModel.onRemoveConfirmationTap()

        // Then
        XCTAssertEqual(onboardingMediatorMock.onboardVerifierRemovedCallsCount, 1)
        XCTAssertTrue(onboardingMediatorMock.onboardVerifierRemovedReceivedVerifierRemoved.contains(true))
        XCTAssertFalse(viewModel.isPresentingRemoveConfirmation)
    }

    func testDismissRequestIsSentOnRemoveConfirmation() {
        // Given
        viewModel = VerifierCertificateView.ViewModel(
            onboardingMediator: onboardingMediatorMock,
            service: generalVerifierServiceMock
        )
        let expectation = XCTestExpectation(description: "Dismiss request should be sent")
        var dismissRequestTriggered = false

        viewModel.dismissViewRequest.sink {
            dismissRequestTriggered = true
            expectation.fulfill()
        }
        .store(in: cancelBag)

        // When
        viewModel.onRemoveConfirmationTap()

        // Then
        wait(for: [expectation], timeout: 1.0)
        XCTAssertTrue(dismissRequestTriggered, "Dismiss request should be triggered on remove confirmation")
    }
}
