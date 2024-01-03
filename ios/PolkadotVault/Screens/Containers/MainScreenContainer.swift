//
//  MainScreenContainer.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import Combine
import SwiftUI

struct MainScreenContainer: View {
    @StateObject var viewModel: ViewModel
    @StateObject var onboarding: OnboardingStateMachine

    var body: some View {
        Group {
            switch viewModel.viewState {
            case .authenticated:
                AuthenticatedScreenContainer(viewModel: .init())
            case .deviceLocked:
                UnlockDeviceView(viewModel: .init())
            case .onboarding:
                onboarding.currentView()
            case .updateRequired:
                ApplicationUpdateRequiredView(viewModel: .init())
            case .noPincode:
                DevicePincodeRequired(viewModel: .init())
            }
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingNoAirgap
        ) {
            NoAirgapView(viewModel: viewModel.noAirgapViewModel())
        }
    }
}

extension MainScreenContainer {
    enum ViewState: Equatable, Hashable {
        case authenticated
        case deviceLocked
        case onboarding
        case updateRequired
        case noPincode
    }

    final class ViewModel: ObservableObject {
        private let authenticationStateMediator: AuthenticatedStateMediator
        private let onboardingMediator: OnboardingMediating
        private let passwordProtectionStatePublisher: PasswordProtectionStatePublisher
        private let databaseVersionMediator: DatabaseVersionMediator
        private let appLaunchMediator: AppLaunchMediating
        private let connectivityMediator: ConnectivityMediator

        private let cancelBag = CancelBag()
        @Published var viewState: ViewState = .deviceLocked
        @Published var isPresentingError: Bool = false
        @Published var isPresentingNoAirgap: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .alertError(message: "")

        init(
            authenticationStateMediator: AuthenticatedStateMediator = ServiceLocator.authenticationStateMediator,
            onboardingMediator: OnboardingMediating = ServiceLocator.onboardingMediator,
            passwordProtectionStatePublisher: PasswordProtectionStatePublisher = PasswordProtectionStatePublisher(),
            databaseVersionMediator: DatabaseVersionMediator = DatabaseVersionMediator(),
            appLaunchMediator: AppLaunchMediating = AppLaunchMediator(),
            connectivityMediator: ConnectivityMediator = ServiceLocator.connectivityMediator
        ) {
            self.authenticationStateMediator = authenticationStateMediator
            self.onboardingMediator = onboardingMediator
            self.passwordProtectionStatePublisher = passwordProtectionStatePublisher
            self.databaseVersionMediator = databaseVersionMediator
            self.appLaunchMediator = appLaunchMediator
            self.connectivityMediator = connectivityMediator
            initialiseAppRun()
        }

        func noAirgapViewModel() -> NoAirgapView.ViewModel {
            .init(mode: .noAirgap) {
                self.isPresentingNoAirgap = false
            }
        }
    }
}

private extension MainScreenContainer.ViewModel {
    func initialiseAppRun() {
        appLaunchMediator.finaliseInitialisation(connectivityMediator.isConnectivityOn) { result in
            switch result {
            case .success:
                self.checkInitialState()
            case let .failure(error):
                self.presentableError = .alertError(message: error.localizedDescription)
                self.isPresentingError = true
            }
        }
    }

    func checkInitialState() {
        databaseVersionMediator.checkDatabaseScheme { result in
            switch result {
            case .success:
                self.listenToStateChanges()
            case let .failure(error):
                switch error {
                case .invalidVersion:
                    self.viewState = .updateRequired
                case let .error(serviceError):
                    /// If DB version check was unavailable, assume user needs to update
                    /// If that's not the case (i.e. there is no newer version), app restart will fix it so should
                    /// be ok
                    self.viewState = .updateRequired
                    self.presentableError = .alertError(message: serviceError.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }

    func listenToStateChanges() {
        Publishers.CombineLatest3(
            onboardingMediator.onboardingDone,
            authenticationStateMediator.$authenticated,
            passwordProtectionStatePublisher.$isProtected
        )
        .map {
            let (onboardingDone, authenticated, isProtected) = $0
            if !isProtected {
                return .noPincode
            }
            if !onboardingDone {
                return .onboarding
            }
            return authenticated ? .authenticated : .deviceLocked
        }
        .assign(to: \.viewState, on: self)
        .store(in: cancelBag)
        connectivityMediator.$isConnectivityOn
            .sink(receiveValue: { newValue in
                guard !self.isPresentingNoAirgap, newValue else { return }
                self.isPresentingNoAirgap = newValue
            })
            .store(in: cancelBag)
    }
}
