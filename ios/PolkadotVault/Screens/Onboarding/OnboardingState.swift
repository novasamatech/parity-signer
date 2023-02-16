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
            OnboardingAirgapView(viewModel: .init(onNextTap: { self.onAirgapNextTap() }))
        case .screenshots:
            OnboardingScreenshotsView(viewModel: .init(onNextTap: { self.resetState() }))
        }
    }

    func onOverviewFinishTap() {
        currentState = .terms
    }

    func onAgreementNextTap() {
        currentState = .airgap
    }

    func onAirgapNextTap() {
        currentState = .screenshots
    }

    func resetState() {
        currentState = .overview
    }
}
