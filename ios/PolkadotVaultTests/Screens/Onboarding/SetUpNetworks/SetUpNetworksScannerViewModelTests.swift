import Foundation
@testable import PolkadotVault
import XCTest

final class SetUpNetworksScannerViewModelTests: XCTestCase {
    func testChainSpecButtonPresentsScanner() {
        let viewModel = SetUpNetworksStepOneView.ViewModel(onNextTap: {}, onBackTap: {})

        viewModel.onScanTap()

        XCTAssertTrue(viewModel.isShowingQRScanner)
    }

    func testMetadataButtonPresentsScanner() {
        let viewModel = SetUpNetworksStepTwoView.ViewModel(onNextTap: {}, onBackTap: {})

        viewModel.onScanTap()

        XCTAssertTrue(viewModel.isShowingQRScanner)
    }
}
