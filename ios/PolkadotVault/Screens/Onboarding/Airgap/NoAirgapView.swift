//
//  NoAirgapView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import SwiftUI

struct NoAirgapView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        GeometryReader { geo in
            ScrollView {
                VStack(alignment: .center, spacing: 0) {
                    // Header text
                    Text(viewModel.title)
                        .font(PrimaryFont.titleL.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .multilineTextAlignment(.center)
                        .padding(.top, Spacing.extraLarge)
                        .padding(.horizontal, Spacing.large)
                    Localizable.Airgap.Label.content.text
                        .font(PrimaryFont.bodyM.font)
                        .foregroundColor(.textAndIconsTertiary)
                        .multilineTextAlignment(.center)
                        .padding(.horizontal, Spacing.medium)
                        .padding(.vertical, Spacing.medium)
                    // Airgap connectivity
                    VStack(spacing: 0) {
                        VStack(alignment: .leading, spacing: 0) {
                            cell(.aiplaneMode, isChecked: viewModel.isAirplaneModeChecked)
                                .padding(.bottom, Spacing.small)
                            Divider()
                            cell(.wifi, isChecked: viewModel.isWifiChecked)
                                .padding(.top, Spacing.small)
                        }
                        .padding(Spacing.medium)
                    }
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.medium)
                    .padding(.vertical, Spacing.extraSmall)
                    // Cables connectivity
                    VStack(spacing: 0) {
                        VStack(alignment: .leading, spacing: 0) {
                            HStack(alignment: .center, spacing: Spacing.large) {
                                Image(.airgapCables)
                                    .padding(.leading, Spacing.extraSmall)
                                    .foregroundColor(.textAndIconsTertiary)
                                Localizable.Airgap.Label.cables.text
                                    .foregroundColor(.textAndIconsTertiary)
                                    .font(PrimaryFont.bodyL.font)
                            }
                            .padding(.bottom, Spacing.medium)
                            Divider()
                            HStack(alignment: .center, spacing: Spacing.large) {
                                Group {
                                    if viewModel.isCableCheckBoxSelected {
                                        Image(.checkboxChecked)
                                            .foregroundColor(.accentPink300)
                                    } else {
                                        Image(.checkboxEmpty)
                                            .foregroundColor(.textAndIconsPrimary)
                                    }
                                }
                                .padding(.leading, Spacing.extraSmall)
                                Localizable.Airgap.Label.Cables.confirmation.text
                                    .multilineTextAlignment(.leading)
                                    .fixedSize(horizontal: false, vertical: true)
                                    .foregroundColor(.textAndIconsPrimary)
                                    .font(PrimaryFont.bodyL.font)
                            }
                            .padding(.top, Spacing.medium)
                            .contentShape(Rectangle())
                            .onTapGesture {
                                viewModel.toggleCheckbox()
                            }
                        }
                        .padding(Spacing.medium)
                    }
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.medium)
                    .padding(.vertical, Spacing.extraSmall)
                    Spacer()
                    PrimaryButton(
                        action: viewModel.onDoneTap,
                        text: viewModel.actionTitle,
                        style: .primary(isDisabled: $viewModel.isActionDisabled)
                    )
                    .padding(Spacing.large)
                }
                .frame(
                    minWidth: geo.size.width,
                    minHeight: geo.size.height
                )
            }
            .background(.backgroundSystem)
        }
    }

    @ViewBuilder
    func cell(_ component: AirgapComponent, isChecked: Bool) -> some View {
        HStack(alignment: .center, spacing: Spacing.medium) {
            Group {
                isChecked ? component.checkedIcon : component.uncheckedIcon
            }
            Text(component.title)
                .foregroundColor(isChecked ? component.checkedForegroundColor : component.uncheckedForegroundColor)
                .font(PrimaryFont.bodyL.font)
        }
    }
}

extension NoAirgapView {
    enum Mode {
        case onboarding
        case noAirgap
    }

    struct AirgapComponentStatus: Equatable, Hashable {
        let component: AirgapComponent
        let isChecked: Bool
    }

    final class ViewModel: ObservableObject {
        @Published var isCableCheckBoxSelected: Bool = false
        @Published var isActionDisabled: Bool = true
        @Published var isAirplaneModeChecked: Bool = false
        @Published var isWifiChecked: Bool = false
        private let mode: Mode
        private let airgapMediator: AirgapMediating
        private let onActionTap: () -> Void
        var title: String {
            mode == .onboarding ? Localizable.Airgap.Label.Title.setup.string : Localizable.Airgap.Label.Title.broken
                .string
        }

        var actionTitle: LocalizedStringKey {
            mode == .onboarding ? Localizable.Airgap.Action.next.key : Localizable.Airgap.Action.done.key
        }

        init(
            mode: Mode,
            airgapMediator: AirgapMediating = AirgapMediatorAssembler().assemble(),
            onActionTap: @escaping () -> Void
        ) {
            self.mode = mode
            self.airgapMediator = airgapMediator
            self.onActionTap = onActionTap
            subscribeToUpdates()
        }

        func subscribeToUpdates() {
            airgapMediator.startMonitoringAirgap { [weak self] isAirplaneModeOn, isWifiOn in
                self?.isAirplaneModeChecked = isAirplaneModeOn
                self?.isWifiChecked = !isWifiOn
                self?.updateActionState()
            }
        }

        func onDoneTap() {
            onActionTap()
        }

        func toggleCheckbox() {
            isCableCheckBoxSelected.toggle()
            updateActionState()
        }

        private func updateActionState() {
            isActionDisabled = !isCableCheckBoxSelected || !isWifiChecked || !isAirplaneModeChecked
        }
    }
}

#if DEBUG
    struct NoAirgapView_Previews: PreviewProvider {
        static var previews: some View {
            NoAirgapView(
                viewModel: .init(mode: .onboarding, onActionTap: {})
            )
            .preferredColorScheme(.dark)
        }
    }
#endif
