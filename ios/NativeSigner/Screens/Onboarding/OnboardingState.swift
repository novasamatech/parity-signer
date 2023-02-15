//
//  OnboardingState.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import Combine
import SwiftUI

enum OnboardingState: Equatable {
    case overview
    case terms
    case airgap
    case screenshots
}

final class OnboardingStateMachine: ObservableObject {
    @Published var currentState: OnboardingState = .overview

    @ViewBuilder
    func currentView() -> some View {
        switch currentState {
        case .overview:
            OnboardingOverviewView(viewModel: .init(onNextTap: { self.onOverviewFinishTap() }))
        case .terms:
            OnboardingAgreementsView(viewModel: .init(onNextTap: { self.onAgreementNextTap() }))
        case .airgap:
            EmptyView()
        case .screenshots:
            OnboardingScreenshotsView(viewModel: .init(onNextTap: { self.onScreenshotNextTap() }))
        }
    }

    func onOverviewFinishTap() {
        currentState = .terms
    }

    func onAgreementNextTap() {
        currentState = .screenshots
    }

    func onScreenshotNextTap() {
        // This is just temporary, on last step we need to revert to first one
        currentState = .overview
    }
}
