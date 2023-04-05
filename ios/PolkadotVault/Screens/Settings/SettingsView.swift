//
//  SettingsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct SettingsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

    var body: some View {
        ZStack(alignment: .bottom) {
            VStack(spacing: 0) {
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: Localizable.Settings.Label.title.string,
                        leftButtons: [.init(type: .empty)],
                        rightButtons: [.init(type: .empty)],
                        backgroundColor: Asset.backgroundSystem.swiftUIColor
                    )
                )
                ScrollView {
                    VStack(alignment: .leading, spacing: 0) {
                        ForEach(viewModel.renderable.items, id: \.id) { renderable in
                            SettingsRowView(renderable: renderable)
                                .contentShape(Rectangle())
                                .onTapGesture {
                                    viewModel.onTapAction(renderable.item)
                                }
                        }
                        Text(Localizable.Settings.Label.version(ApplicationInformation.cfBundleShortVersionString))
                            .font(PrimaryFont.captionM.font)
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .padding(.top, Spacing.medium)
                            .padding(.horizontal, Spacing.large)
                            .padding(.bottom, Spacing.extraSmall)
                    }
                }
                TabBarView(
                    selectedTab: $navigation.selectedTab
                )
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            ConnectivityAlertOverlay(viewModel: .init())
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
            viewModel.loadData()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingWipeConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .wipeAll,
                mainAction: viewModel.wipe(),
                isShowingBottomAlert: $viewModel.isPresentingWipeConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingTermsOfService) {
            TermsOfServiceView(viewModel: .init(isPresented: $viewModel.isPresentingTermsOfService))
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingPrivacyPolicy) {
            PrivacyPolicyView(viewModel: .init(isPresented: $viewModel.isPresentingPrivacyPolicy))
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingLogs) {
            LogsListView(viewModel: .init(isPresented: $viewModel.isPresentingLogs))
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingVerifier) {
            VerfierCertificateView(viewModel: .init(isPresented: $viewModel.isPresentingVerifier))
        }
        .fullScreenCover(isPresented: $viewModel.isPresentingBackup) {
            BackupSelectKeyView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingBackup
                )
            )
        }
    }
}

extension SettingsView {
    final class ViewModel: ObservableObject {
        @Published var renderable: SettingsViewRenderable = .init()
        @Published var isPresentingWipeConfirmation = false
        @Published var isPresentingTermsOfService = false
        @Published var isPresentingPrivacyPolicy = false
        @Published var isPresentingBackup = false
        @Published var isPresentingLogs = false
        @Published var isPresentingVerifier = false

        private weak var appState: AppState!
        private weak var navigation: NavigationCoordinator!
        private let onboardingMediator: OnboardingMediator

        init(
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator
        ) {
            self.onboardingMediator = onboardingMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func loadData() {
            renderable = SettingsViewRenderable()
        }

        func onTapAction(_ item: SettingsItem) {
            switch item {
            case .logs:
                onTapLogs()
            case .wipe:
                onTapWipe()
            case .termsAndConditions:
                onTermsAndConditionsTap()
            case .privacyPolicy:
                onPrivacyPolicyTap()
            case .networks:
                navigation.perform(navigation: .init(action: .manageNetworks))
            case .verifier:
                guard case let .vVerifier(value) = navigation
                    .performFake(navigation: .init(action: .viewGeneralVerifier)).screenData else { return }
                navigation.performFake(navigation: .init(action: .goBack))
                appState.userData.verifierDetails = value
                isPresentingVerifier = true
            case .backup:
                onBackupTap()
            }
        }

        private func onTapLogs() {
            isPresentingLogs = true
        }

        private func onTapWipe() {
            isPresentingWipeConfirmation = true
        }

        private func onTermsAndConditionsTap() {
            isPresentingTermsOfService = true
        }

        private func onPrivacyPolicyTap() {
            isPresentingPrivacyPolicy = true
        }

        private func onBackupTap() {
            isPresentingBackup = true
        }

        func wipe() {
            onboardingMediator.onboard()
            navigation.perform(navigation: .init(action: .start))
        }
    }
}

struct SettingsViewRenderable: Equatable {
    let items: [SettingsRowRenderable]

    init(items: [SettingsItem] = SettingsItem.allCases) {
        self.items = items
            .map { .init(item: $0, title: $0.title, isDestructive: $0.isDestructive, hasDetails: $0.hasDetails) }
    }
}

#if DEBUG
    struct SettingsView_Previews: PreviewProvider {
        static var previews: some View {
            SettingsView(viewModel: .init())
                .environmentObject(NavigationCoordinator())
        }
    }
#endif
