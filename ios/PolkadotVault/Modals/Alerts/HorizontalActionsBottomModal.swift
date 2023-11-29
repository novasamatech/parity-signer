//
//  HorizontalActionsBottomModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/09/2022.
//

import SwiftUI

struct HorizontalActionsBottomModalViewModel {
    let title: String
    let content: String?
    let dismissActionLabel: LocalizedStringKey
    let mainActionLabel: LocalizedStringKey
    var mainActionStyle: ActionButtonStyle = .primaryDestructive()
    var alignment: HorizontalAlignment = .center

    static let cancelAddingDerivedKeys = HorizontalActionsBottomModalViewModel(
        title: Localizable.AddDerivedKeys.Modal.Label.title.string,
        content: Localizable.AddDerivedKeys.Modal.Label.content.string,
        dismissActionLabel: Localizable.AddDerivedKeys.Modal.Action.cancel.key,
        mainActionLabel: Localizable.AddDerivedKeys.Modal.Action.main.key,
        mainActionStyle: .primary()
    )

    static let forgetKeySet = HorizontalActionsBottomModalViewModel(
        title: Localizable.KeySetsModal.Confirmation.Label.title.string,
        content: Localizable.KeySetsModal.Confirmation.Label.content.string,
        dismissActionLabel: Localizable.KeySetsModal.Confirmation.Action.cancel.key,
        mainActionLabel: Localizable.KeySetsModal.Confirmation.Action.remove.key
    )

    static let forgetSingleKey = HorizontalActionsBottomModalViewModel(
        title: Localizable.PublicKeyDetailsModal.Confirmation.Label.title.string,
        content: Localizable.PublicKeyDetailsModal.Confirmation.Label.content.string,
        dismissActionLabel: Localizable.PublicKeyDetailsModal.Confirmation.Action.cancel.key,
        mainActionLabel: Localizable.PublicKeyDetailsModal.Confirmation.Action.remove.key
    )

    static let clearLog = HorizontalActionsBottomModalViewModel(
        title: Localizable.LogsList.ClearConfirmation.Label.title.string,
        content: nil,
        dismissActionLabel: Localizable.LogsList.ClearConfirmation.Action.cancel.key,
        mainActionLabel: Localizable.LogsList.ClearConfirmation.Action.clear.key
    )

    static let wipeAll = HorizontalActionsBottomModalViewModel(
        title: Localizable.Settings.Modal.WipeAll.Label.title.string,
        content: Localizable.Settings.Modal.WipeAll.Label.content.string,
        dismissActionLabel: Localizable.Settings.Modal.WipeAll.Action.cancel.key,
        mainActionLabel: Localizable.Settings.Modal.WipeAll.Action.wipe.key
    )

    static let removeMetadata = HorizontalActionsBottomModalViewModel(
        title: Localizable.Settings.NetworkDetails.DeleteMetadata.Label.title.string,
        content: Localizable.Settings.NetworkDetails.DeleteMetadata.Label.content.string,
        dismissActionLabel: Localizable.Settings.NetworkDetails.DeleteMetadata.Action.cancel.key,
        mainActionLabel: Localizable.Settings.NetworkDetails.DeleteMetadata.Action.remove.key
    )

    static let removeNetwork = HorizontalActionsBottomModalViewModel(
        title: Localizable.Settings.NetworkDetails.More.Delete.Label.title.string,
        content: Localizable.Settings.NetworkDetails.More.Delete.Label.content.string,
        dismissActionLabel: Localizable.Settings.NetworkDetails.More.Delete.Action.cancel.key,
        mainActionLabel: Localizable.Settings.NetworkDetails.More.Delete.Action.remove.key
    )

    static let createEmptyKeySet = HorizontalActionsBottomModalViewModel(
        title: Localizable.CreateKeysForNetwork.Modal.title.string,
        content: Localizable.CreateKeysForNetwork.Modal.content.string,
        dismissActionLabel: Localizable.CreateKeysForNetwork.Modal.cancel.key,
        mainActionLabel: Localizable.CreateKeysForNetwork.Modal.done.key,
        mainActionStyle: .primary(),
        alignment: .leading
    )
}

struct HorizontalActionsBottomModal: View {
    private var viewModel: HorizontalActionsBottomModalViewModel
    private let mainAction: () -> Void
    private let dismissAction: () -> Void
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBottomAlert: Bool

    init(
        viewModel: HorizontalActionsBottomModalViewModel,
        mainAction: @escaping @autoclosure () -> Void,
        dismissAction: @escaping @autoclosure () -> Void = {}(),
        isShowingBottomAlert: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        self.viewModel = viewModel
        self.mainAction = mainAction
        self.dismissAction = dismissAction
        _isShowingBottomAlert = isShowingBottomAlert
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal(dismissAction()) },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: viewModel.alignment, spacing: Spacing.medium) {
                    Text(viewModel.title)
                        .font(PrimaryFont.titleL.font)
                    if let content = viewModel.content {
                        Text(content)
                            .font(PrimaryFont.bodyL.font)
                            .lineSpacing(Spacing.extraExtraSmall)
                            .multilineTextAlignment(viewModel.alignment == .center ? .center : .leading)
                            .foregroundColor(.textAndIconsSecondary)
                    }
                    HStack {
                        ActionButton(
                            action: { animateDismissal(dismissAction()) },
                            text: viewModel.dismissActionLabel,
                            style: .secondary()
                        )
                        ActionButton(
                            action: { animateDismissal(mainAction()) },
                            text: viewModel.mainActionLabel,
                            style: viewModel.mainActionStyle
                        )
                    }
                    .padding(.top, Spacing.medium)
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall + Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping @autoclosure () -> Void = {}()) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingBottomAlert = false
                completion()
            }()
        )
    }
}

#if DEBUG
    struct HorizontalActionsBottomModal_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                HorizontalActionsBottomModal(
                    viewModel: .forgetKeySet,
                    mainAction: {}(),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
                HorizontalActionsBottomModal(
                    viewModel: .forgetSingleKey,
                    mainAction: {}(),
                    isShowingBottomAlert: Binding<Bool>.constant(true)
                )
            }
            .previewLayout(.sizeThatFits)
        }
    }
#endif
