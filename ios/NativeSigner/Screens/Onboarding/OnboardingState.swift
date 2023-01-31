//
//  OnboardingState.swift
//  NativeSigner
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
            OnboardingOverviewView(viewModel: .init(stateMachine: self))
        case .terms:
            OnboardingAgreementsView(viewModel: .init(stateMachine: self))
        case .airgap:
            EmptyView()
        case .screenshots:
            EmptyView()
        }
    }

    func onOverviewSkipTap() {
        currentState = .terms
    }

    func onAgreementNextTap() {
        currentState = .airgap
    }
}
