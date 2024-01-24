//
//  LogsListViewModelTests.swift
//  PolkadotVaultTests
//
//  Created by Krzysztof Rodak on 22/01/2024.
//

import Combine
import Foundation
@testable import PolkadotVault
import XCTest

final class LogsListViewViewModelTests: XCTestCase {
    private var viewModel: LogsListView.ViewModel!
    private var logsServiceMock: LogsServicingMock!
    private var devicePasscodeAuthenticatorMock: DevicePasscodeAuthenticatorProtocolMock!
    private var renderableBuilderMock: LogEntryRenderableBuildingMock!
    private var cancelBag: Set<AnyCancellable>!

    override func setUp() {
        super.setUp()
        logsServiceMock = LogsServicingMock()
        devicePasscodeAuthenticatorMock = DevicePasscodeAuthenticatorProtocolMock()
        renderableBuilderMock = LogEntryRenderableBuildingMock()
        viewModel = LogsListView.ViewModel(
            logsService: logsServiceMock,
            devicePasscodeAuthenticator: devicePasscodeAuthenticatorMock,
            renderableBuilder: renderableBuilderMock
        )
        cancelBag = []
    }

    override func tearDown() {
        viewModel = nil
        logsServiceMock = nil
        devicePasscodeAuthenticatorMock = nil
        renderableBuilderMock = nil
        cancelBag = nil
        super.tearDown()
    }

    func testLoadData_Success() {
        // Given
        let logs = MLog.generate()
        let renderables = [LogEntryRenderable.generate()]
        renderableBuilderMock.buildReturnValue = renderables

        // When
        viewModel.loadData()
        logsServiceMock.getLogsReceivedCompletion.first?(.success(logs))

        // Then
        XCTAssertEqual(viewModel.renderables, renderables)
    }

    func testLoadData_Failure() {
        // Given
        let error = ServiceError(message: "Error")
        let presentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.loadData()
        logsServiceMock.getLogsReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(viewModel.presentableError, presentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testOnEventTap_eventBasic_noCallsToService() {
        // Given
        let renderable = LogEntryRenderable.generate(type: .basic)

        // When
        viewModel.onEventTap(renderable)

        // Then
        XCTAssertEqual(logsServiceMock.getLogDetailsCallsCount, 0)
    }

    func testOnEventTap_Success() {
        // Given
        let renderable = LogEntryRenderable.generate(type: .bottomDetails)
        let details = MLogDetails.generate()

        // When
        viewModel.onEventTap(renderable)
        logsServiceMock.getLogDetailsReceivedCompletion.first?(.success(details))

        // Then
        XCTAssertEqual(viewModel.selectedDetails, details)
        XCTAssertTrue(viewModel.isPresentingDetails)
    }

    func testOnEventTap_Failure() {
        // Given
        let renderable = LogEntryRenderable.generate(type: .bottomDetails)
        let error = ServiceError(message: "Error")
        let presentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.onEventTap(renderable)
        logsServiceMock.getLogDetailsReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertNil(viewModel.selectedDetails)
        XCTAssertEqual(viewModel.presentableError, presentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testClearLogsAction_authenticated_whenSuccess() {
        // Given
        devicePasscodeAuthenticatorMock.authenticateUserReturnValue = true

        // When
        viewModel.clearLogsAction()
        logsServiceMock.cleaLogHistoryReceivedCompletion.first?(.success(()))

        // Then
        XCTAssertEqual(logsServiceMock.cleaLogHistoryCallsCount, 1)
        XCTAssertEqual(viewModel.renderables, [])
    }

    func testClearLogsAction_authenticated_whenFailure() {
        // Given
        devicePasscodeAuthenticatorMock.authenticateUserReturnValue = true
        let error = ServiceError(message: "Error")
        let presentableError: ErrorBottomModalViewModel = .alertError(message: error.localizedDescription)

        // When
        viewModel.clearLogsAction()
        logsServiceMock.cleaLogHistoryReceivedCompletion.first?(.failure(error))

        // Then
        XCTAssertEqual(logsServiceMock.cleaLogHistoryCallsCount, 1)
        XCTAssertEqual(viewModel.presentableError, presentableError)
        XCTAssertTrue(viewModel.isPresentingError)
    }

    func testClearLogsAction_AuthenticationFailed() {
        // Given
        devicePasscodeAuthenticatorMock.authenticateUserReturnValue = false
        // When
        viewModel.clearLogsAction()

        // Then
        XCTAssertEqual(logsServiceMock.cleaLogHistoryCallsCount, 0)
    }

    func testOnMoreMenuTap_SetsIsShowingActionSheetToTrue() {
        // When
        viewModel.onMoreMenuTap()

        // Then
        XCTAssertTrue(viewModel.isShowingActionSheet)
    }

    func testOnMoreActionSheetDismissal_WhenShouldPresentAddNoteModalIsTrue_SetsIsPresentingAddNoteModalToTrue() {
        // Given
        viewModel.shouldPresentAddNoteModal = true

        // When
        viewModel.onMoreActionSheetDismissal()

        // Then
        XCTAssertTrue(viewModel.isPresentingAddNoteModal)
        XCTAssertFalse(viewModel.shouldPresentAddNoteModal)
    }

    func testOnMoreActionSheetDismissal_WhenShouldPresentClearConfirmationModalIsTrue_SetsIsPresentingClearConfirmationModalToTrue(
    ) {
        // Given
        viewModel.shouldPresentClearConfirmationModal = true

        // When
        viewModel.onMoreActionSheetDismissal()

        // Then
        XCTAssertTrue(viewModel.isPresentingClearConfirmationModal)
        XCTAssertFalse(viewModel.shouldPresentClearConfirmationModal)
    }

    func testOnMoreActionSheetDismissal_WhenBothModalsAreFalse_NoChangeInModalPresentationState() {
        // Given
        viewModel.shouldPresentAddNoteModal = false
        viewModel.shouldPresentClearConfirmationModal = false
        let initialAddNoteModalState = viewModel.isPresentingAddNoteModal
        let initialClearConfirmationModalState = viewModel.isPresentingClearConfirmationModal

        // When
        viewModel.onMoreActionSheetDismissal()

        // Then
        XCTAssertEqual(viewModel.isPresentingAddNoteModal, initialAddNoteModalState)
        XCTAssertEqual(viewModel.isPresentingClearConfirmationModal, initialClearConfirmationModalState)
    }

    func testOnEventDetailsDismiss_whenSelectedDetails_clearsSelection() {
        // Given
        viewModel.selectedDetails = .generate()

        // When
        viewModel.onEventDetailsDismiss()

        // Then
        XCTAssertNil(viewModel.selectedDetails)
    }
}

private extension LogEntryRenderable {
    static func generate(
        title: String = "Default Title",
        displayValue: String? = "Display Value",
        additionalValue: String? = "Additional Value",
        isWarning: Bool = false,
        type: EntryType = .basic,
        dateHeader: String? = "Date Header",
        timestamp: String = "Timestamp",
        navigationDetails: UInt32 = 0
    ) -> LogEntryRenderable {
        LogEntryRenderable(
            title: title,
            displayValue: displayValue,
            additionalValue: additionalValue,
            isWarning: isWarning,
            type: type,
            dateHeader: dateHeader,
            timestamp: timestamp,
            navigationDetails: navigationDetails
        )
    }
}
