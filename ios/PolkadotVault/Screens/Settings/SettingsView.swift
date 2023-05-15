//
//  SettingsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct SettingsView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var appState: AppState

    var body: some View {
        NavigationView {
            ZStack(alignment: .bottom) {
                VStack(spacing: 0) {
                    NavigationBarView(
                        viewModel: NavigationBarViewModel(
                            title: Localizable.Settings.Label.title.string,
                            leftButtons: [.init(type: .empty)],
                            rightButtons: [.init(type: .empty)],
                            backgroundColor: Asset.backgroundPrimary.swiftUIColor
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
                }
                .background(Asset.backgroundPrimary.swiftUIColor)
                .navigationViewStyle(StackNavigationViewStyle())
                .navigationBarHidden(true)
                VStack(spacing: 0) {
                    ConnectivityAlertOverlay(viewModel: .init())
                    TabBarView(
                        viewModel: viewModel.tabBarViewModel
                    )
                }
                NavigationLink(
                    destination: detailView(viewModel.detailScreen)
                        .navigationBarHidden(true),
                    isActive: $viewModel.isDetailsPresented
                ) { EmptyView() }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
        .onAppear {
            viewModel.use(appState: appState)
            viewModel.loadData()
        }
        .fullScreenModal(isPresented: $viewModel.isPresentingWipeConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .wipeAll,
                mainAction: viewModel.wipe(),
                isShowingBottomAlert: $viewModel.isPresentingWipeConfirmation
            )
            .clearModalBackground()
        }
        .environment(\.rootPresentationMode, $viewModel.isDetailsPresented)
    }

    @ViewBuilder
    func detailView(_ item: SettingsItem?) -> some View {
        switch item {
        case .logs:
            LogsListView(viewModel: .init())
        case .networks:
            NetworkSelectionSettings(viewModel: .init())
        case .verifier:
            VerifierCertificateView(viewModel: .init())
        case .backup:
            BackupSelectKeyView(viewModel: .init())
        case .privacyPolicy:
            PrivacyPolicyView(viewModel: .init())
        case .termsAndConditions:
            TermsOfServiceView(viewModel: .init())
        case .wipe:
            EmptyView()
        case .none:
            EmptyView()
        }
    }
}

extension SettingsView {
    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        @Published var renderable: SettingsViewRenderable = .init()
        @Published var isPresentingWipeConfirmation = false
        @Published var isDetailsPresented = false
        @Published var detailScreen: SettingsItem?
        private weak var appState: AppState!
        private let onboardingMediator: OnboardingMediator
        private let settingsService: SettingsTabService
        let tabBarViewModel: TabBarView.ViewModel

        init(
            onboardingMediator: OnboardingMediator = ServiceLocator.onboardingMediator,
            settingsService: SettingsTabService = SettingsTabService(),
            tabBarViewModel: TabBarView.ViewModel
        ) {
            self.onboardingMediator = onboardingMediator
            self.settingsService = settingsService
            self.tabBarViewModel = tabBarViewModel
            listenToUpdates()
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func loadData() {
            renderable = SettingsViewRenderable()
        }

        private func listenToUpdates() {
            $isDetailsPresented.sink { [weak self] isPresented in
                guard let self = self, !isPresented else { return }
                self.settingsService.onSettingsTabDisplay()
            }.store(in: cancelBag)
        }

        func onTapAction(_ item: SettingsItem) {
            switch item {
            case .logs:
                detailScreen = .logs
                isDetailsPresented = true
            case .wipe:
                onTapWipe()
            case .termsAndConditions:
                detailScreen = .termsAndConditions
                isDetailsPresented = true
            case .privacyPolicy:
                detailScreen = .privacyPolicy
                isDetailsPresented = true
            case .backup:
                detailScreen = .backup
                isDetailsPresented = true
            case .networks:
                detailScreen = .networks
                isDetailsPresented = true
            case .verifier:
                detailScreen = .verifier
                isDetailsPresented = true
            }
        }

        private func onTapWipe() {
            isPresentingWipeConfirmation = true
        }

        func wipe() {
            onboardingMediator.onboard()
            isPresentingWipeConfirmation = false
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
            SettingsView(viewModel: .init(tabBarViewModel: .mock))
        }
    }
#endif
